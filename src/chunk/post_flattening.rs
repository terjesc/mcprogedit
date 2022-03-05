use std::collections::HashMap;

use crate::block::*;
use crate::block_cuboid::BlockCuboid;
use crate::block_entity::{BlockEntity, ChestTags, FurnaceTags};
use crate::chunk::Chunk;
use crate::chunk::palette;
use crate::chunk::palette::{PaletteItem, ProtoBlock};
use crate::colour::Colour;
use crate::coordinates::{BlockCoord, ChunkCoord};
use crate::inventory::Inventory;
use crate::mc_version::McVersion;
use crate::mc_version;
use crate::nbt_lookup::*;
use crate::utils;

impl Chunk {
    // This function reads a "Section" nbt entry, converting it into an array of
    // block::Block elements, using the save format of Minecraft 1.12.2.
    // It also needs a pre-parsed hasmap of block entities, including internal
    // "pseudo block entities" for two-part block structures such as doors and
    // large flowers. Those structures have some metadata in the top block, and
    // some metadata in the bottom block, while the internal mcprogedit format
    // keeps all data in both blocks.
    pub(crate) fn post_flattening_fill_block_cuboid_from_section(
        data_version: McVersion,
        section: &nbt::Value,
        block_entities: &HashMap<BlockCoord, BlockEntity>,
        chunk_position: &ChunkCoord,
        block_cuboid: &mut BlockCuboid,
    ) {

        let xz_offset: BlockCoord = chunk_position.into();
        let section_y_index = nbt_value_lookup_byte(section, "Y").unwrap() as i64;

        // If there's no palette, then there's no blocks in this section
        let palette = match palette::from_section(&section) {
            Some(palette) => palette,
            None => return,
        };

        // If the palette is empty, then there's no blocks in this section
        if palette.is_empty() {
            return;
        }

        // If the palette has one element, then the whole section is filled with that block
        if palette.len() == 1 {
            // Fill the section area of the block_cuboid with the palette block
            let block = match &palette[0] {
                PaletteItem::Block(block) => block,
                PaletteItem::ProtoBlock(_proto_block) => unimplemented!(), // TODO handle block entity blocks
            };

            for coordinate_index in 0 .. 16 * 16 * 16 {
                let coordinates = Self::coordinates(section_y_index, (0, 0, 0).into(), coordinate_index);
                let coordinates = (
                    coordinates.0 as usize,
                    coordinates.1 as usize,
                    coordinates.2 as usize,
                );
                block_cuboid.insert(coordinates, block.clone());
            }

            return;
        }

        // Exctract the block state array
        let bits_per_value = bits_per_value(palette.len());
        let block_states = nbt_value_lookup_long_array(&section, "BlockStates").unwrap();
        let block_states = if data_version >= mc_version::BLOCK_STATES_PADDED {
            utils::paddedly_unpacked::<u64>(&utils::vec_i64_into_vec_u64(block_states), bits_per_value)
        } else {
            utils::tightly_unpacked::<u64>(&utils::vec_i64_into_vec_u64(block_states), bits_per_value)
        };

        // Insert blocks
        for (coordinate_index, palette_index) in block_states.iter().enumerate() {
            let block = match &palette[*palette_index as usize] {
                PaletteItem::Block(block) => block.clone(),
                PaletteItem::ProtoBlock(proto_block) => {
                    let coordinates = Self::coordinates(section_y_index, xz_offset, coordinate_index);
                    block_from_proto_and_entity(
                        proto_block,
                        block_entities.get(&coordinates).unwrap(),
                    )
                }
            };

            let coordinates = Self::coordinates(section_y_index, (0, 0, 0).into(), coordinate_index);
            let coordinates = (
                coordinates.0 as usize,
                coordinates.1 as usize,
                coordinates.2 as usize,
            );

            block_cuboid.insert(coordinates, block);
        }

        fn block_from_proto_and_entity(proto_block: &ProtoBlock, block_entity: &BlockEntity) -> Block {
            match proto_block {
                ProtoBlock::Banner { colour, placement } => {
                    let (custom_name, patterns) =
                        if let BlockEntity::Banner { custom_name, patterns, .. } = block_entity {
                            (custom_name.clone(), patterns.clone())
                        } else {
                            (None, Vec::new())
                        };

                    Block::Banner(Box::new(Banner {
                            colour: *colour,
                            custom_name,
                            placement: *placement,
                            patterns,
                    }))
                }

                ProtoBlock::Beacon => {
                    let (lock, levels, primary, secondary) =
                        if let BlockEntity::Beacon { lock, levels, primary, secondary, .. } = block_entity {
                            (lock.clone(), *levels, primary.clone(), secondary.clone())
                        } else {
                            (None, 0, None, None)
                        };

                    Block::Beacon(Box::new(Beacon {
                        lock,
                        levels,
                        primary,
                        secondary,
                    }))
                }

                ProtoBlock::BrewingStand => {
                    let (custom_name, lock, items, brew_time, fuel) =
                        if let BlockEntity::BrewingStand { custom_name, lock, items, brew_time, fuel, .. } = block_entity {
                            (custom_name.clone(), lock.clone(), items.clone(), *brew_time, *fuel)
                        } else {
                            (None, None, Inventory::new(), 0, 0)
                        };

                    Block::BrewingStand(Box::new(BrewingStand {
                        custom_name,
                        lock,
                        items,
                        brew_time,
                        fuel,
                    }))
                }

                ProtoBlock::Chest { facing, variant, waterlogged } => {
                    let (custom_name, lock, items) =
                        if let BlockEntity::Chest { tags: ChestTags { custom_name, lock, items, .. } } = block_entity {
                            (custom_name.clone(), lock.clone(), items.clone())
                        } else {
                            (None, None, Inventory::new())
                        };

                    Block::Chest(Box::new(Chest {
                        facing: *facing,
                        variant: variant.clone(),
                        waterlogged: *waterlogged,
                        custom_name,
                        lock,
                        items,
                    }))
                }

                ProtoBlock::Dispenser { facing } => {
                    let (custom_name, lock, items) =
                        if let BlockEntity::Dispenser { tags: ChestTags { custom_name, lock, items, .. } } = block_entity {
                            (custom_name.clone(), lock.clone(), items.clone())
                        } else {
                            (None, None, Inventory::new())
                        };

                    Block::Dispenser(Box::new(Dispenser {
                        facing: *facing,
                        custom_name,
                        lock,
                        items,
                    }))
                }

                ProtoBlock::Dropper { facing } => {
                    let (custom_name, lock, items) =
                        if let BlockEntity::Dropper { tags: ChestTags { custom_name, lock, items, .. } } = block_entity {
                            (custom_name.clone(), lock.clone(), items.clone())
                        } else {
                            (None, None, Inventory::new())
                        };

                    Block::Dropper(Box::new(Dropper {
                        facing: *facing,
                        custom_name,
                        lock,
                        items,
                    }))
                }

                ProtoBlock::EnchantingTable => {
                    let custom_name =
                        if let BlockEntity::EnchantingTable { custom_name, .. } = block_entity {
                            custom_name.clone()
                        } else {
                            None
                        };

                    Block::EnchantingTable { custom_name: Box::new(custom_name) }
                }

                ProtoBlock::Furnace { facing, lit } => {
                    let (custom_name, lock, items, burn_time, cook_time, cook_time_total) =
                        if let BlockEntity::Furnace {
                            tags: FurnaceTags { custom_name, lock, items, burn_time, cook_time, cook_time_total, .. }
                        } = block_entity {
                            (custom_name.clone(), lock.clone(), items.clone(), *burn_time, *cook_time, *cook_time_total)
                        } else {
                            (None, None, Inventory::new(), 0, 0, 0)
                        };

                    Block::Furnace(Box::new(Furnace {
                        facing: *facing,
                        lit: *lit,
                        custom_name,
                        lock,
                        items,
                        burn_time,
                        cook_time,
                        cook_time_total,
                    }))
                }

                ProtoBlock::Hopper { facing } => {
                    let (custom_name, lock, items) =
                        if let BlockEntity::Hopper { tags: ChestTags { custom_name, lock, items, .. } } = block_entity {
                            (custom_name.clone(), lock.clone(), items.clone())
                        } else {
                            (None, None, Inventory::new())
                        };

                    Block::Hopper(Box::new(Hopper {
                        facing: *facing,
                        custom_name,
                        lock,
                        items,
                    }))
                }

                ProtoBlock::Jukebox => {
                    let record = if let BlockEntity::Jukebox { record, .. } = block_entity {
                        record.clone()
                    } else {
                        None
                    };

                    Block::Jukebox(Box::new(Jukebox { record }))
                }

                ProtoBlock::ShulkerBox { colour, facing } => {
                    let (custom_name, lock, items) =
                        if let BlockEntity::ShulkerBox { tags: ChestTags { custom_name, lock, items, .. } } = block_entity {
                            (custom_name.clone(), lock.clone(), items.clone())
                        } else {
                            (None, None, Inventory::new())
                        };

                    Block::ShulkerBox(Box::new(ShulkerBox {
                        colour: colour.clone(),
                        facing: *facing,
                        custom_name,
                        lock,
                        items,
                    }))
                }

                ProtoBlock::Sign { material, placement, waterlogged } => {
                    let (colour, text) =
                        if let BlockEntity::Sign { colour, text, .. } = block_entity {
                            (*colour, text.clone())
                        } else {
                            (Colour::Black, vec![String::new(); 4])
                        };

                    Block::Sign(Box::new(Sign {
                        material: *material,
                        placement: *placement,
                        waterlogged: *waterlogged,
                        colour,
                        text1: text[0].clone(),
                        text2: text[1].clone(),
                        text3: text[2].clone(),
                        text4: text[3].clone(),
                    }))
                }

                ProtoBlock::TrappedChest { facing, variant, waterlogged } => {
                    let (custom_name, lock, items) =
                        if let BlockEntity::TrappedChest { tags: ChestTags { custom_name, lock, items, .. } } = block_entity {
                            (custom_name.clone(), lock.clone(), items.clone())
                        } else {
                            (None, None, Inventory::new())
                        };

                    Block::TrappedChest(Box::new(Chest {
                        facing: *facing,
                        variant: variant.clone(),
                        waterlogged: *waterlogged,
                        custom_name,
                        lock,
                        items,
                    }))
                }
            }
        }
        /*
        //let mut block_cuboid = BlockCuboid::new((16, 16, 16));
        blocks
            .iter()
            .enumerate()
            .map(|(index, block)| (index, ((add[index] as u16) << 8) + ((*block as u16) & 0xFF)))
            .map(|(index, block)| {
                (
                    index,
                    match block {

                        // TODO Blocks for which proto block and block entity must be combined
                        // Dispenser
                        23 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates).unwrap();

                            match block_entity {
                                BlockEntity::Dispenser { tags } => {
                                    Block::Dispenser(Box::new(Dispenser {
                                        facing: facing6_dunswe(data[index]),
                                        custom_name: tags.custom_name.clone(),
                                        lock: tags.lock.clone(),
                                        items: tags.items.clone(),
                                    }))
                                }
                                _ => panic!("Wrong block entity variant for dispenser"),
                            }
                        }
                        // Chest
                        54 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates).unwrap();

                            match block_entity {
                                BlockEntity::Chest { tags } => Block::Chest(Box::new(Chest {
                                    facing: facing4_xxnswe(data[index]),
                                    variant: None,
                                    waterlogged: false,
                                    custom_name: tags.custom_name.clone(),
                                    lock: tags.lock.clone(),
                                    items: tags.items.clone(),
                                })),
                                _ => panic!("Wrong block entity variant for chest"),
                            }
                        }
                        // Furnace
                        61 | 62 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates);

                            match block_entity {
                                Some(BlockEntity::Furnace { tags }) => {
                                    Block::Furnace(Box::new(Furnace {
                                        facing: facing4_xxnswe(data[index]),
                                        lit: block == 62,
                                        custom_name: tags.custom_name.clone(),
                                        lock: tags.lock.clone(),
                                        items: tags.items.clone(),
                                        burn_time: tags.burn_time,
                                        cook_time: tags.cook_time,
                                        cook_time_total: tags.cook_time_total,
                                    }))
                                }
                                _ => Block::Sponge,
                                //_ => panic!("Wrong block entity variant for chest"),
                            }
                        }
                        // Both block variants of signs
                        63 | 68 => {
                            let placement = match block {
                                63 => WallOrRotatedOnFloor::Floor((data[index] & 0xF).into()),
                                68 => WallOrRotatedOnFloor::Wall(facing4_xxnswe(data[index])),
                                _ => unreachable!(),
                            };
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates).unwrap();

                            match block_entity {
                                BlockEntity::Sign { colour, text, .. } => {
                                    Block::Sign(Box::new(Sign {
                                        material: WoodMaterial::Oak,
                                        placement,
                                        waterlogged: false,
                                        colour: *colour,
                                        // TODO something reasonable instead of JSON text
                                        text1: text.get(0).unwrap_or(&String::new()).to_string(),
                                        text2: text.get(1).unwrap_or(&String::new()).to_string(),
                                        text3: text.get(2).unwrap_or(&String::new()).to_string(),
                                        text4: text.get(3).unwrap_or(&String::new()).to_string(),
                                    }))
                                }
                                _ => panic!("Wrong block entity variant for sign"),
                            }
                        }
                        // Jukebox
                        84 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates).unwrap();

                            match block_entity {
                                BlockEntity::Jukebox { record, .. } => {
                                    Block::Jukebox(Box::new(Jukebox {
                                        record: record.clone(),
                                    }))
                                }
                                _ => panic!("Wrong block entity variant for jukebox"),
                            }
                        }
                        // Enchanting Table
                        116 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates).unwrap();

                            match block_entity {
                                BlockEntity::EnchantingTable { custom_name, .. } => {
                                    Block::EnchantingTable {
                                        custom_name: Box::new(custom_name.clone()),
                                    }
                                }
                                _ => panic!("Wrong block entity variant for enchanting table"),
                            }
                        }
                        // Brewing Stand
                        117 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates);

                            match block_entity {
                                Some(BlockEntity::BrewingStand {
                                    custom_name,
                                    lock,
                                    items,
                                    brew_time,
                                    fuel,
                                    ..
                                }) => Block::BrewingStand(Box::new(BrewingStand {
                                    custom_name: custom_name.clone(),
                                    lock: lock.clone(),
                                    items: items.clone(),
                                    brew_time: *brew_time,
                                    fuel: *fuel,
                                })),
                                _ => Block::Sponge,
                                //_ => panic!("Wrong block entity variant for brewing stand"),
                            }
                        }
                        // Beacon
                        138 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates).unwrap();

                            match block_entity {
                                BlockEntity::Beacon {
                                    lock,
                                    levels,
                                    primary,
                                    secondary,
                                    ..
                                } => Block::Beacon(Box::new(Beacon {
                                    lock: lock.clone(),
                                    levels: *levels,
                                    primary: *primary,
                                    secondary: *secondary,
                                })),
                                _ => panic!("Wrong block entity variant for beacon"),
                            }
                        }
                        // Trapped Chest
                        146 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates).unwrap();

                            match block_entity {
                                BlockEntity::Chest { tags } => {
                                    Block::TrappedChest(Box::new(Chest {
                                        facing: facing4_xxnswe(data[index]),
                                        variant: None,
                                        waterlogged: false,
                                        custom_name: tags.custom_name.clone(),
                                        lock: tags.lock.clone(),
                                        items: tags.items.clone(),
                                    }))
                                }
                                _ => panic!("Wrong block entity variant for chest"),
                            }
                        }
                        // Hopper
                        154 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates).unwrap();

                            match block_entity {
                                BlockEntity::Hopper { tags } => Block::Hopper(Box::new(Hopper {
                                    facing: facing5_dxnswe(data[index]),
                                    waterlogged: false,
                                    custom_name: tags.custom_name.clone(),
                                    lock: tags.lock.clone(),
                                    items: tags.items.clone(),
                                })),
                                _ => panic!("Wrong block entity variant for hopper"),
                            }
                        }
                        // Dropper
                        158 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates).unwrap();

                            match block_entity {
                                BlockEntity::Dropper { tags } => {
                                    Block::Dropper(Box::new(Dropper {
                                        facing: facing6_dunswe(data[index]),
                                        custom_name: tags.custom_name.clone(),
                                        lock: tags.lock.clone(),
                                        items: tags.items.clone(),
                                    }))
                                }
                                _ => panic!("Wrong block entity variant for dispenser"),
                            }
                        }
                        // Banners
                        176 | 177 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates).unwrap();

                            match block_entity {
                                BlockEntity::Banner {
                                    colour,
                                    custom_name,
                                    patterns,
                                    ..
                                } => Block::Banner(Box::new(Banner {
                                    colour: *colour,
                                    custom_name: custom_name.clone(),
                                    placement: if block == 176 {
                                        WallOrRotatedOnFloor::Floor((data[index] & 0xF).into())
                                    } else {
                                        WallOrRotatedOnFloor::Wall(facing4_xxnswe(data[index]))
                                    },
                                    patterns: patterns.clone(),
                                })),
                                _ => panic!("Wrong block entity variant for standing sign"),
                            }
                        }
                        // All shulker box colours
                        block_id @ 219..=234 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates).unwrap();

                            match block_entity {
                                BlockEntity::ShulkerBox { tags } => {
                                    Block::ShulkerBox(Box::new(ShulkerBox {
                                        colour: Some(((block_id - 219) as i32).into()),
                                        facing: facing6_dunswe(data[index]),
                                        custom_name: tags.custom_name.clone(),
                                        lock: tags.lock.clone(),
                                        items: tags.items.clone(),
                                    }))
                                }
                                _ => panic!("Wrong block entity variant for shulker box"),
                            }
                        }

                        n => Block::Unknown(Some(n)),
                    },
                )
            })
            .for_each(|(index, block)| {
                let coordinates = Self::coordinates(section_y_index, (0, 0, 0).into(), index);
                let coordinates = (
                    coordinates.0 as usize,
                    coordinates.1 as usize,
                    coordinates.2 as usize,
                );
                block_cuboid.insert(coordinates, block);
            });
        //return block_cuboid;
        */
    }
}
        
fn bits_per_value(palette_length: usize) -> usize {
    std::cmp::max(4, (((palette_length - 1) as f64).log(2f64)).floor() as usize + 1)
}

// TODO write tests for bits_per_value
