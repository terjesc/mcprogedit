use std::collections::HashMap;

use crate::block;
use crate::block::{
    BedEnd, Block, Chest, Dispenser, Flower, Furnace, Jukebox, OnOffState, RailShape, RailType, Sign, Slab, SlabVariant, Stair,
};
use crate::block_entity::BlockEntity;
use crate::bounded_ints::*;
use crate::colour::Colour;
use crate::coordinates::{BlockCoord, ChunkCoord};
use crate::material::*;
use crate::mc_version::McVersion;
use crate::nbt_lookup::*;
use crate::positioning::*;

#[derive(Clone)]
pub enum RawChunkData {
    Empty,
    GZip(Vec<u8>),
    ZLib(Vec<u8>),
    Uncompressed(Vec<u8>),
}

impl RawChunkData {
    fn to_nbt(&self) -> nbt::Blob {
        match self {
            RawChunkData::GZip(chunk_data) => {
                println!("Has GZip data!");
                let mut cursor = std::io::Cursor::new(chunk_data);
                nbt::Blob::from_gzip_reader(&mut cursor)
                    .unwrap_or_else(|err| panic!("Bad chunk read: {}", err))
            }
            RawChunkData::ZLib(chunk_data) => {
                println!("Has ZLib data!");
                let mut cursor = std::io::Cursor::new(chunk_data);
                nbt::Blob::from_zlib_reader(&mut cursor)
                    .unwrap_or_else(|err| panic!("Bad chunk read: {}", err))
            }
            RawChunkData::Uncompressed(chunk_data) => {
                println!("Has uncompressed data!");
                let mut cursor = std::io::Cursor::new(chunk_data);
                nbt::Blob::from_reader(&mut cursor)
                    .unwrap_or_else(|err| panic!("Bad chunk read: {}", err))
            }
            RawChunkData::Empty => nbt::Blob::new(),
        }
    }
}

pub struct Chunk {
    data_version: McVersion,
    global_pos: ChunkCoord,
    last_update: i64,
    //biome: BiomeMapping,
    //entities: HashMap<BlockCoord, Vec<Entity>>,
    //blocks: ???<Block>, // ???
}

impl Chunk {
    pub fn from_raw_chunk_data(data: &RawChunkData) -> Self {
        let nbt = data.to_nbt();
        //println!("{}", nbt);

        let data_version = nbt_blob_lookup_int(&nbt, "DataVersion")
            .map(McVersion::from_id)
            .unwrap();

        let x_pos = nbt_blob_lookup_int(&nbt, "Level/xPos").unwrap();
        let z_pos = nbt_blob_lookup_int(&nbt, "Level/zPos").unwrap();
        let global_pos: ChunkCoord = (x_pos.into(), z_pos.into()).into();

        let last_update = nbt_blob_lookup_long(&nbt, "Level/LastUpdate").unwrap();

        let tile_entities = nbt_blob_lookup(&nbt, "Level/TileEntities")
            .unwrap_or_else(|| panic!("Level/TileEntities not found"));
        let block_entities = BlockEntity::map_from_nbt_list(&tile_entities);
        //println!("TileEntities: {:#?}", block_entities);

        let mut blocks = Vec::new();
        let sections = nbt_blob_lookup_list(&nbt, "Level/Sections")
            .unwrap_or_else(|| panic!("Level/Sections not found"));
        for section in sections {
            println!("Y index: {:?}", nbt_value_lookup(&section, "Y"));
            blocks.extend_from_slice(&Chunk::section_to_block_array(&section, &block_entities));
        }

        //println!("{:?}", blocks);
        let x_index = 1;
        let z_index = 1;
        let xz_index = 16 * z_index + x_index;
        let column: Vec<Block> = blocks.iter().skip(xz_index).step_by(256).cloned().collect();
        println!("{:?}", column);

        Self {
            data_version,
            global_pos,
            last_update,
        }
    }

