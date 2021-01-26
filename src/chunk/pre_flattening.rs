use std::collections::HashMap;

use crate::block::*;
use crate::block_cuboid::BlockCuboid;
use crate::block_entity::BlockEntity;
use crate::bounded_ints::*;
use crate::chunk::Chunk;
use crate::colour::Colour;
use crate::coordinates::{BlockColumnCoord, BlockCoord, ChunkCoord};
use crate::material::*;
use crate::nbt_lookup::*;
use crate::positioning::*;
use crate::utils;

impl Chunk {
    /// Calculates the global block coordinates of the block at index `index`
    /// of the "Blocks" and similar NBT tags, within section `section_y_index`
    /// of the chunk whose local (0, 0, 0) coordinates are at global block
    /// coordinates `chunk_offset`.
    fn coordinates(section_y_index: i64, chunk_offset: BlockCoord, index: usize) -> BlockCoord {
        // index = (y * X_LENGTH * Z_LENGTH) + (z * X_LENGTH) + x
        const X_LENGTH: i64 = 16;
        const Y_HEIGHT: i64 = 16;
        const Z_LENGTH: i64 = 16;
        let y_offset = section_y_index * Y_HEIGHT;
        let y = y_offset + (index as i64) / (X_LENGTH * Z_LENGTH);
        let z = ((index as i64) % (X_LENGTH * Z_LENGTH)) / X_LENGTH;
        let x = (index as i64) % X_LENGTH;
        //println!("Looking for block entity at ({}, {}, {})", x, y, z);
        let local_coordinates: BlockCoord = (x, y, z).into();
        local_coordinates + chunk_offset
    }

    /// Calculates the index into the "Blocks" and similar NBT tags, for a block
    /// within section `section_y_index`, located at chunk local coordinates
    /// `local_block_coords`.
    fn local_index(section_y_index: i64, local_block_coords: BlockCoord) -> usize {
        const X_LENGTH: i64 = 16;
        const Y_HEIGHT: i64 = 16;
        const Z_LENGTH: i64 = 16;
        let y_offset = section_y_index * Y_HEIGHT;
        let (x, y, z) = (local_block_coords.0, local_block_coords.1, local_block_coords.2);
        ((y - y_offset) * X_LENGTH * Z_LENGTH + z * X_LENGTH + x) as usize
    }

    /// Calculates the index into the "Blocks" and similar NBT tags, for a block
    /// within section `section_y_index` of the chunk whose local (0, 0, 0)
    /// coordinates are at global block coordinates `chunk_offset`.
    fn global_index(
        section_y_index: i64,
        chunk_offset: BlockCoord,
        global_block_coordinates: BlockCoord,
    ) -> usize {
        let local_coords = global_block_coordinates - chunk_offset;
        Self::local_index(section_y_index, local_coords)
    }

    /// Generates tile entities for all blocks in the chunk, and returns them
    /// in an NBT list value ready for inclusion in the pre flattening chunk format.
    pub(crate) fn pre_flattening_tile_entities(&self) -> nbt::Value {
        let (x_dim, y_dim, z_dim) = self.blocks.dim();

        let chunk_offset_blocks: BlockColumnCoord = self.global_pos.into();

        let mut tile_entities = Vec::new();

        for x in 0..x_dim {
            let block_x = chunk_offset_blocks.0 as i32 + x as i32;
            for y in 0..y_dim {
                for z in 0..z_dim {
                    let block_z = chunk_offset_blocks.1 as i32 + z as i32;
                    match self.blocks.block_at((x, y, z)) {
                        None => (),
                        Some(Block::Banner(banner)) => {
                            if let Some(value) = banner
                                .to_block_entity((block_x, y as i32, block_z))
                                .to_nbt_value()
                            {
                                tile_entities.push(value);
                            }
                        }
                        // TODO add handling of other blocks with entities
                        //Some(block) => println!("Block found: {:?}", block),
                        _ => (),
                    }
                }
            }
        }
        nbt::Value::List(tile_entities)
    }

    /// Generates section NBT tags for the blocks in the chunk, and returns them
    /// in an NBT list value ready for inclusion in the pre flattening chunk format.
    pub(crate) fn pre_flattening_sections(&self) -> nbt::Value {
        let mut sections = Vec::new();

        // TODO We only need sections 0..=N, where section N is the highest
        // section with a non-air block in it. For now we write all blocks.
        for y in 0..=15 {
            sections.push(self.pre_flattening_section(y));
        }

        nbt::Value::List(sections)
    }

    /// Generates an individual section NBT tag from the chunk.
    fn pre_flattening_section(&self, section_y: i8) -> nbt::Value {
        let mut blocks = vec![0u8; 4096];
        // NB "Add" is never used, unless there are blocks from mods involved.
        //let mut add = vec![0u8; 2048];
        let mut data = vec![0u8; 2048];

        // Handle block IDs and block data
        for x in 0..16 {
            for z in 0..16 {
                for y in (section_y as i64 * 16)..(16 + section_y as i64 * 16) {
                    let index = Self::local_index(section_y as i64, (x, y, z).into());

                    if let Some(block) = self.blocks
                        .block_at((x as usize, y as usize, z as usize))
                    {
                        let (block_id, data_value) = match block {
                            Block::Air => (0, 0),
                            Block::Stone => (1, 0),
                            Block::Granite => (1, 1),
                            Block::PolishedGranite => (1, 2),
                            Block::Diorite => (1, 3),
                            Block::PolishedDiorite => (1, 4),
                            Block::Andesite => (1, 5),
                            Block::PolishedAndesite => (1, 6),
                            Block::GrassBlock => (2, 0),
                            Block::Dirt => (3, 0),
                            Block::CoarseDirt => (3, 1),
                            Block::Podzol => (3, 2),
                            Block::Cobblestone => (4, 0),
                            Block::Planks { material } => {
                                match material {
                                    WoodMaterial::Oak => (5, 0),
                                    WoodMaterial::Spruce => (5, 1),
                                    WoodMaterial::Birch => (5, 2),
                                    WoodMaterial::Jungle => (5, 3),
                                    WoodMaterial::Acacia => (5, 4),
                                    WoodMaterial::DarkOak => (5, 5),
                                    WoodMaterial::Crimson => (5, 0), // Fallback to oak
                                    WoodMaterial::Warped => (5, 0), // Fallback to oak
                                }
                            },
                            Block::Sapling { growth_stage, material } => {
                                let data = (growth_stage.get() as u8) << 3;
                                let data = data | match material {
                                    SaplingMaterial::Oak => 0,
                                    SaplingMaterial::Spruce => 1,
                                    SaplingMaterial::Birch => 2,
                                    SaplingMaterial::Jungle => 3,
                                    SaplingMaterial::Acacia => 4,
                                    SaplingMaterial::DarkOak => 5,
                                    SaplingMaterial::Bamboo => 0, // Fallback to oak
                                };
                                (6, data)
                            },
                            Block::Bedrock => (7, 0),
                            // TODO add more block types
                            /*
                            Block::Door(door) => {
                                (match door.material {
                                },
                                match door.half {
                                },
                                )
                            }
                            */
                            _ => (0, 0),
                        };

                        blocks[index] = block_id;
                        utils::set_nibble(&mut data, data_value, index);
                    }
                }
            }
        }

        // TODO Somehow fill block light and sky light with reasonable values...
        let block_light = vec![0u8; 2048];
        let sky_light = vec![0u8; 2048];

        // A section is a TAG_Compound containing:
        // - "Y" TAG_Byte index 0 to 15 (bottom to top)
        // - "Blocks" TAG_Byte_Array 4096 bytes, one per block
        // - "Add" TAG_Byte_Array 2048 bytes, half byte per block
        // - "Data" TAG_Byte_Array 2048 bytes, half byte per block
        // - "BlockLight" TAG_Byte_Array 2048 bytes, half byte per block
        // - "SkyLight" TAG_Byte_Array 2048 bytes, half byte per block

        // The NBT library for some reason needs Vec<i8>,
        // even though these fields are series of bytes,
        // and u8 is much easier to work with for that purpose...
        let blocks = utils::vec_u8_into_vec_i8(blocks);
        let data = utils::vec_u8_into_vec_i8(data);
        let block_light = utils::vec_u8_into_vec_i8(block_light);
        let sky_light = utils::vec_u8_into_vec_i8(sky_light);

        let mut section = nbt::Map::new();
        section.insert("Y".into(), nbt::Value::Byte(section_y));
        section.insert("Blocks".into(), nbt::Value::ByteArray(blocks));
        section.insert("Data".into(), nbt::Value::ByteArray(data));
        section.insert("BlockLight".into(), nbt::Value::ByteArray(block_light));
        section.insert("SkyLight".into(), nbt::Value::ByteArray(sky_light));

        nbt::Value::Compound(section)
    }

