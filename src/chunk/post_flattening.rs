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
    // block::Block elements, using the save format for Minecraft 1.13 and higher.
    // It also needs a pre-parsed hasmap of block entities, and some meta data
    // such as save format version (data version) and where the chunk is located
    // in the world.
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
    }

    /// Generates section NBT tags for the blocks in the chunk, and returns them
    /// in an NBT list value ready for inclusion in the post flattening chunk format.
    pub(crate) fn post_flattening_sections(&self) -> nbt::Value {
        let mut sections = Vec::new();

        for y in 0..=15 {
            sections.push(self.post_flattening_section(y));
        }

        nbt::Value::List(sections)
    }

    // TODO
    fn post_flattening_section(&self, section_y: i8) -> nbt::Value {

        let mut block_states: Vec<usize> = Vec::new();
        let mut palette: HashMap<PaletteItem, usize> = HashMap::new();
        let mut palette_index_next = 0;

        for x in 0..16 {
            for z in 0..16 {
                for y in (section_y as i64 * 16)..(16 + section_y as i64 * 16) {
                    //let index = Self::local_index(section_y as i64, (x, y, z).into());

                    if let Some(block) = self.blocks.block_at((x as usize, y as usize, z as usize))
                    {
                        let palette_item = PaletteItem::from_block(block);
                        let palette_index = palette.entry(palette_item).or_insert_with(|| {
                            let index = palette_index_next;
                            palette_index_next += 1;
                            index
                        });
                        block_states.push(*palette_index);
                    }
                }
            }
        }

        // TODO restructure block_states according to the number of bits needed for the palette
        // TODO convert the palette to its final form

        // Generate the section
        let mut section = nbt::Map::new();

        section.insert("Y".into(), nbt::Value::Byte(section_y));
//        section.insert("BlockStates".into(), nbt::Value::LongArray(block_states));
//        section.insert("Palette".into(), nbt::Value::List(palette));
//        section.insert("BlockLight".into(), nbt::Value::ByteArray(block_light));
//        section.insert("SkyLight".into(), nbt::Value::ByteArray(sky_light));

        return nbt::Value::Compound(section);
    }
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

fn bits_per_value(palette_length: usize) -> usize {
    std::cmp::max(4, (((palette_length - 1) as f64).log(2f64)).floor() as usize + 1)
}

// TODO write tests for bits_per_value