    // TODO Move this function to a more reasonable place. A new file, perhaps?
    // It should be a non-public function. It belongs somewhat here and somewhat to Block.
    fn section_to_block_array(
        section: &nbt::Value,
        block_entities: &HashMap<BlockCoord, BlockEntity>,
    ) -> Vec<Block> {
        const X_LENGTH: i64 = 16;
        const Y_HEIGHT: i64 = 16;
        const Z_LENGTH: i64 = 16;
        // index = (y * X_LENGTH * Z_LENGTH) + (z * X_LENGTH) + x
        fn coordinates(section_y_index: i64, index: usize) -> BlockCoord {
            let y_offset = section_y_index * Y_HEIGHT;
            let y = y_offset + (index as i64) / (X_LENGTH * Z_LENGTH);
            let z = (index as i64) % (X_LENGTH * Z_LENGTH) / X_LENGTH;
            let x = (index as i64) % X_LENGTH;
            //println!("Looking for block entity at ({}, {}, {})", x, y, z);
            (x, y, z).into()
        }

        fn ladder_furnace_chest_facing(data: i8) -> Surface4 {
            match data & 0x7 {
                2 => Surface4::North,
                3 => Surface4::South,
                4 => Surface4::West,
                5 => Surface4::East,
                n => panic!("Unknown facing data variant for chest: {}", n),
            }
        }

        let section_y_index = nbt_value_lookup_byte(&section, "Y").unwrap() as i64;
        let blocks = nbt_value_lookup_byte_array(&section, "Blocks").unwrap();
        let add = packed_nibbles_to_bytes(
            &nbt_value_lookup_byte_array(&section, "Add")
                .unwrap_or_else(|| vec![0; blocks.len() / 2]),
        );
        let data = packed_nibbles_to_bytes(&nbt_value_lookup_byte_array(&section, "Data").unwrap());

        blocks
            .iter()
            .enumerate()
            .map(|(index, block)| (index, ((add[index] as i16) << 8) + *block as i16))
            .map(|(index, block)| {
                match block as u8 {
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
                    5 => match data[index] {
                        0 => Block::Planks {
                            material: WoodMaterial::Oak,
                        },
                        1 => Block::Planks {
                            material: WoodMaterial::Spruce,
                        },
                        2 => Block::Planks {
                            material: WoodMaterial::Birch,
                        },
                        3 => Block::Planks {
                            material: WoodMaterial::Jungle,
                        },
                        4 => Block::Planks {
                            material: WoodMaterial::Acacia,
                        },
                        5 => Block::Planks {
                            material: WoodMaterial::DarkOak,
                        },
                        n => panic!("Unknown plank data variant: {}", n),
                    },
                    6 => Block::Sapling {
                        growth_stage: Int0Through1::new((data[index] & 0x8) >>3).unwrap(),
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
                    //8 => // TODO flowing water, not yet implemented
                    9 => Block::WaterSource,
                    //10 => // TODO flowing lava, not yet implemented
                    11 => Block::LavaSource,
                    12 => match data[index] {
                        0 => Block::Sand,
                        1 => Block::RedSand,
                        n => panic!("Unknown sand data variant: {}", n),
                    },
                    13 => Block::Gravel,
                    14 => Block::GoldOre,
                    15 => Block::IronOre,
                    16 => Block::CoalOre,
                    17 => {
                        let material = match data[index] & 0x3 {
                            0 => WoodMaterial::Oak,
                            1 => WoodMaterial::Spruce,
                            2 => WoodMaterial::Birch,
                            3 => WoodMaterial::Jungle,
                            n => panic!("Impossible log material data: {}", n),
                        };
                        let alignment = match (data[index] & 0xC) >> 2 {
                            0 => Some(Axis3::Y),
                            1 => Some(Axis3::X),
                            2 => Some(Axis3::Z),
                            3 => None,
                            n => panic!("Impossible log alignment data: {}", n),
                        };
                        let stripped = false;
                        Block::Log(block::Log {
                            material,
                            alignment,
                            stripped,
                        })
                    }
                    18 => {
                        let material = match data[index] & 0x3 {
                            0 => LeavesMaterial::Oak,
                            1 => LeavesMaterial::Spruce,
                            2 => LeavesMaterial::Birch,
                            3 => LeavesMaterial::Jungle,
                            n => panic!("Impossible leaves material data: {}", n),
                        };
                        let persistent = (data[index] & 0x4) == 0x4;
                        let distance_to_trunk = None;
                        Block::Leaves {
                            material,
                            distance_to_trunk,
                            persistent,
                        }
                    }
                    19 => match data[index] {
                        0 => Block::Sponge,
                        1 => Block::WetSponge,
                        n => panic!("Unknown sponge data variant: {}", n),
                    },
                    20 => Block::Glass { colour: None },
                    21 => Block::LapisLazuliOre,
                    22 => Block::LapisLazuliBlock,
                    23 => {
                        let facing = match data[index] & 0x7 {
                            0 => Surface6::Down,
                            1 => Surface6::Up,
                            2 => Surface6::North,
                            3 => Surface6::South,
                            4 => Surface6::West,
                            5 => Surface6::East,
                            n => panic!("Unknown surface facing data variant: {}", n),
                        };

                        let coordinates = coordinates(section_y_index, index);
                        let block_entity = block_entities.get(&coordinates).unwrap();

                        match block_entity {
                            BlockEntity::Dispenser { tags } => {
                                Block::Dispenser(Box::new(Dispenser {
                                    facing,
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
                        let coordinates = coordinates(section_y_index, index);
                        let block_entity = block_entities.get(&coordinates).unwrap();

                        if let BlockEntity::Noteblock { note, .. } = block_entity {
                            Block::Noteblock {
                                pitch: note.clone(),
                            }
                        } else {
                            panic!("Wrong block entity variant for note block")
                        }
                    }
                    26 => {
                        let colour = Colour::Red;
                        let facing = match data[index] & 0x3 {
                            0 => Surface4::South,
                            1 => Surface4::West,
                            2 => Surface4::North,
                            3 => Surface4::East,
                            n => panic!("Impossible bed facing data variant: {}", n),
                        };
                        let end = match data[index] & 0x4 {
                            0 => BedEnd::Foot,
                            4 => BedEnd::Head,
                            n => panic!("Impossible bed ending data variant: {}", n),
                        };
                        Block::Bed {
                            colour,
                            facing,
                            end,
                        }
                    }
                    27 => Block::Rail {
                        variant: RailType::Powered,
                        shape: RailShape::from_value(data[index] & 0x7),
                    },
                    28 => Block::Rail {
                        variant: RailType::Detector,
                        shape: RailShape::from_value(data[index] & 0x7),
                    },
                    29 => Block::StickyPiston {
                        facing: match data[index] & 0x7 {
                            0 => Surface6::Down,
                            1 => Surface6::Down,
                            2 => Surface6::Down,
                            3 => Surface6::Down,
                            4 => Surface6::Down,
                            5 => Surface6::Down,
                            n => panic!("Unknown sticky piston facing data variant: {}", n),
                        },
                        extended: data[index] & 0x8 == 0x8,
                    },
                    30 => Block::Cobweb,
                    // NB TODO add parsing of more blocks
                    // (uncertain about data value 31 "tallgrass" types)
                    32 => Block::DeadBush,
                    33 => Block::Piston {
                        facing: match data[index] & 0x7 {
                            0 => Surface6::Down,
                            1 => Surface6::Down,
                            2 => Surface6::Down,
                            3 => Surface6::Down,
                            4 => Surface6::Down,
                            5 => Surface6::Down,
                            n => panic!("Unknown piston facing data variant: {}", n),
                        },
                        extended: data[index] & 0x8 == 0x8,
                    },
                    34 => {
                        let facing = match data[index] & 0x7 {
                            0 => Surface6::Down,
                            1 => Surface6::Down,
                            2 => Surface6::Down,
                            3 => Surface6::Down,
                            4 => Surface6::Down,
                            5 => Surface6::Down,
                            n => panic!("Unknown piston head facing data variant: {}", n),
                        };
                        if data[index] & 0x8 == 0x8 {
                            Block::StickyPistonHead { facing }
                        } else {
                            Block::PistonHead { facing }
                        }
                    }
                    35 => Block::Wool {
                        colour: Some((data[index] as i32).into()),
                    },
                    // NB TODO add parsing of more blocks
                    // (uncertain about data value 36 "Block moved by Piston")
                    37 => Block::Flower(Flower::Dandelion),
                    38 => Block::Flower(match data[index] {
                        0 => Flower::Poppy,
                        1 => Flower::BlueOrchid,
                        2 => Flower::Allium,
                        3 => Flower::AzureBluet,
                        4 => Flower::TulipRed,
                        5 => Flower::TulipOrange,
                        6 => Flower::TulipLightGray,
                        7 => Flower::TulipPink,
                        8 => Flower::OxeyeDaisy,
                        n => panic!("Unkown red flower data variant: {}", n),
                    }),
                    39 => Block::BrownMushroom,
                    40 => Block::RedMushroom,
                    41 => Block::BlockOfGold,
                    42 => Block::BlockOfIron,
                    43 => {
                        let material = match data[index] & 0x7 {
                            0 => SlabMaterial::SmoothStone,
                            1 => SlabMaterial::Sandstone,
                            2 => SlabMaterial::PetrifiedOak, // legacy
                            3 => SlabMaterial::Cobblestone,
                            4 => SlabMaterial::Brick,
                            5 => SlabMaterial::StoneBrick,
                            6 => SlabMaterial::NetherBrick,
                            7 => SlabMaterial::Quartz,
                            n => panic!("Impossible double stone slab data variant: {}", n),
                        };
                        let position = SlabVariant::Double;
                        let waterlogged = false;
                        Block::Slab(Slab {
                            material,
                            position,
                            waterlogged,
                        })
                    }
                    44 => {
                        let material = match data[index] & 0x7 {
                            0 => SlabMaterial::SmoothStone,
                            1 => SlabMaterial::Sandstone,
                            2 => SlabMaterial::PetrifiedOak, // legacy
                            3 => SlabMaterial::Cobblestone,
                            4 => SlabMaterial::Brick,
                            5 => SlabMaterial::StoneBrick,
                            6 => SlabMaterial::NetherBrick,
                            7 => SlabMaterial::Quartz,
                            n => panic!("Impossible stone slab data variant: {}", n),
                        };
                        let position = if (data[index] & 0x8) == 0x8 {
                            SlabVariant::Top
                        } else {
                            SlabVariant::Bottom
                        };
                        let waterlogged = false;
                        Block::Slab(Slab {
                            material,
                            position,
                            waterlogged,
                        })
                    }
                    45 => Block::BrickBlock,
                    46 => Block::TNT,
                    47 => Block::Bookshelf,
                    48 => Block::MossyCobblestone,
                    49 => Block::Obsidian,
                    50 => Block::Torch {
                        attached: match data[index] {
                            1 => Surface5::West,
                            2 => Surface5::East,
                            3 => Surface5::North,
                            4 => Surface5::South,
                            5 => Surface5::Down,
                            n => panic!("Unknown torch data variant: {}", n),
                        },
                    },
                    51 => Block::Fire {
                        age: Int0Through15::new(data[index]).unwrap(),
                    },
                    // NB TODO add parsing of more blocks
                    // 52 mob spawner
                    53 => Block::Stairs(Stair {
                        material: StairMaterial::Oak,
                        position: (data[index] & 0x7).into(),
                        waterlogged: false,
                    }),
                    54 => {
                        let coordinates = coordinates(section_y_index, index);
                        let block_entity = block_entities.get(&coordinates).unwrap();

                        match block_entity {
                            BlockEntity::Chest { tags } => {
                                Block::Chest(Box::new(Chest {
                                    facing: ladder_furnace_chest_facing(data[index]),
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
                        let coordinates = coordinates(section_y_index, index);
                        let block_entity = block_entities.get(&coordinates).unwrap();

                        match block_entity {
                            BlockEntity::Furnace { tags } => {
                                Block::Furnace(Box::new(Furnace {
                                    facing: ladder_furnace_chest_facing(data[index]),
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
                    63 => {
                        let coordinates = coordinates(section_y_index, index);
                        let block_entity = block_entities.get(&coordinates).unwrap();

                        match block_entity {
                            BlockEntity::Sign { colour, text, .. } => {
                                Block::Sign(Box::new(Sign {
                                    material: WoodMaterial::Oak,
                                    placement: WallOrRotatedOnFloor::Floor(
                                        (data[index] & 0xF).into()
                                    ),
                                    waterlogged: false,
                                    colour: colour.clone(),
                                    // TODO something reasonable instead of JSON text
                                    text1: text.get(0).unwrap_or(&String::new()).to_string(),
                                    text2: text.get(1).unwrap_or(&String::new()).to_string(),
                                    text3: text.get(2).unwrap_or(&String::new()).to_string(),
                                    text4: text.get(3).unwrap_or(&String::new()).to_string(),
                                }))
                            }
                            _ => panic!("Wrong block entity variant for standing sign"),
                        }
                    }
                    // NB TODO add parsing of more blocks
                    // 64 oak door
                    // It's complicated:
                    // - some information is in data for the "top" section of the door
                    // - some information is in data for the "bottom" section of the door
                    // - a door may be split between two sections
                    // Conclusion: need to come up with something for doors...
                    65 => Block::Ladder {
                        facing: ladder_furnace_chest_facing(data[index]),
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
                    68 => {
                        let coordinates = coordinates(section_y_index, index);
                        let block_entity = block_entities.get(&coordinates).unwrap();

                        match block_entity {
                            BlockEntity::Sign { colour, text, .. } => {
                                Block::Sign(Box::new(Sign {
                                    material: WoodMaterial::Oak,
                                    placement: WallOrRotatedOnFloor::Wall(
                                        ladder_furnace_chest_facing(data[index]),
                                    ),
                                    waterlogged: false,
                                    colour: colour.clone(),
                                    // TODO something reasonable instead of JSON text
                                    text1: text.get(0).unwrap_or(&String::new()).to_string(),
                                    text2: text.get(1).unwrap_or(&String::new()).to_string(),
                                    text3: text.get(2).unwrap_or(&String::new()).to_string(),
                                    text4: text.get(3).unwrap_or(&String::new()).to_string(),
                                }))
                            }
                            _ => panic!("Wrong block entity variant for wall sign"),
                        }
                    }
                    69 => Block::Lever(
                        match data[index] & 0x7 {
                            // NB these directions are probably wrong...
                            0 => SurfaceRotation12::DownFacingEast,
                            1 => SurfaceRotation12::East,
                            2 => SurfaceRotation12::West,
                            3 => SurfaceRotation12::South,
                            4 => SurfaceRotation12::North,
                            5 => SurfaceRotation12::UpFacingSouth,
                            6 => SurfaceRotation12::UpFacingEast,
                            7 => SurfaceRotation12::DownFacingSouth,
                            n => panic!("Impossible position data variant for lever: {}", n),
                        },
                        if data[index] & 0x8 == 0x8 {
                            OnOffState::On
                        } else {
                            OnOffState::Off
                        }
                    ),
                    70 => Block::PressurePlate {
                        material: PressurePlateMaterial::Stone,
                    },
                    // NB TODO add parsing of more blocks
                    // 71 iron door
                    72 => Block::PressurePlate {
                        material: PressurePlateMaterial::Oak,
                    },
                    73 | 74 => Block::RedstoneOre, // TODO glowing
                    75 | 76 => Block::RedstoneTorch {
                        attached: match data[index] {
                            1 => Surface5::West,
                            2 => Surface5::East,
                            3 => Surface5::North,
                            4 => Surface5::South,
                            5 => Surface5::Down,
                            n => panic!("Unknown redstone torch data variant: {}", n),
                        },
                    },
                    77 => Block::Button(
                        ButtonMaterial::Stone,
                        match data[index] & 0x7 {
                            // NB these directions are probably wrong...
                            0 => SurfaceRotation12::DownFacingEast,
                            1 => SurfaceRotation12::East,
                            2 => SurfaceRotation12::West,
                            3 => SurfaceRotation12::South,
                            4 => SurfaceRotation12::North,
                            5 => SurfaceRotation12::UpFacingSouth,
                            n => panic!("Unknown position data for stone button: {}", n),
                        },
                    ),
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
                        let coordinates = coordinates(section_y_index, index);
                        let block_entity = block_entities.get(&coordinates).unwrap();

                        match block_entity {
                            BlockEntity::Jukebox { record, .. } => {
                                Block::Jukebox(Box::new(Jukebox {
                                    record: record.clone(),
                                }))
                            }
                            _ => panic!("Wrong block entity variant for jukebox"),
                        }
                    },
                    85 => Block::Fence {
                        material: FenceMaterial::Oak,
                        waterlogged: false
                    },
                    86 => Block::Pumpkin {
                        facing: match data[index] & 0x3 {
                            0 => Surface4::South,
                            1 => Surface4::West,
                            2 => Surface4::North,
                            3 => Surface4::East,
                            n => panic!("Impossible facing data for pumpkin: {}", n),
                        },
                    },
                    87 => Block::Netherrack,
                    88 => Block::SoulSand,
                    89 => Block::Glowstone,
                    90 => Block::NetherPortal { alignment: None },
                    91 => Block::JackOLantern {
                        facing: match data[index] & 0x3 {
                            0 => Surface4::South,
                            1 => Surface4::West,
                            2 => Surface4::North,
                            3 => Surface4::East,
                            n => panic!("Impossible facing data for jack o'lantern: {}", n),
                        },
                    },
                    92 => Block::Cake {
                        bites: Int0Through6::new(data[index] & 0x7).unwrap(),
                    },
                    93 | 94 => Block::RedstoneRepeater {
                        facing: match data[index] & 0x3 {
                            0 => Surface4::North,
                            1 => Surface4::East,
                            2 => Surface4::South,
                            3 => Surface4::West,
                            n => panic!("Impossible facing data for redstone repeater: {}", n),
                        },
                        delay: Int1Through4::new(((data[index] >> 2) & 0x3) + 1).unwrap(),
                    },
                    95 => Block::Glass {
                        colour: Some(((data[index] & 0xF) as i32).into()),
                    },
                    // NB TODO add parsing of more blocks
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
                    // NB TODO add parsing of more blocks
                    113 => Block::Fence {
                        material: FenceMaterial::NetherBrick,
                        waterlogged: false
                    },
                    114 => Block::Stairs(Stair {
                        material: StairMaterial::NetherBrick,
                        position: (data[index] & 0x7).into(),
                        waterlogged: false,
                    }),
                    // NB TODO add parsing of more blocks
                    125 => {
                        let material = match data[index] & 0x7 {
                            0 => SlabMaterial::Oak,
                            1 => SlabMaterial::Spruce,
                            2 => SlabMaterial::Birch,
                            3 => SlabMaterial::Jungle,
                            4 => SlabMaterial::Acacia,
                            5 => SlabMaterial::DarkOak,
                            n => panic!("Unknown double wooden slab data variant: {}", n),
                        };
                        let position = SlabVariant::Double;
                        let waterlogged = false;
                        Block::Slab(Slab {
                            material,
                            position,
                            waterlogged,
                        })
                    }
                    126 => {
                        let material = match data[index] & 0x7 {
                            0 => SlabMaterial::Oak,
                            1 => SlabMaterial::Spruce,
                            2 => SlabMaterial::Birch,
                            3 => SlabMaterial::Jungle,
                            4 => SlabMaterial::Acacia,
                            5 => SlabMaterial::DarkOak,
                            n => panic!("Unknown wooden slab data variant: {}", n),
                        };
                        let position = if (data[index] & 0x8) == 0x8 {
                            SlabVariant::Top
                        } else {
                            SlabVariant::Bottom
                        };
                        let waterlogged = false;
                        Block::Slab(Slab {
                            material,
                            position,
                            waterlogged,
                        })
                    }
                    // NB TODO add parsing of more blocks
                    128 => Block::Stairs(Stair {
                        material: StairMaterial::Sandstone,
                        position: (data[index] & 0x7).into(),
                        waterlogged: false,
                    }),
                    // NB TODO add parsing of more blocks
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
                    // NB TODO add parsing of more blocks
                    141 => Block::Carrots {
                        growth_stage: Int0Through7::new(data[index] & 0x7).unwrap(),
                    },
                    142 => Block::Potatoes {
                        growth_stage: Int0Through7::new(data[index] & 0x7).unwrap(),
                    },
                    // NB TODO add parsing of more blocks
                    146 => {
                        let coordinates = coordinates(section_y_index, index);
                        let block_entity = block_entities.get(&coordinates).unwrap();

                        match block_entity {
                            BlockEntity::Chest { tags } => {
                                Block::TrappedChest(Box::new(Chest {
                                    facing: ladder_furnace_chest_facing(data[index]),
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
                    // NB TODO add parsing of more blocks
                    156 => Block::Stairs(Stair {
                        material: StairMaterial::Quartz,
                        position: (data[index] & 0x7).into(),
                        waterlogged: false,
                    }),
                    157 => Block::Rail {
                        variant: RailType::Activator,
                        shape: RailShape::from_value(data[index] & 0x7),
                    },
                    // NB TODO add parsing of more blocks
                    161 => {
                        let material = match data[index] & 0x3 {
                            0 => LeavesMaterial::Acacia,
                            1 => LeavesMaterial::DarkOak,
                            n => panic!("Unknown leaves2 material data: {}", n),
                        };
                        let persistent = (data[index] & 0x4) == 0x4;
                        let distance_to_trunk = None;
                        Block::Leaves {
                            material,
                            distance_to_trunk,
                            persistent,
                        }
                    }
                    162 => {
                        let material = match data[index] & 0x3 {
                            0 => WoodMaterial::Acacia,
                            1 => WoodMaterial::DarkOak,
                            n => panic!("Unknown log2 material data: {}", n),
                        };
                        let alignment = match (data[index] & 0xC) >> 2 {
                            0 => Some(Axis3::Y),
                            1 => Some(Axis3::X),
                            2 => Some(Axis3::Z),
                            3 => None,
                            n => panic!("Impossible log2 alignment data: {}", n),
                        };
                        let stripped = false;
                        Block::Log(block::Log {
                            material,
                            alignment,
                            stripped,
                        })
                    }
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
                    // NB TODO add parsing of more blocks
                    180 => Block::Stairs(Stair {
                        material: StairMaterial::RedSandstone,
                        position: (data[index] & 0x7).into(),
                        waterlogged: false,
                    }),
                    // NB TODO add parsing of more blocks
                    182 => {
                        let material = match data[index] & 0x7 {
                            0 => SlabMaterial::RedSandstone,
                            n => panic!("Unknown stone slab 2 data variant: {}", n),
                        };
                        let position = if (data[index] & 0x8) == 0x8 {
                            SlabVariant::Top
                        } else {
                            SlabVariant::Bottom
                        };
                        let waterlogged = false;
                        Block::Slab(Slab {
                            material,
                            position,
                            waterlogged,
                        })
                    }
                    // NB TODO add parsing of more blocks
                    188 => Block::Fence {
                        material: FenceMaterial::Spruce,
                        waterlogged: false
                    },
                    189 => Block::Fence {
                        material: FenceMaterial::Birch,
                        waterlogged: false
                    },
                    190 => Block::Fence {
                        material: FenceMaterial::Jungle,
                        waterlogged: false
                    },
                    191 => Block::Fence {
                        material: FenceMaterial::DarkOak,
                        waterlogged: false
                    },
                    192 => Block::Fence {
                        material: FenceMaterial::Acacia,
                        waterlogged: false
                    },
                    // NB TODO add parsing of more blocks
                    // 193 spruce door
                    // 194 birch door
                    // 195 jungle door
                    // 196 acacia door
                    // 197 dark oak door
                    203 => Block::Stairs(Stair {
                        material: StairMaterial::Purpur,
                        position: (data[index] & 0x7).into(),
                        waterlogged: false,
                    }),
                    // NB TODO add parsing of more blocks
                    207 => Block::Beetroots {
                        growth_stage: Int0Through3::new(data[index] & 0x3).unwrap(),
                    },
                    // NB TODO add parsing of more blocks
                    n => Block::Unknown(Some(n)),
                }
            })
            .collect()
    }
}

// FIXME there may be something going on with i8 overflow,
// which makes the behaviour different from with u8.
/// Convert byte vector of packed nibbles into byte vector
/// The packing is little endian
fn packed_nibbles_to_bytes(nibbles: &[i8]) -> Vec<i8> {
    nibbles
        .iter()
        .flat_map(|byte| vec![byte & 0x0F, byte >> 4])
        .collect()
}

// FIXME there may be something going on with i8 overflow,
// which makes the behaviour different from with u8.
/// Convert byte vector into byte vector of packed nibbles
/// The packing is little endian
fn bytes_to_packed_nibbles(bytes: &[i8]) -> Vec<i8> {
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
            bytes_to_packed_nibbles(&[0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7]),
            vec![0x10, 0x32, 0x54, 0x76]
        );
    }
}