    /// Generates custom block entities later used by chunk section parsing.
    ///
    /// These "pseudo" block entities are not part of the game save format, and
    /// they are strictly internal to mcprogedit. Their sole purpose is to solve
    /// the following parsing challenge:
    ///
    /// Some blocks, e.g. doors, depend on neighbouring block data for their state.
    /// This is problematic when the neighbouring block is in a different section.
    ///
    /// The solution is to run a preparatory pass over all sections, and store
    /// data needed by neighbouring blocks in special "pseudo" block entities,
    /// that are inserted into the collection of "real" block entities. Since the
    /// collection of block entities is global to the whole chunk, this gives the
    /// section parser access to the needed data regardless of what section it
    /// originated from.
    ///
    /// Accessing neighbour block data is therefore done the same way as accessing
    /// block entity data.
    pub(crate) fn pre_flattening_pseudo_block_entities(
        section: &nbt::Value,
        chunk_position: &ChunkCoord,
    ) -> HashMap<BlockCoord, BlockEntity> {
        let xz_offset: BlockCoord = chunk_position.into();
        let section_y_index = nbt_value_lookup_byte(&section, "Y").unwrap() as i64;
        let blocks = nbt_value_lookup_byte_array(&section, "Blocks").unwrap();
        let add = packed_nibbles_to_bytes(
            &nbt_value_lookup_byte_array(&section, "Add")
                .unwrap_or_else(|| vec![0; blocks.len() / 2]),
        );
        let data = packed_nibbles_to_bytes(&nbt_value_lookup_byte_array(&section, "Data").unwrap());

        return blocks
            .iter()
            .enumerate()
            .map(|(index, block)| (index, ((add[index] as u16) << 8) + ((*block as u16) & 0xFF)))
            .filter_map(|(index, block)| {
                match block {
                    // All doors
                    64 | 71 | 193..=197 => {
                        // Doors. Check if top or bottom, generate tuple of
                        // coordinates and pesudo block entity
                        let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                        if (data[index] & 0x8) == 0x8 {
                            // Top of door
                            Some((
                                coordinates,
                                BlockEntity::PseudoDoorTop {
                                    hinge: if (data[index] & 0x1) == 0x1 {
                                        Hinge::Right
                                    } else {
                                        Hinge::Left
                                    },
                                },
                            ))
                        } else {
                            // Bottom of door
                            Some((
                                coordinates,
                                BlockEntity::PseudoDoorBottom {
                                    open: (data[index] & 0x4) == 0x4,
                                    facing: facing4_wnes(data[index]),
                                },
                            ))
                        }
                    }

                    // Large flowers / grass / ferns
                    175 => match data[index] {
                        0..=1 | 4..=5 => Some((
                            Self::coordinates(section_y_index, xz_offset, index),
                            BlockEntity::PseudoFlowerBottom(match data[index] {
                                0 => Flower::SunflowerBottom,
                                1 => Flower::LilacBottom,
                                4 => Flower::RoseBushBottom,
                                5 => Flower::PeonyBottom,
                                _ => unreachable!(),
                            }),
                        )),
                        2..=3 => Some((
                            Self::coordinates(section_y_index, xz_offset, index),
                            BlockEntity::PseudoGrassBottom(match data[index] {
                                2 => Grass::TallGrassBottom,
                                3 => Grass::LargeFernBottom,
                                _ => unreachable!(),
                            }),
                        )),
                        _ => None, // Top flower block does not get pseudo entity
                    },
                    _ => None,
                }
            })
            .collect();

        fn facing4_wnes(data: i8) -> Surface4 {
            match data & 0x3 {
                0 => Surface4::West,
                1 => Surface4::North,
                2 => Surface4::East,
                3 => Surface4::South,
                _ => unreachable!(),
            }
        }
    }

    // This function reads a "Section" nbt entry, converting it into an array of
    // block::Block elements, using the save format of Minecraft 1.12.2.
    // It also needs a pre-parsed hasmap of block entities, including internal
    // "pseudo block entities" for two-part block structures such as doors and
    // large flowers. Those structures have some metadata in the top block, and
    // some metadata in the bottom block, while the internal mcprogedit format
    // keeps all data in both blocks.
    pub(crate) fn pre_flattening_section_into_block_cuboid(
        section: &nbt::Value,
        block_entities: &HashMap<BlockCoord, BlockEntity>,
        chunk_position: &ChunkCoord,
        block_cuboid: &mut BlockCuboid,
    ) {
        let xz_offset: BlockCoord = chunk_position.into();
        let section_y_index = nbt_value_lookup_byte(&section, "Y").unwrap() as i64;
        let blocks = nbt_value_lookup_byte_array(&section, "Blocks").unwrap();
        let add = packed_nibbles_to_bytes(
            &nbt_value_lookup_byte_array(&section, "Add")
                .unwrap_or_else(|| vec![0; blocks.len() / 2]),
        );
        let data = packed_nibbles_to_bytes(&nbt_value_lookup_byte_array(&section, "Data").unwrap());

        //let mut block_cuboid = BlockCuboid::new((16, 16, 16));
        blocks
            .iter()
            .enumerate()
            .map(|(index, block)| (index, ((add[index] as u16) << 8) + ((*block as u16) & 0xFF)))
            .map(|(index, block)| {
                (
                    index,
                    match block {
                        0 => Block::Air,
                        1 => match data[index] {
                            0 => Block::Stone,
                            1 => Block::Granite,
                            2 => Block::PolishedGranite,
                            3 => Block::Diorite,
                            4 => Block::PolishedDiorite,
                            5 => Block::Andesite,
                            6 => Block::PolishedAndesite,
                            n => panic!("Unknown stone data variant: {}", n),
                        },
                        2 => Block::GrassBlock,
                        3 => match data[index] {
                            0 => Block::Dirt,
                            1 => Block::CoarseDirt,
                            2 => Block::Podzol,
                            n => panic!("Unknown dirt data variant: {}", n),
                        },
                        4 => Block::Cobblestone,
                        5 => Block::Planks {
                            material: match data[index] {
                                0 => WoodMaterial::Oak,
                                1 => WoodMaterial::Spruce,
                                2 => WoodMaterial::Birch,
                                3 => WoodMaterial::Jungle,
                                4 => WoodMaterial::Acacia,
                                5 => WoodMaterial::DarkOak,
                                n => panic!("Unknown plank data variant: {}", n),
                            },
                        },
                        6 => Block::Sapling {
                            growth_stage: Int0Through1::new((data[index] & 0x8) >> 3).unwrap(),
                            material: match data[index] & 0x7 {
                                0 => SaplingMaterial::Oak,
                                1 => SaplingMaterial::Spruce,
                                2 => SaplingMaterial::Birch,
                                3 => SaplingMaterial::Jungle,
                                4 => SaplingMaterial::Acacia,
                                5 => SaplingMaterial::DarkOak,
                                n => panic!("Unknown sapling data variant: {}", n),
                            },
                        },
                        7 => Block::Bedrock,
                        #[allow(clippy::verbose_bit_mask)]
                        8 | 9 => {
                            if (data[index] & 0x7) == 0x0 {
                                Block::WaterSource
                            } else {
                                Block::Water {
                                    falling: (data[index] & 0x8) == 0x8,
                                    level: Int1Through7::new(8 - (data[index] & 0x7)).unwrap(),
                                }
                            }
                        }
                        #[allow(clippy::verbose_bit_mask)]
                        10 | 11 => {
                            if (data[index] & 0x7) == 0x0 {
                                Block::LavaSource
                            } else {
                                Block::Lava {
                                    falling: (data[index] & 0x8) == 0x8,
                                    level: Int1Through7::new(8 - (data[index] & 0x7)).unwrap(),
                                }
                            }
                        }
                        12 => match data[index] {
                            0 => Block::Sand,
                            1 => Block::RedSand,
                            n => panic!("Unknown sand data variant: {}", n),
                        },
                        13 => Block::Gravel,
                        14 => Block::GoldOre,
                        15 => Block::IronOre,
                        16 => Block::CoalOre,
                        17 => Block::Log(Log {
                            material: match data[index] & 0x3 {
                                0 => WoodMaterial::Oak,
                                1 => WoodMaterial::Spruce,
                                2 => WoodMaterial::Birch,
                                3 => WoodMaterial::Jungle,
                                _ => unreachable!(),
                            },
                            alignment: wood_alignment(data[index]),
                            stripped: false,
                        }),
                        18 => Block::Leaves {
                            material: match data[index] & 0x3 {
                                0 => LeavesMaterial::Oak,
                                1 => LeavesMaterial::Spruce,
                                2 => LeavesMaterial::Birch,
                                3 => LeavesMaterial::Jungle,
                                _ => unreachable!(),
                            },
                            distance_to_trunk: None,
                            persistent: (data[index] & 0x4) == 0x4,
                        },
                        19 => match data[index] {
                            0 => Block::Sponge,
                            1 => Block::WetSponge,
                            n => panic!("Unknown sponge data variant: {}", n),
                        },
                        20 => Block::Glass { colour: None },
                        21 => Block::LapisLazuliOre,
                        22 => Block::LapisLazuliBlock,
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
                        24 => match data[index] {
                            0 => Block::Sandstone,
                            1 => Block::ChiseledSandstone,
                            2 => Block::SmoothSandstone,
                            n => panic!("Unknown sandstone data variant: {}", n),
                        },
                        25 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates).unwrap();

                            if let BlockEntity::Noteblock { note, .. } = block_entity {
                                Block::Noteblock(Noteblock {
                                    pitch: note.clone(),
                                })
                            } else {
                                panic!("Wrong block entity variant for note block")
                            }
                        }
                        26 => Block::Bed(Bed {
                            colour: Colour::Red,
                            facing: facing4_swne(data[index]),
                            end: if (data[index] & 0x8) == 0x8 {
                                BedEnd::Head
                            } else {
                                BedEnd::Foot
                            },
                        }),
                        27 => Block::Rail {
                            variant: RailType::Powered,
                            shape: RailShape::from_value(data[index] & 0x7),
                        },
                        28 => Block::Rail {
                            variant: RailType::Detector,
                            shape: RailShape::from_value(data[index] & 0x7),
                        },
                        29 => Block::StickyPiston {
                            facing: facing6_dunswe(data[index]),
                            extended: data[index] & 0x8 == 0x8,
                        },
                        30 => Block::Cobweb,
                        31 => Block::Grass(match data[index] & 0x3 {
                            1 => Grass::Grass,
                            2 => Grass::Fern,
                            n @ 0 | n @ 3 => panic!("Unkown grass data variant: {}", n),
                            _ => unreachable!(),
                        }),
                        32 => Block::DeadBush,
                        33 => Block::Piston {
                            facing: facing6_dunswe(data[index]),
                            extended: data[index] & 0x8 == 0x8,
                        },
                        34 => {
                            let facing = facing6_dunswe(data[index]);
                            if data[index] & 0x8 == 0x8 {
                                Block::StickyPistonHead { facing }
                            } else {
                                Block::PistonHead { facing }
                            }
                        }
                        35 => Block::Wool {
                            colour: ((data[index] & 0xF) as i32).into(),
                        },
                        // TODO block 36 piston_extension ("Block moved by Piston")
                        37 => Block::Flower(Flower::Dandelion),
                        38 => Block::Flower(match data[index] {
                            0 => Flower::Poppy,
                            1 => Flower::BlueOrchid,
                            2 => Flower::Allium,
                            3 => Flower::AzureBluet,
                            4 => Flower::TulipRed,
                            5 => Flower::TulipOrange,
                            6 => Flower::TulipWhite,
                            7 => Flower::TulipPink,
                            8 => Flower::OxeyeDaisy,
                            n => panic!("Unkown red flower data variant: {}", n),
                        }),
                        39 => Block::BrownMushroom,
                        40 => Block::RedMushroom,
                        41 => Block::BlockOfGold,
                        42 => Block::BlockOfIron,
                        43 => Block::Slab(Slab {
                            material: match data[index] & 0x7 {
                                0 => SlabMaterial::SmoothStone,
                                1 => SlabMaterial::Sandstone,
                                2 => SlabMaterial::PetrifiedOak, // legacy
                                3 => SlabMaterial::Cobblestone,
                                4 => SlabMaterial::Brick,
                                5 => SlabMaterial::StoneBrick,
                                6 => SlabMaterial::NetherBrick,
                                7 => SlabMaterial::Quartz,
                                _ => unreachable!(),
                            },
                            position: SlabVariant::Double,
                            waterlogged: false,
                        }),
                        44 => Block::Slab(Slab {
                            material: match data[index] & 0x7 {
                                0 => SlabMaterial::SmoothStone,
                                1 => SlabMaterial::Sandstone,
                                2 => SlabMaterial::PetrifiedOak, // legacy
                                3 => SlabMaterial::Cobblestone,
                                4 => SlabMaterial::Brick,
                                5 => SlabMaterial::StoneBrick,
                                6 => SlabMaterial::NetherBrick,
                                7 => SlabMaterial::Quartz,
                                _ => unreachable!(),
                            },
                            position: if (data[index] & 0x8) == 0x8 {
                                SlabVariant::Top
                            } else {
                                SlabVariant::Bottom
                            },
                            waterlogged: false,
                        }),
                        45 => Block::BrickBlock,
                        46 => Block::TNT,
                        47 => Block::Bookshelf,
                        48 => Block::MossyCobblestone,
                        49 => Block::Obsidian,
                        50 => Block::Torch {
                            attached: facing5_xwensd(data[index]),
                        },
                        51 => Block::Fire {
                            age: Int0Through15::new(data[index] & 0xF).unwrap(),
                        },
                        // TODO block 52 mob spawner
                        53 => Block::Stairs(Stair {
                            material: StairMaterial::Oak,
                            position: (data[index] & 0x7).into(),
                            waterlogged: false,
                        }),
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
                        55 => Block::RedstoneWire,
                        56 => Block::DiamondOre,
                        57 => Block::BlockOfDiamond,
                        58 => Block::CraftingTable,
                        59 => Block::Wheat {
                            growth_stage: Int0Through7::new(data[index] & 0x7).unwrap(),
                        },
                        60 => Block::Farmland {
                            wetness: Int0Through7::new(data[index] & 0x7).unwrap(),
                        },
                        61 | 62 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates).unwrap();

                            match block_entity {
                                BlockEntity::Furnace { tags } => {
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
                                _ => panic!("Wrong block entity variant for chest"),
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
                        // All doors
                        64 | 71 | 193..=197 => {
                            let half = if (data[index] & 0x8) == 0x8 {
                                DoorHalf::Upper
                            } else {
                                DoorHalf::Lower
                            };

                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);

                            let top_coordinates = match half {
                                DoorHalf::Upper => coordinates,
                                DoorHalf::Lower => coordinates + (0, 1, 0).into(),
                            };

                            let bottom_coordinates = match half {
                                DoorHalf::Upper => coordinates - (0, 1, 0).into(),
                                DoorHalf::Lower => coordinates,
                            };

                            let top_entity = block_entities.get(&top_coordinates).unwrap();
                            let bottom_entity = block_entities.get(&bottom_coordinates).unwrap();

                            match (top_entity, bottom_entity) {
                                (
                                    BlockEntity::PseudoDoorTop { hinge, .. },
                                    BlockEntity::PseudoDoorBottom { open, facing, .. },
                                ) => Block::Door(Door {
                                    facing: *facing,
                                    half,
                                    hinged_at: hinge.clone(),
                                    open: *open,
                                    material: match block {
                                        64 => DoorMaterial::Oak,
                                        71 => DoorMaterial::Iron,
                                        193 => DoorMaterial::Spruce,
                                        194 => DoorMaterial::Birch,
                                        195 => DoorMaterial::Jungle,
                                        196 => DoorMaterial::Acacia,
                                        197 => DoorMaterial::DarkOak,
                                        _ => unreachable!(),
                                    },
                                }),
                                _ => panic!("Wrong block entity variant(s) for door"),
                            }
                        }
                        65 => Block::Ladder {
                            facing: facing4_xxnswe(data[index]),
                            waterlogged: false,
                        },
                        66 => Block::Rail {
                            variant: RailType::Normal,
                            shape: RailShape::from_value(data[index]),
                        },
                        67 => Block::Stairs(Stair {
                            material: StairMaterial::Cobblestone,
                            position: (data[index] & 0x7).into(),
                            waterlogged: false,
                        }),
                        // 68 wall sign - already handled
                        69 => Block::Lever(
                            lever_facing(data[index]),
                            if data[index] & 0x8 == 0x8 {
                                OnOffState::On
                            } else {
                                OnOffState::Off
                            },
                        ),
                        70 => Block::PressurePlate {
                            material: PressurePlateMaterial::Stone,
                        },
                        // 71 Iron door - already handled
                        72 => Block::PressurePlate {
                            material: PressurePlateMaterial::Oak,
                        },
                        73 | 74 => Block::RedstoneOre,
                        75 | 76 => Block::RedstoneTorch {
                            attached: facing5_xwensd(data[index]),
                        },
                        77 => Block::Button(ButtonMaterial::Stone, facing6_dewsnu(data[index])),
                        78 => Block::Snow {
                            thickness: Int1Through8::new((data[index] & 0x7) + 1).unwrap(),
                        },
                        79 => Block::Ice,
                        80 => Block::SnowBlock,
                        81 => Block::Cactus {
                            growth_stage: Int0Through15::new(data[index] & 0xF).unwrap(),
                        },
                        82 => Block::Clay,
                        83 => Block::SugarCane {
                            growth_stage: Int0Through15::new(data[index] & 0xF).unwrap(),
                        },
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
                        // All fences
                        85 | 113 | 188..=192 => Block::Fence {
                            waterlogged: false,
                            material: match block {
                                85 => FenceMaterial::Oak,
                                113 => FenceMaterial::NetherBrick,
                                188 => FenceMaterial::Spruce,
                                189 => FenceMaterial::Birch,
                                190 => FenceMaterial::Jungle,
                                191 => FenceMaterial::DarkOak,
                                192 => FenceMaterial::Acacia,
                                _ => unreachable!(),
                            },
                        },
                        86 => Block::Pumpkin {
                            facing: facing4_swne(data[index]),
                        },
                        87 => Block::Netherrack,
                        88 => Block::SoulSand,
                        89 => Block::Glowstone,
                        90 => Block::NetherPortal { alignment: None },
                        91 => Block::JackOLantern {
                            facing: facing4_swne(data[index]),
                        },
                        92 => Block::Cake {
                            pieces: Int1Through7::new(7 - (data[index] & 0x7)).unwrap(),
                        },
                        93 | 94 => Block::RedstoneRepeater(RedstoneRepeater {
                            facing: facing4_nesw(data[index]),
                            delay: Int1Through4::new(((data[index] >> 2) & 0x3) + 1).unwrap(),
                        }),
                        95 => Block::Glass {
                            colour: Some(((data[index] & 0xF) as i32).into()),
                        },
                        // All trapdoors
                        96 | 167 => Block::Trapdoor(Trapdoor {
                            hinge_at: trapdoor_hinge_at(data[index]),
                            open: data[index] & 0x4 == 0x4,
                            waterlogged: false,
                            material: match block {
                                96 => DoorMaterial::Oak,
                                167 => DoorMaterial::Iron,
                                _ => unreachable!(),
                            },
                        }),
                        97 => match data[index] {
                            0 => Block::InfestedStone,
                            1 => Block::InfestedCobblestone,
                            2 => Block::InfestedStoneBricks,
                            3 => Block::InfestedMossyStoneBricks,
                            4 => Block::InfestedCrackedStoneBricks,
                            5 => Block::InfestedChiseledStoneBricks,
                            n => panic!("Unknown infested block data variant: {}", n),
                        },
                        98 => match data[index] {
                            0 => Block::StoneBricks,
                            1 => Block::MossyStoneBricks,
                            2 => Block::CrackedStoneBricks,
                            3 => Block::ChiseledStoneBricks,
                            n => panic!("Unknown stone brick data variant: {}", n),
                        },
                        99 | 100 => match data[index] {
                            stem @ 10 | stem @ 15 => Block::MushroomStem {
                                stem_directions: mushroom_caps(stem),
                            },
                            cap => {
                                let cap_directions = mushroom_caps(cap);
                                if block == 99 {
                                    Block::BrownMushroomBlock { cap_directions }
                                } else if block == 100 {
                                    Block::RedMushroomBlock { cap_directions }
                                } else {
                                    unreachable!();
                                }
                            }
                        },
                        101 => Block::IronBars { waterlogged: false },
                        102 => Block::GlassPane {
                            colour: None,
                            waterlogged: false,
                        },
                        103 => Block::Melon,
                        104 => Block::PumpkinStem {
                            state: StemState::Growing(
                                Int0Through7::new(data[index] & 0x7).unwrap(),
                            ),
                        },
                        105 => Block::MelonStem {
                            state: StemState::Growing(
                                Int0Through7::new(data[index] & 0x7).unwrap(),
                            ),
                        },
                        106 => Block::Vines(Vines {
                            anchored_at: DirectionFlags6 {
                                east: data[index] & 0x8 == 0x8,
                                down: false,
                                north: data[index] & 0x4 == 0x4,
                                south: data[index] & 0x1 == 0x1,
                                up: false,
                                west: data[index] & 0x2 == 0x2,
                            },
                        }),
                        // All fence gates
                        107 | 183..=187 => Block::FenceGate {
                            facing: facing4_swne(data[index]),
                            open: data[index] & 0x4 == 0x4,
                            material: match block {
                                107 => WoodMaterial::Oak,
                                183 => WoodMaterial::Spruce,
                                184 => WoodMaterial::Birch,
                                185 => WoodMaterial::Jungle,
                                186 => WoodMaterial::DarkOak,
                                187 => WoodMaterial::Acacia,
                                _ => unreachable!(),
                            },
                        },
                        108 => Block::Stairs(Stair {
                            material: StairMaterial::Brick,
                            position: (data[index] & 0x7).into(),
                            waterlogged: false,
                        }),
                        109 => Block::Stairs(Stair {
                            material: StairMaterial::StoneBrick,
                            position: (data[index] & 0x7).into(),
                            waterlogged: false,
                        }),
                        110 => Block::Mycelium,
                        111 => Block::LilyPad,
                        112 => Block::NetherBricks,
                        // 113 nether brick fence - already handled
                        114 => Block::Stairs(Stair {
                            material: StairMaterial::NetherBrick,
                            position: (data[index] & 0x7).into(),
                            waterlogged: false,
                        }),
                        115 => Block::NetherWart {
                            growth_stage: Int0Through3::new(data[index] & 0x3).unwrap(),
                        },
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
                        117 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates).unwrap();

                            match block_entity {
                                BlockEntity::BrewingStand {
                                    custom_name,
                                    lock,
                                    items,
                                    brew_time,
                                    fuel,
                                    ..
                                } => Block::BrewingStand(Box::new(BrewingStand {
                                    custom_name: custom_name.clone(),
                                    lock: lock.clone(),
                                    items: items.clone(),
                                    brew_time: *brew_time,
                                    fuel: *fuel,
                                })),
                                _ => panic!("Wrong block entity variant for brewing stand"),
                            }
                        }
                        118 => Block::Cauldron {
                            water_level: Int0Through3::new(data[index] & 0x3).unwrap(),
                        },
                        119 => Block::EndPortal, // TODO check if block entity data is needed
                        120 => Block::EndPortalFrame {
                            facing: facing4_swne(data[index]),
                            has_eye: data[index] & 0x4 == 0x4,
                        },
                        121 => Block::EndStone,
                        122 => Block::DragonEgg,
                        123 | 124 => Block::RedstoneLamp,
                        125 => Block::Slab(Slab {
                            material: match data[index] & 0x7 {
                                0 => SlabMaterial::Oak,
                                1 => SlabMaterial::Spruce,
                                2 => SlabMaterial::Birch,
                                3 => SlabMaterial::Jungle,
                                4 => SlabMaterial::Acacia,
                                5 => SlabMaterial::DarkOak,
                                n @ 6..=7 => panic!("Unknown double wooden slab data value: {}", n),
                                _ => unreachable!(),
                            },
                            position: SlabVariant::Double,
                            waterlogged: false,
                        }),
                        126 => Block::Slab(Slab {
                            material: match data[index] & 0x7 {
                                0 => SlabMaterial::Oak,
                                1 => SlabMaterial::Spruce,
                                2 => SlabMaterial::Birch,
                                3 => SlabMaterial::Jungle,
                                4 => SlabMaterial::Acacia,
                                5 => SlabMaterial::DarkOak,
                                n @ 6..=7 => panic!("Unknown double wooden slab data value: {}", n),
                                _ => unreachable!(),
                            },
                            position: if (data[index] & 0x8) == 0x8 {
                                SlabVariant::Top
                            } else {
                                SlabVariant::Bottom
                            },
                            waterlogged: false,
                        }),
                        127 => Block::CocoaBeans {
                            growth_stage: Int0Through2::new((data[index] & 0xC) >> 2).unwrap(),
                            facing: facing4_nesw(data[index]),
                        },
                        128 => Block::Stairs(Stair {
                            material: StairMaterial::Sandstone,
                            position: (data[index] & 0x7).into(),
                            waterlogged: false,
                        }),
                        129 => Block::EmeraldOre,
                        130 => Block::EnderChest {
                            facing: facing4_xxnswe(data[index]),
                            waterlogged: false,
                            // TODO check if block entity data is needed
                        },
                        131 => Block::TripwireHook {
                            facing: facing4_swne(data[index]),
                        },
                        132 => Block::Tripwire,
                        133 => Block::BlockOfEmerald,
                        134 => Block::Stairs(Stair {
                            material: StairMaterial::Spruce,
                            position: (data[index] & 0x7).into(),
                            waterlogged: false,
                        }),
                        135 => Block::Stairs(Stair {
                            material: StairMaterial::Birch,
                            position: (data[index] & 0x7).into(),
                            waterlogged: false,
                        }),
                        136 => Block::Stairs(Stair {
                            material: StairMaterial::Jungle,
                            position: (data[index] & 0x7).into(),
                            waterlogged: false,
                        }),
                        // TODO 137 command block // Deferred for now, too complicated
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
                                    primary: primary.clone(),
                                    secondary: secondary.clone(),
                                })),
                                _ => panic!("Wrong block entity variant for beacon"),
                            }
                        }
                        139 => Block::Wall {
                            material: match data[index] {
                                0 => WallMaterial::Cobblestone,
                                1 => WallMaterial::MossyCobblestone,
                                n => panic!(
                                    "Unknown material data value for cobblestone wall: {}",
                                    n,
                                ),
                            },
                            waterlogged: false,
                        },
                        140 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates);

                            if let Some(BlockEntity::FlowerPot { plant, .. }) = block_entity {
                                Block::FlowerPot(FlowerPot { plant: *plant })
                            } else {
                                Block::FlowerPot(FlowerPot { plant: None })
                            }
                        }
                        141 => Block::Carrots {
                            growth_stage: Int0Through7::new(data[index] & 0x7).unwrap(),
                        },
                        142 => Block::Potatoes {
                            growth_stage: Int0Through7::new(data[index] & 0x7).unwrap(),
                        },
                        143 => Block::Button(ButtonMaterial::Oak, facing6_dewsnu(data[index])),
                        144 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates).unwrap();

                            match block_entity {
                                BlockEntity::Skull {
                                    skull_type, facing, ..
                                } => Block::Head(Head {
                                    variant: *skull_type,
                                    placement: match data[index] {
                                        1 => WallOrRotatedOnFloor::Floor(*facing),
                                        2 => WallOrRotatedOnFloor::Wall(Surface4::North),
                                        3 => WallOrRotatedOnFloor::Wall(Surface4::South),
                                        4 => WallOrRotatedOnFloor::Wall(Surface4::West),
                                        5 => WallOrRotatedOnFloor::Wall(Surface4::East),
                                        n => panic!("Unknown data value for skull: {}", n),
                                    },
                                    waterlogged: false,
                                }),
                                _ => panic!("Wrong block entity variant for skull / head"),
                            }
                        }
                        145 => Block::Anvil {
                            facing: facing4_swne(data[index]),
                            damage: match data[index] & 0b1100 {
                                0b0000 => AnvilDamage::Intact,
                                0b0100 => AnvilDamage::SlightlyDamaged,
                                0b1000 => AnvilDamage::VeryDamaged,
                                n => panic!("Unknown anvil damage data value: {}", n),
                            },
                        },
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
                        147 => Block::PressurePlate {
                            material: PressurePlateMaterial::Gold,
                        },
                        148 => Block::PressurePlate {
                            material: PressurePlateMaterial::Iron,
                        },
                        149 | 150 => {
                            let facing = facing4_nesw(data[index]);
                            if data[index] & 0x4 == 0x4 {
                                Block::RedstoneSubtractor { facing }
                            } else {
                                Block::RedstoneComparator { facing }
                            }
                        }
                        151 => Block::DaylightDetector,
                        152 => Block::BlockOfRedstone,
                        153 => Block::QuartzOre,
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
                        155 => match data[index] {
                            0 => Block::BlockOfQuartz,
                            1 => Block::ChiseledQuartzBlock,
                            2 => Block::QuartzPillar {
                                alignment: Axis3::Y,
                            },
                            3 => Block::QuartzPillar {
                                alignment: Axis3::X,
                            },
                            4 => Block::QuartzPillar {
                                alignment: Axis3::Z,
                            },
                            n => panic!("Unknown data value for quartz block: {}", n),
                        },
                        156 => Block::Stairs(Stair {
                            material: StairMaterial::Quartz,
                            position: (data[index] & 0x7).into(),
                            waterlogged: false,
                        }),
                        157 => Block::Rail {
                            variant: RailType::Activator,
                            shape: RailShape::from_value(data[index] & 0x7),
                        },
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
                        159 => Block::Terracotta {
                            colour: Some(((data[index] & 0xF) as i32).into()),
                        },
                        160 => Block::GlassPane {
                            colour: Some(((data[index] & 0xF) as i32).into()),
                            waterlogged: false,
                        },
                        161 => Block::Leaves {
                            material: match data[index] & 0x1 {
                                0 => LeavesMaterial::Acacia,
                                1 => LeavesMaterial::DarkOak,
                                _ => unreachable!(),
                            },
                            distance_to_trunk: None,
                            persistent: (data[index] & 0x4) == 0x4,
                        },
                        162 => Block::Log(Log {
                            material: match data[index] & 0x1 {
                                0 => WoodMaterial::Acacia,
                                1 => WoodMaterial::DarkOak,
                                _ => unreachable!(),
                            },
                            alignment: wood_alignment(data[index]),
                            stripped: false,
                        }),
                        163 => Block::Stairs(Stair {
                            material: StairMaterial::Acacia,
                            position: (data[index] & 0x7).into(),
                            waterlogged: false,
                        }),
                        164 => Block::Stairs(Stair {
                            material: StairMaterial::DarkOak,
                            position: (data[index] & 0x7).into(),
                            waterlogged: false,
                        }),
                        165 => Block::BlockOfSlime,
                        166 => Block::Barrier,
                        // 167 iron trapdoor - already handled
                        168 => match data[index] {
                            0 => Block::Prismarine,
                            1 => Block::PrismarineBricks,
                            2 => Block::DarkPrismarine,
                            n => panic!("Unknown data value for prismarine: {}", n),
                        },
                        169 => Block::SeaLantern,
                        170 => Block::HayBale {
                            alignment: match data[index] {
                                0 => Axis3::Y,
                                4 => Axis3::X,
                                8 => Axis3::Z,
                                n => panic!("Unknown data value for hay bale alignment: {}", n),
                            },
                        },
                        171 => Block::Carpet {
                            colour: ((data[index] & 0xF) as i32).into(),
                        },
                        172 => Block::Terracotta { colour: None },
                        173 => Block::BlockOfCoal,
                        174 => Block::PackedIce,
                        // All double tall plants (Flowers, Ferns, Grass)
                        175 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let entity_coordinates = if (data[index] & 0x8) == 0x8 {
                                // top block; pseudo block entity is found at the bottom
                                coordinates - (0, 1, 0).into()
                            } else {
                                // bottom block; pseudo block entity is found here
                                coordinates
                            };
                            match block_entities.get(&entity_coordinates).unwrap() {
                                BlockEntity::PseudoFlowerBottom(bottom_flower) => {
                                    if (data[index] & 0x8) == 0x8 {
                                        let top_flower = match bottom_flower {
                                            Flower::LilacBottom => Flower::LilacTop,
                                            Flower::PeonyBottom => Flower::PeonyTop,
                                            Flower::RoseBushBottom => Flower::RoseBushTop,
                                            Flower::SunflowerBottom => Flower::SunflowerTop,
                                            variant => panic!(
                                                "Unexpected grass variant for bottom grass: {:?}",
                                                variant,
                                            ),
                                        };
                                        Block::Flower(top_flower)
                                    } else {
                                        Block::Flower(bottom_flower.clone())
                                    }
                                }
                                BlockEntity::PseudoGrassBottom(bottom_grass) => {
                                    if (data[index] & 0x8) == 0x8 {
                                        let top_grass = match bottom_grass {
                                            Grass::LargeFernBottom => Grass::LargeFernTop,
                                            Grass::TallGrassBottom => Grass::TallGrassTop,
                                            variant => panic!(
                                                "Unexpected grass variant for bottom grass: {:?}",
                                                variant,
                                            ),
                                        };
                                        Block::Grass(top_grass)
                                    } else {
                                        Block::Grass(bottom_grass.clone())
                                    }
                                }
                                _ => panic!("Wrong block entity variant for flower or grass"),
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
                        178 => Block::InvertedDaylightDetector,
                        179 => match data[index] {
                            0 => Block::RedSandstone,
                            1 => Block::ChiseledRedSandstone,
                            2 => Block::SmoothRedSandstone,
                            n => panic!("Unknown data variant for red sandstone block: {}", n),
                        },
                        180 => Block::Stairs(Stair {
                            material: StairMaterial::RedSandstone,
                            position: (data[index] & 0x7).into(),
                            waterlogged: false,
                        }),
                        181 => Block::Slab(Slab {
                            material: SlabMaterial::RedSandstone,
                            position: SlabVariant::Double,
                            waterlogged: false,
                        }),
                        182 => Block::Slab(Slab {
                            material: SlabMaterial::RedSandstone,
                            position: if (data[index] & 0x8) == 0x8 {
                                SlabVariant::Top
                            } else {
                                SlabVariant::Bottom
                            },
                            waterlogged: false,
                        }),
                        // 183..=187 various fence gates - already handled
                        // 188..=192 various fences - already handled
                        // 193..=197 various doors - already handled
                        198 => Block::EndRod {
                            facing: facing6_dunswe(data[index]),
                        },
                        199 => Block::ChorusPlant(ChorusPlant {
                            // TODO actually figure out how to parse connections
                            // For now:
                            // - use same as mushroom caps, and hope for the best...
                            connections: mushroom_caps(data[index]),
                        }),
                        200 => Block::ChorusFlower {
                            growth_stage: Int0Through5::new(data[index]).unwrap(),
                        },
                        201 => Block::PurpurBlock,
                        202 => Block::PurpurPillar {
                            alignment: match data[index] {
                                0 => Axis3::Y,
                                4 => Axis3::X,
                                8 => Axis3::Z,
                                n => {
                                    panic!("Unknown data value for purpur pillar alignment: {}", n)
                                }
                            },
                        },
                        203 => Block::Stairs(Stair {
                            material: StairMaterial::Purpur,
                            position: (data[index] & 0x7).into(),
                            waterlogged: false,
                        }),
                        204 => Block::Slab(Slab {
                            material: SlabMaterial::Purpur,
                            position: SlabVariant::Double,
                            waterlogged: false,
                        }),
                        205 => Block::Slab(Slab {
                            material: SlabMaterial::Purpur,
                            position: if (data[index] & 0x8) == 0x8 {
                                SlabVariant::Top
                            } else {
                                SlabVariant::Bottom
                            },
                            waterlogged: false,
                        }),
                        206 => Block::EndStoneBricks,
                        207 => Block::Beetroots {
                            growth_stage: Int0Through3::new(data[index] & 0x3).unwrap(),
                        },
                        208 => Block::GrassPath,
                        209 => Block::EndGateway,
                        // TODO 210 repeating command block
                        // TODO 211 chain command block
                        212 => Block::FrostedIce, // NB there might be data values here
                        213 => Block::MagmaBlock,
                        214 => Block::NetherWartBlock,
                        215 => Block::RedNetherBricks,
                        216 => Block::BoneBlock {
                            alignment: match data[index] {
                                0 => Axis3::Y,
                                4 => Axis3::X,
                                8 => Axis3::Z,
                                n => panic!("Unknown data value for bone block alignment: {}", n),
                            },
                        },
                        217 => Block::StructureVoid,
                        218 => Block::Observer {
                            facing: facing6_dunswe(data[index]),
                        },
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
                        // All glazed terracotta colours
                        block_id @ 235..=250 => Block::GlazedTerracotta(GlazedTerracotta {
                            colour: ((block_id - 235) as i32).into(),
                            facing: facing4_swne(data[index]),
                        }),
                        251 => Block::Concrete {
                            colour: ((data[index] & 0xF) as i32).into(),
                        },
                        252 => Block::ConcretePowder {
                            colour: ((data[index] & 0xF) as i32).into(),
                        },
                        // TODO 255 structure block
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

        fn facing4_xxnswe(data: i8) -> Surface4 {
            match data & 0x7 {
                2 => Surface4::North,
                3 => Surface4::South,
                4 => Surface4::West,
                5 => Surface4::East,
                n @ 0..=1 | n @ 6..=7 => panic!("Unknown facing4 nswe value: {}", n),
                _ => unreachable!(),
            }
        }

        fn facing4_swne(data: i8) -> Surface4 {
            match data & 0x3 {
                0 => Surface4::South,
                1 => Surface4::West,
                2 => Surface4::North,
                3 => Surface4::East,
                _ => unreachable!(),
            }
        }

        fn facing4_nesw(data: i8) -> Surface4 {
            match data & 0x3 {
                0 => Surface4::North,
                1 => Surface4::East,
                2 => Surface4::South,
                3 => Surface4::West,
                _ => unreachable!(),
            }
        }

        fn facing5_xwensd(data: i8) -> Surface5 {
            match data & 0x7 {
                1 => Surface5::West,
                2 => Surface5::East,
                3 => Surface5::North,
                4 => Surface5::South,
                5 => Surface5::Down,
                n @ 0 | n @ 6..=7 => panic!("Unknown facing5 xwensd value: {}", n),
                _ => unreachable!(),
            }
        }

        fn facing5_dxnswe(data: i8) -> Surface5 {
            match data & 0x7 {
                0 => Surface5::Down,
                2 => Surface5::North,
                3 => Surface5::South,
                4 => Surface5::West,
                5 => Surface5::East,
                n @ 1 | n @ 6..=7 => panic!("Unknown facing5 dxnswe value: {}", n),
                _ => unreachable!(),
            }
        }

        fn facing6_dewsnu(data: i8) -> Surface6 {
            match data & 0x7 {
                0 => Surface6::Down,
                1 => Surface6::East,
                2 => Surface6::West,
                3 => Surface6::South,
                4 => Surface6::North,
                5 => Surface6::Up,
                n @ 6..=7 => panic!("Unknown facing6 dunswe value: {}", n),
                _ => unreachable!(),
            }
        }

        fn facing6_dunswe(data: i8) -> Surface6 {
            match data & 0x7 {
                0 => Surface6::Down,
                1 => Surface6::Up,
                2 => Surface6::North,
                3 => Surface6::South,
                4 => Surface6::West,
                5 => Surface6::East,
                n @ 6..=7 => panic!("Unknown facing6 dunswe value: {}", n),
                _ => unreachable!(),
            }
        }

        fn trapdoor_hinge_at(data: i8) -> Edge8 {
            match data & (0x3 | 0x8) {
                0 => Edge8::DownSouth,
                1 => Edge8::DownNorth,
                2 => Edge8::DownEast,
                3 => Edge8::DownWest,
                8 => Edge8::UpSouth,
                9 => Edge8::UpNorth,
                10 => Edge8::UpEast,
                11 => Edge8::UpWest,
                _ => unreachable!(),
            }
        }

        fn wood_alignment(data: i8) -> Option<Axis3> {
            match (data & 0xC) >> 2 {
                0 => Some(Axis3::Y),
                1 => Some(Axis3::X),
                2 => Some(Axis3::Z),
                3 => None,
                _ => unreachable!(),
            }
        }

        fn lever_facing(data: i8) -> SurfaceRotation12 {
            match data & 0x7 {
                0 => SurfaceRotation12::DownFacingEast,
                1 => SurfaceRotation12::East,
                2 => SurfaceRotation12::West,
                3 => SurfaceRotation12::South,
                4 => SurfaceRotation12::North,
                5 => SurfaceRotation12::UpFacingSouth,
                6 => SurfaceRotation12::UpFacingEast,
                7 => SurfaceRotation12::DownFacingSouth,
                _ => unreachable!(),
            }
        }

        // OK this is kind of messy, but basically each number gives a combination
        // of what sides have a cap (or stem) on them. Values 10 and 15 are for stems,
        // and all other values are for caps. The caller needs to check whether the
        // resulting sides should be caps or stem, by checking if the data value is
        // 10 or 15 (stem) or any other value (caps).
        fn mushroom_caps(data: i8) -> DirectionFlags6 {
            // Only the four least significant bytes count
            let data = data & 0xF;

            // Prepare direction flags
            let east = (data <= 9 && (data % 3) == 0) || data == 10 || data >= 14;
            let down = data >= 14;
            let north = (1..=3).contains(&data) || data == 10 || data >= 14;
            let south = (7..=10).contains(&data) || data >= 14;
            let up = (1..=9).contains(&data) || data >= 14;
            let west = (data <= 10 && (data % 3) == 1) || data >= 14;

            // Create and return value
            DirectionFlags6 {
                east,
                down,
                north,
                south,
                up,
                west,
            }
        }
    }
}

// FIXME there may be something going on with i8 overflow,
// which makes the behaviour different from with u8.
/// Convert byte vector of packed nibbles into byte vector
/// The packing is little endian
fn packed_nibbles_to_bytes(nibbles: &[i8]) -> Vec<i8> {
    nibbles
        .iter()
        .flat_map(|byte| vec![byte & 0x0F, (byte >> 4) & 0x0F])
        .collect()
}

// FIXME there may be something going on with i8 overflow,
// which makes the behaviour different from with u8.
/// Convert byte vector into byte vector of packed nibbles
/// The packing is little endian
fn _bytes_to_packed_nibbles(bytes: &[i8]) -> Vec<i8> {
    bytes
        .chunks(2)
        .map(|c| c.iter().fold(0i8, |acc, x| (acc >> 4) + ((x & 0x0F) << 4)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // FIXME test the full range 0-F for the nibbles.
    #[test]
    fn test_packed_nibbles_to_bytes() {
        assert_eq!(
            packed_nibbles_to_bytes(&[0x10, 0x32, 0x54, 0x76]),
            vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7]
        );
    }

    // FIXME test the full range 0-F for the nibbles.
    #[test]
    fn test_bytes_to_packed_nibbles() {
        assert_eq!(
            _bytes_to_packed_nibbles(&[0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7]),
            vec![0x10, 0x32, 0x54, 0x76]
        );
    }
}
