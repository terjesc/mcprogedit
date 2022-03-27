use std::collections::HashMap;
use log::warn;

use crate::block::*;
use crate::block_cuboid::BlockCuboid;
use crate::block_entity::{BlockEntity, CommonTags};
use crate::bounded_ints::*;
use crate::chunk::Chunk;
use crate::colour::Colour;
use crate::coordinates::{BlockColumnCoord, BlockCoord, ChunkCoord};
use crate::light_cuboid::LightCuboid;
use crate::material::*;
use crate::nbt_lookup::*;
use crate::positioning::*;
use crate::utils;

impl Chunk {
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
                    let block_coordinates = (block_x, y as i32, block_z);
                    let tile_entity_nbt = match self.blocks.block_at((x, y, z)) {
                        None => None,
                        Some(Block::Banner(banner)) => {
                            banner.to_block_entity(block_coordinates).to_nbt_value()
                        }
                        Some(Block::Barrel(barrel)) => {
                            barrel.to_block_entity(block_coordinates).to_nbt_value()
                        }
                        Some(Block::Beacon(beacon)) => {
                            beacon.to_block_entity(block_coordinates).to_nbt_value()
                        }
                        Some(Block::Bed(_)) => {
                            Some(CommonTags::new_nbt("minecraft:bed", block_coordinates))
                        }
                        Some(Block::Campfire { .. })
                        | Some(Block::SoulCampfire { .. }) => {
                            // TODO Items, CookingTimes and CookingTotalTimes
                            Some(CommonTags::new_nbt("minecraft:campfire", block_coordinates))
                        }
                        Some(Block::Chest(chest)) => {
                            chest.to_block_entity(block_coordinates).to_nbt_value()
                        }
                        Some(Block::Dispenser(dispenser)) => {
                            dispenser.to_block_entity(block_coordinates).to_nbt_value()
                        }
                        Some(Block::Dropper(dropper)) => {
                            dropper.to_block_entity(block_coordinates).to_nbt_value()
                        }
                        Some(Block::EnderChest { .. }) => {
                            Some(CommonTags::new_nbt("minecraft:ender_chest", block_coordinates))
                        }
                        Some(Block::EndPortal) => {
                            Some(CommonTags::new_nbt("minecraft:end_portal", block_coordinates))
                        }
                        Some(Block::Furnace(furnace)) => {
                            furnace.to_block_entity(block_coordinates).to_nbt_value()
                        }
                        Some(Block::Head(head)) => {
                            head.to_block_entity(block_coordinates).to_nbt_value()
                        }
                        Some(Block::Hopper(hopper)) => {
                            hopper.to_block_entity(block_coordinates).to_nbt_value()
                        }
                        Some(Block::Jukebox(jukebox)) => {
                            jukebox.to_block_entity(block_coordinates).to_nbt_value()
                        }
                        Some(Block::Noteblock(noteblock)) => {
                            noteblock.to_block_entity(block_coordinates).to_nbt_value()
                        }
                        Some(Block::ShulkerBox(shulker_box)) => {
                            shulker_box.to_block_entity(block_coordinates).to_nbt_value()
                        }
                        Some(Block::Sign(sign)) => {
                            sign.to_block_entity(block_coordinates).to_nbt_value()
                        }
                        Some(Block::TrappedChest(trapped_chest)) => {
                            trapped_chest.to_trapped_block_entity(block_coordinates).to_nbt_value()
                        }
                        // TODO add handling of other blocks with entities
                        //Some(block) => { println!("Block found: {:?}", block); None },
                        _ => None,
                    };
                    if let Some(value) = tile_entity_nbt {
                        tile_entities.push(value);
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
        // (There might also be need for storing light data in air, extending
        // the required volume by (almost) one section upwards.)
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

                    if let Some(block) = self.blocks.block_at((x as usize, y as usize, z as usize))
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
                            Block::Planks { material } => match material {
                                WoodMaterial::Oak => (5, 0),
                                WoodMaterial::Spruce => (5, 1),
                                WoodMaterial::Birch => (5, 2),
                                WoodMaterial::Jungle => (5, 3),
                                WoodMaterial::Acacia => (5, 4),
                                WoodMaterial::DarkOak => (5, 5),
                                WoodMaterial::Crimson => (5, 0), // Fallback to oak
                                WoodMaterial::Warped => (5, 0),  // Fallback to oak
                            },
                            Block::Sapling {
                                growth_stage,
                                material,
                            } => {
                                let data = (growth_stage.get() as u8) << 3;
                                let data = data
                                    | match material {
                                        SaplingMaterial::Oak => 0,
                                        SaplingMaterial::Spruce => 1,
                                        SaplingMaterial::Birch => 2,
                                        SaplingMaterial::Jungle => 3,
                                        SaplingMaterial::Acacia => 4,
                                        SaplingMaterial::DarkOak => 5,
                                        SaplingMaterial::Bamboo => 0, // Fallback to oak
                                    };
                                (6, data)
                            }
                            Block::Bedrock => (7, 0),
                            Block::WaterSource => (9, 0),
                            Block::Water { falling, level } => {
                                let data = if *falling { 0x8 } else { 0 };
                                let data = data | (8 - (level.get() as u8));
                                (9, data)
                            }
                            Block::LavaSource => (11, 0),
                            Block::Lava { falling, level } => {
                                let data = if *falling { 0x8 } else { 0 };
                                let data = data | (8 - (level.get() as u8));
                                (11, data)
                            }
                            Block::Sand => (12, 0),
                            Block::RedSand => (12, 1),
                            Block::Gravel => (13, 0),
                            Block::GoldOre => (14, 0),
                            Block::IronOre => (15, 0),
                            Block::CoalOre => (16, 0),
                            Block::Log(Log {
                                material,
                                alignment,
                                ..
                            }) => {
                                let data = match alignment {
                                    Axis3::Y => 0,
                                    Axis3::X => 1 << 2,
                                    Axis3::Z => 2 << 2,
                                };
                                match material {
                                    WoodMaterial::Oak => (17, data),
                                    WoodMaterial::Spruce => (17, data | 1),
                                    WoodMaterial::Birch => (17, data | 2),
                                    WoodMaterial::Jungle => (17, data | 3),
                                    WoodMaterial::Acacia => (162, data),
                                    WoodMaterial::DarkOak => (162, data | 1),
                                    WoodMaterial::Crimson => (17, data), // Fallback to oak
                                    WoodMaterial::Warped => (17, data),  // Fallback to oak
                                }
                            }
                            Block::Leaves {
                                material,
                                persistent,
                                ..
                            } => {
                                let data = if *persistent { 0x4 } else { 0 };
                                match material {
                                    LeavesMaterial::Oak => (18, data),
                                    LeavesMaterial::Spruce => (18, data | 1),
                                    LeavesMaterial::Birch => (18, data | 2),
                                    LeavesMaterial::Jungle => (18, data | 3),
                                    LeavesMaterial::Acacia => (161, data),
                                    LeavesMaterial::DarkOak => (161, data | 1),
                                }
                            }
                            Block::Sponge => (19, 0),
                            Block::WetSponge => (19, 1),
                            Block::Glass { colour } => match colour {
                                None => (20, 0),
                                Some(colour) => (95, (*colour as i32) as u8),
                            },
                            Block::LapisLazuliOre => (21, 0),
                            Block::LapisLazuliBlock => (22, 0),
                            Block::Dispenser(dispenser) => (23, facing6_dunswe(&dispenser.facing)),
                            Block::Sandstone => (24, 0),
                            Block::ChiseledSandstone => (24, 1),
                            Block::SmoothSandstone => (24, 2),
                            Block::Noteblock(_) => (25, 0),
                            Block::Bed(bed) => {
                                let end_data = match bed.end {
                                    BedEnd::Head => 0x8,
                                    BedEnd::Foot => 0x0,
                                };
                                (26, facing4_swne(&bed.facing) | end_data)
                            }
                            Block::Rail { variant, shape, .. } => {
                                let shape_data = shape.to_value();
                                match variant {
                                    RailType::Powered => (27, shape_data),
                                    RailType::Detector => (28, shape_data),
                                    RailType::Normal => (66, shape_data),
                                    RailType::Activator => (157, shape_data),
                                }
                            }
                            Block::StickyPiston {
                                facing, extended, ..
                            } => {
                                let extended_data = match extended {
                                    true => 0x8,
                                    false => 0x0,
                                };
                                (29, facing6_dunswe(facing) & extended_data)
                            }
                            Block::Cobweb => (30, 0),
                            Block::Grass(grass) => match grass {
                                Grass::Grass => (31, 1),
                                Grass::Fern => (31, 2),
                                Grass::TallGrassBottom => (175, 2),
                                Grass::TallGrassTop => (175, 8),
                                Grass::LargeFernBottom => (175, 3),
                                Grass::LargeFernTop => (175, 8),
                            },
                            Block::DeadBush => (32, 0),
                            Block::Piston {
                                facing, extended, ..
                            } => {
                                let extended_data = match extended {
                                    true => 0x8,
                                    false => 0x0,
                                };
                                (33, facing6_dunswe(facing) | extended_data)
                            }
                            Block::StickyPistonHead { facing } => {
                                let sticky = 0x8;
                                (34, sticky | facing6_dunswe(facing))
                            }
                            Block::PistonHead { facing } => (34, facing6_dunswe(facing)),
                            Block::Wool { colour } => (35, (*colour).into()),
                            // NB 36 "Block moved by Piston" not implemented
                            Block::Flower(flower) => {
                                match flower {
                                    Flower::Dandelion => (37, 0),
                                    Flower::Poppy => (38, 0),
                                    Flower::BlueOrchid => (38, 1),
                                    Flower::Allium => (38, 2),
                                    Flower::AzureBluet => (38, 3),
                                    Flower::TulipRed => (38, 4),
                                    Flower::TulipOrange => (38, 5),
                                    Flower::TulipWhite => (38, 6),
                                    Flower::TulipPink => (38, 7),
                                    Flower::OxeyeDaisy => (38, 8),
                                    Flower::SunflowerBottom => (175, 0),
                                    Flower::SunflowerTop => (175, 8),
                                    Flower::LilacBottom => (175, 1),
                                    Flower::LilacTop => (175, 8),
                                    Flower::RoseBushBottom => (175, 4),
                                    Flower::RoseBushTop => (175, 8),
                                    Flower::PeonyBottom => (175, 5),
                                    Flower::PeonyTop => (175, 8),
                                    Flower::Cornflower
                                    | Flower::LilyOfTheValley
                                    | Flower::WitherRose => (37, 0), // Fallback to Dandelion
                                }
                            }
                            Block::BrownMushroom => (39, 0),
                            Block::RedMushroom => (40, 0),
                            Block::BlockOfGold => (41, 0),
                            Block::BlockOfIron => (42, 0),
                            Block::Slab(slab) => {
                                let position_data = match slab.position {
                                    SlabVariant::Top => 0x8,
                                    _ => 0x0,
                                };
                                let data = match slab.material {
                                    SlabMaterial::SmoothStone
                                    | SlabMaterial::Oak
                                    | SlabMaterial::RedSandstone
                                    | SlabMaterial::Purpur => position_data,
                                    SlabMaterial::Sandstone | SlabMaterial::Spruce => {
                                        1 | position_data
                                    }
                                    SlabMaterial::PetrifiedOak | SlabMaterial::Birch => {
                                        2 | position_data
                                    }
                                    SlabMaterial::Cobblestone | SlabMaterial::Jungle => {
                                        3 | position_data
                                    }
                                    SlabMaterial::Brick | SlabMaterial::Acacia => 4 | position_data,
                                    SlabMaterial::StoneBrick | SlabMaterial::DarkOak => {
                                        5 | position_data
                                    }
                                    SlabMaterial::NetherBrick => 6 | position_data,
                                    SlabMaterial::Quartz => 7 | position_data,
                                    _ => position_data, // fallback to SmoothStone
                                };
                                let block_id = match slab.material {
                                    SlabMaterial::SmoothStone
                                    | SlabMaterial::Sandstone
                                    | SlabMaterial::PetrifiedOak
                                    | SlabMaterial::Cobblestone
                                    | SlabMaterial::Brick
                                    | SlabMaterial::StoneBrick
                                    | SlabMaterial::NetherBrick
                                    | SlabMaterial::Quartz => match slab.position {
                                        SlabVariant::Double => 43,
                                        _ => 44,
                                    },
                                    SlabMaterial::Spruce
                                    | SlabMaterial::Birch
                                    | SlabMaterial::Jungle
                                    | SlabMaterial::Acacia
                                    | SlabMaterial::DarkOak => match slab.position {
                                        SlabVariant::Double => 125,
                                        _ => 126,
                                    },
                                    SlabMaterial::RedSandstone => match slab.position {
                                        SlabVariant::Double => 181,
                                        _ => 182,
                                    },
                                    SlabMaterial::Purpur => match slab.position {
                                        SlabVariant::Double => 204,
                                        _ => 205,
                                    },
                                    // fallback to SmoothStone
                                    _ => match slab.position {
                                        SlabVariant::Double => 43,
                                        _ => 44,
                                    },
                                };
                                (block_id, data)
                            }
                            Block::BrickBlock => (45, 0),
                            Block::TNT => (46, 0),
                            Block::Bookshelf => (47, 0),
                            Block::MossyCobblestone => (48, 0),
                            Block::Obsidian => (49, 0),
                            Block::Torch { attached } => (50, facing5_xwensd(attached)),
                            Block::Fire { age } => (51, age.get() as u8),
                            // NB 52 mob spawner is not implemented
                            Block::Stairs(stair) => {
                                let data = stair.position.into();
                                match stair.material {
                                    StairMaterial::Oak => (53, data),
                                    StairMaterial::Cobblestone => (67, data),
                                    StairMaterial::Brick => (108, data),
                                    StairMaterial::StoneBrick => (109, data),
                                    StairMaterial::NetherBrick => (114, data),
                                    StairMaterial::Sandstone => (128, data),
                                    StairMaterial::Spruce => (134, data),
                                    StairMaterial::Birch => (135, data),
                                    StairMaterial::Jungle => (136, data),
                                    StairMaterial::Quartz => (156, data),
                                    StairMaterial::Acacia => (163, data),
                                    StairMaterial::DarkOak => (164, data),
                                    StairMaterial::RedSandstone => (180, data),
                                    StairMaterial::Purpur => (203, data),
                                    _ => (53, data), // fallback to oak stairs
                                }
                            }
                            Block::Chest(chest) => (54, facing4_xxnswe(&chest.facing)),
                            Block::RedstoneWire => (55, 0),
                            Block::DiamondOre => (56, 0),
                            Block::BlockOfDiamond => (57, 0),
                            Block::CraftingTable => (58, 0),
                            Block::Wheat { growth_stage } => (59, growth_stage.get() as u8),
                            Block::Farmland { wetness } => (60, wetness.get() as u8),
                            Block::Furnace(furnace) => {
                                let block_id = if furnace.lit { 62 } else { 61 };
                                (block_id, facing4_xxnswe(&furnace.facing))
                            }
                            Block::Sign(sign) => match sign.placement {
                                WallOrRotatedOnFloor::Floor(facing) => (63, facing.into()),
                                WallOrRotatedOnFloor::Wall(facing) => (68, facing4_xxnswe(&facing)),
                            },
                            Block::Door(door) => {
                                let data = match door.half {
                                    DoorHalf::Upper => {
                                        let upper = 0x8;
                                        let hinge = match door.hinged_at {
                                            Hinge::Right => 0x1,
                                            Hinge::Left => 0x0,
                                        };
                                        upper | hinge
                                    }
                                    DoorHalf::Lower => {
                                        let lower = 0x0;
                                        let open = if door.open { 0x4 } else { 0x0 };
                                        let facing = facing4_wnes(&door.facing);
                                        lower | open | facing
                                    }
                                };
                                let block_id = match door.material {
                                    DoorMaterial::Oak => 64,
                                    DoorMaterial::Iron => 71,
                                    DoorMaterial::Spruce => 193,
                                    DoorMaterial::Birch => 194,
                                    DoorMaterial::Jungle => 195,
                                    DoorMaterial::Acacia => 196,
                                    DoorMaterial::DarkOak => 197,
                                    _ => 64, // fallback to oak door
                                };
                                (block_id, data)
                            }
                            Block::Ladder { facing, .. } => (65, facing4_xxnswe(facing)),
                            // 66 normal rail already handled
                            // 67 cobblestone stairs already handled
                            // 68 standing sign already handled
                            Block::Lever(facing, state) => {
                                let state_data = match state {
                                    OnOffState::On => 0x8,
                                    OnOffState::Off => 0x0,
                                };
                                let data = state_data | lever_facing(facing);
                                (69, data)
                            }
                            Block::PressurePlate { material } => {
                                match material {
                                    PressurePlateMaterial::Stone => (70, 0),
                                    PressurePlateMaterial::Oak => (72, 0),
                                    PressurePlateMaterial::Gold => (147, 0),
                                    PressurePlateMaterial::Iron => (148, 0),
                                    _ => (72, 0), // fallback to oak pressure plate
                                }
                            }
                            // 71 iron door already handled
                            // 72 oak pressure plate already handled
                            Block::RedstoneOre => (73, 0),
                            // NB 74 lit redstone ore is not implemented
                            // NB 75 unlit redstone torch is not implemented
                            Block::RedstoneTorch { attached } => (76, facing5_xwensd(attached)),
                            Block::Button(material, facing) => {
                                let data = button_facing(facing);
                                match material {
                                    ButtonMaterial::Stone => (77, data),
                                    ButtonMaterial::Oak => (143, data),
                                    _ => (143, data), // fallback to oak button
                                }
                            }
                            Block::Snow { thickness } => {
                                let data = (thickness.get() as u8) + 1;
                                (78, data)
                            }
                            Block::Ice => (79, 0),
                            Block::SnowBlock => (80, 0),
                            Block::Cactus { growth_stage } => (81, growth_stage.get() as u8),
                            Block::Clay => (82, 0),
                            Block::SugarCane { growth_stage } => (83, growth_stage.get() as u8),
                            Block::Jukebox(_) => (84, 0),
                            Block::Fence { material, .. } => {
                                match material {
                                    FenceMaterial::Oak => (85, 0),
                                    FenceMaterial::NetherBrick => (113, 0),
                                    FenceMaterial::Spruce => (188, 0),
                                    FenceMaterial::Birch => (189, 0),
                                    FenceMaterial::Jungle => (190, 0),
                                    FenceMaterial::DarkOak => (191, 0),
                                    FenceMaterial::Acacia => (192, 0),
                                    _ => (85, 0), // fallback to oak fence
                                }
                            }
                            Block::CarvedPumpkin { facing } => (86, facing4_swne(facing)),
                            Block::Netherrack => (87, 0),
                            Block::SoulSand => (88, 0),
                            Block::Glowstone => (89, 0),
                            Block::NetherPortal { .. } => (90, 0),
                            Block::JackOLantern { facing } => (91, facing4_swne(facing)),
                            Block::Cake { pieces } => (92, 7 - (pieces.get() as u8)),
                            Block::RedstoneRepeater(repeater) => {
                                let delay_data = (repeater.delay.get() as u8) << 2;
                                let facing_data = facing4_nesw(&repeater.facing);
                                (93, delay_data | facing_data)
                            }
                            // NB 94 powered redstone repeater is not implemented
                            //       (may be added to Block::RedstoneRepeater in the future)
                            // 95 coloured class already handled
                            Block::Trapdoor(trapdoor) => {
                                let open_data = if trapdoor.open { 0x4 } else { 0x0 };
                                let hinge_data = trapdoor_hinge_at(&trapdoor.hinge_at);
                                let data = open_data | hinge_data;
                                match trapdoor.material {
                                    DoorMaterial::Oak => (96, data),
                                    DoorMaterial::Iron => (167, data),
                                    _ => (96, data), // fallback to oak trapdoor
                                }
                            }
                            Block::InfestedStone => (97, 0),
                            Block::InfestedCobblestone => (97, 1),
                            Block::InfestedStoneBricks => (97, 2),
                            Block::InfestedMossyStoneBricks => (97, 3),
                            Block::InfestedCrackedStoneBricks => (97, 4),
                            Block::InfestedChiseledStoneBricks => (97, 5),
                            Block::StoneBricks => (98, 0),
                            Block::MossyStoneBricks => (98, 1),
                            Block::CrackedStoneBricks => (98, 2),
                            Block::ChiseledStoneBricks => (98, 3),
                            // MushroomStem is non-coloured in later versions,
                            // and always considered to be brown for early version file export
                            Block::MushroomStem { stem_directions } => {
                                (99, mushroom_stems(stem_directions))
                            }
                            Block::BrownMushroomBlock { cap_directions } => {
                                (99, mushroom_caps(cap_directions))
                            }
                            Block::RedMushroomBlock { cap_directions } => {
                                (100, mushroom_caps(cap_directions))
                            }
                            Block::IronBars { .. } => (101, 0),
                            Block::GlassPane { colour, .. } => match colour {
                                None => (102, 0),
                                Some(colour) => (160, (*colour as i32) as u8),
                            },
                            Block::Melon => (103, 0),
                            Block::PumpkinStem { state } => match state {
                                StemState::Growing(age) => (104, age.get() as u8),
                                _ => (104, 7), // fallback to fully grown stem (not attached)
                            },
                            Block::MelonStem { state } => match state {
                                StemState::Growing(age) => (105, age.get() as u8),
                                _ => (105, 7), // fallback to fully grown stem (not attached)
                            },
                            Block::Vines(vines) => {
                                let mut data = if vines.anchored_at.east { 0x8 } else { 0x0 };
                                data |= if vines.anchored_at.north { 0x4 } else { 0x0 };
                                data |= if vines.anchored_at.south { 0x1 } else { 0x0 };
                                data |= if vines.anchored_at.west { 0x2 } else { 0x0 };
                                (106, data)
                            }
                            Block::FenceGate {
                                facing,
                                open,
                                material,
                            } => {
                                let facing_data = facing4_swne(facing);
                                let open_data = if *open { 0x4 } else { 0x0 };
                                let data = facing_data | open_data;
                                match material {
                                    WoodMaterial::Oak => (107, data),
                                    WoodMaterial::Spruce => (183, data),
                                    WoodMaterial::Birch => (184, data),
                                    WoodMaterial::Jungle => (185, data),
                                    WoodMaterial::DarkOak => (186, data),
                                    WoodMaterial::Acacia => (187, data),
                                    _ => (107, data), // fallback to oak fence gate
                                }
                            }
                            // 108 and 109  brick and stone brick stairs already handled
                            Block::Mycelium => (110, 0),
                            Block::LilyPad => (111, 0),
                            Block::NetherBricks => (112, 0),
                            // 113 nether brick fence already handled
                            // 114 nether brick stairs already handled
                            Block::NetherWart { growth_stage } => (115, growth_stage.get() as u8),
                            Block::EnchantingTable { .. } => (116, 0),
                            Block::BrewingStand { .. } => (117, 0),
                            Block::Cauldron { water_level } => (118, water_level.get() as u8),
                            Block::EndPortal => (119, 0),
                            Block::EndPortalFrame { facing, has_eye } => {
                                let facing_data = facing4_swne(facing);
                                let has_eye_data = if *has_eye { 0x4 } else { 0x0 };
                                (120, facing_data | has_eye_data)
                            }
                            Block::EndStone => (121, 0),
                            Block::DragonEgg => (122, 0),
                            Block::RedstoneLamp => (123, 0),
                            // NB 124 lit redstone lamp is not implemented
                            // 125 and 126 wooden slabs already handled
                            Block::Cocoa {
                                growth_stage,
                                facing,
                            } => {
                                let growth_data = (growth_stage.get() as u8) << 2;
                                let facing_data = facing4_nesw(facing);
                                (127, growth_data | facing_data)
                            }
                            // 128 sandstone stairs already handled
                            Block::EmeraldOre => (129, 0),
                            Block::EnderChest { facing, .. } => (130, facing4_xxnswe(facing)),
                            Block::TripwireHook { facing } => (131, facing4_swne(facing)),
                            Block::Tripwire => (132, 0),
                            Block::BlockOfEmerald => (133, 0),
                            // 134-136 spruce / birch / jungle stairs already handled
                            // NB 137 command block is not implemented
                            Block::Beacon(_) => (138, 0),
                            Block::Wall { material, .. } => match material {
                                WallMaterial::Cobblestone => (139, 0),
                                WallMaterial::MossyCobblestone => (139, 1),
                                _ => (139, 0), // fallback to cobblestone wall
                            },
                            Block::FlowerPot(_) => (140, 0),
                            Block::Carrots { growth_stage } => (141, growth_stage.get() as u8),
                            Block::Potatoes { growth_stage } => (142, growth_stage.get() as u8),
                            // 143 oak button already handled
                            Block::Head(head) => {
                                let data = match head.placement {
                                    WallOrRotatedOnFloor::Floor(_) => 1,
                                    WallOrRotatedOnFloor::Wall(Surface4::North) => 2,
                                    WallOrRotatedOnFloor::Wall(Surface4::South) => 3,
                                    WallOrRotatedOnFloor::Wall(Surface4::West) => 4,
                                    WallOrRotatedOnFloor::Wall(Surface4::East) => 5,
                                };
                                (144, data)
                            }
                            Block::Anvil { facing, damage } => {
                                let facing_data = facing4_swne(facing);
                                let damage_data = match damage {
                                    AnvilDamage::Intact => 0b0000,
                                    AnvilDamage::SlightlyDamaged => 0b0100,
                                    AnvilDamage::VeryDamaged => 0b1000,
                                };
                                (145, facing_data | damage_data)
                            }
                            Block::TrappedChest(chest) => (146, facing4_xxnswe(&chest.facing)),
                            // 147 and 148 gold / iron pressure plate already handled
                            Block::RedstoneComparator { facing } => (149, facing4_nesw(facing)),
                            Block::RedstoneSubtractor { facing } => {
                                (149, 0x4 | facing4_nesw(facing))
                            }
                            // NB 150 powered redstone comparator is not implemented
                            Block::DaylightDetector => (151, 0),
                            Block::BlockOfRedstone => (152, 0),
                            Block::QuartzOre => (153, 0),
                            Block::Hopper(hopper) => (154, facing5_dxnswe(&hopper.facing)),
                            Block::BlockOfQuartz => (155, 0),
                            Block::ChiseledQuartzBlock => (155, 1),
                            Block::QuartzPillar { alignment } => match alignment {
                                Axis3::Y => (155, 2),
                                Axis3::X => (155, 3),
                                Axis3::Z => (155, 4),
                            },
                            // 156 quartz stairs already handled
                            // 157 activator rail already handled
                            Block::Dropper(dropper) => (158, facing6_dunswe(&dropper.facing)),
                            Block::Terracotta { colour } => match colour {
                                Some(colour) => (159, (*colour).into()),
                                None => (172, 0),
                            },
                            // 160 coloured glass pane already handled
                            // 161 acacia / dark oak leaves already handled
                            // 162 acacia / dark oak logs already handled
                            // 163 and 164 acacia / dark oak stairs already handled
                            Block::BlockOfSlime => (165, 0),
                            Block::Barrier => (166, 0),
                            // 167 iron trapdoor already handled
                            Block::Prismarine => (168, 0),
                            Block::PrismarineBricks => (168, 1),
                            Block::DarkPrismarine => (168, 2),
                            Block::SeaLantern => (169, 0),
                            Block::HayBale { alignment } => match alignment {
                                Axis3::Y => (170, 0),
                                Axis3::X => (170, 4),
                                Axis3::Z => (170, 8),
                            },
                            Block::Carpet { colour } => (171, *colour as u8),
                            // 172 terracotta (no colour) already handled
                            Block::BlockOfCoal => (173, 0),
                            Block::PackedIce => (174, 0),
                            // 175 double tall plants already handled
                            Block::Banner(banner) => match banner.placement {
                                WallOrRotatedOnFloor::Floor(facing) => (176, facing as u8),
                                WallOrRotatedOnFloor::Wall(facing) => {
                                    (177, facing4_xxnswe(&facing))
                                }
                            },
                            Block::InvertedDaylightDetector => (178, 0),
                            Block::RedSandstone => (179, 0),
                            Block::ChiseledRedSandstone => (179, 1),
                            Block::SmoothRedSandstone => (179, 2),
                            // 180 red sandstone stairs already handled
                            // 181 and 182 red sandstone slabs already handled
                            // 183-187 spruce / birch / jungle / acacia / dark oak fence gates
                            //     already handled
                            // 188-192 spruce / birch / jungle / acacia / dark oak fences
                            //     already handled
                            // 193-197 spruce / birch / jungle / acacia / dark oak doors
                            //     already handled
                            Block::EndRod { facing } => (198, facing6_dunswe(facing)),
                            Block::ChorusPlant => (199, 0),
                            Block::ChorusFlower { growth_stage } => (200, growth_stage.get() as u8),
                            Block::PurpurBlock => (201, 0),
                            Block::PurpurPillar { alignment } => match alignment {
                                Axis3::Y => (202, 0),
                                Axis3::X => (202, 1),
                                Axis3::Z => (202, 2),
                            },
                            // 203 purpur stairs already handled
                            // 204 and 205 purpur slabs already handled
                            Block::EndStoneBricks => (206, 0),
                            Block::Beetroots { growth_stage } => (207, growth_stage.get() as u8),
                            Block::GrassPath => (208, 0),
                            Block::EndGateway => (209, 0),
                            // NB 210 repeating command block is not implemented
                            // NB 211 chain command block is not implemented
                            Block::FrostedIce => (212, 0),
                            Block::MagmaBlock => (213, 0),
                            Block::NetherWartBlock => (214, 0),
                            Block::RedNetherBricks => (215, 0),
                            Block::BoneBlock { alignment } => match alignment {
                                Axis3::Y => (216, 0),
                                Axis3::X => (216, 4),
                                Axis3::Z => (216, 8),
                            },
                            Block::StructureVoid => (217, 0),
                            Block::Observer { facing } => (218, facing6_dunswe(facing)),
                            Block::ShulkerBox(shulker_box) => {
                                let colour = shulker_box.colour.unwrap_or(Colour::Purple);
                                let block_id = (colour as u8) + 219;
                                (block_id, 0)
                            }
                            Block::GlazedTerracotta(glazed_terracotta) => {
                                let block_id = (glazed_terracotta.colour as u8) + 235;
                                (block_id, 0)
                            }
                            Block::Concrete { colour } => (251, *colour as u8),
                            Block::ConcretePowder { colour } => (252, *colour as u8),
                            // NB 255 structure block is not implemented
                            _ => (0, 0), // fallback to air
                        };

                        blocks[index] = block_id;
                        utils::set_nibble(&mut data, data_value, index);
                    }
                }
            }
        }

        // TODO Somehow fill block light and sky light with reasonable values...
        let block_light = vec![0xFFu8; 2048];
        let sky_light = vec![0xFFu8; 2048];

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

        return nbt::Value::Compound(section);

        fn facing4_nesw(facing: &Surface4) -> u8 {
            match facing {
                Surface4::North => 0,
                Surface4::East => 1,
                Surface4::South => 2,
                Surface4::West => 3,
            }
        }

        fn facing4_swne(facing: &Surface4) -> u8 {
            match facing {
                Surface4::South => 0,
                Surface4::West => 1,
                Surface4::North => 2,
                Surface4::East => 3,
            }
        }

        fn facing4_wnes(facing: &Surface4) -> u8 {
            match facing {
                Surface4::West => 0,
                Surface4::North => 1,
                Surface4::East => 2,
                Surface4::South => 3,
            }
        }

        fn facing4_xxnswe(facing: &Surface4) -> u8 {
            match facing {
                Surface4::North => 2,
                Surface4::South => 3,
                Surface4::West => 4,
                Surface4::East => 5,
            }
        }

        fn facing5_dxnswe(facing: &Surface5) -> u8 {
            match facing {
                Surface5::Down => 0,
                Surface5::North => 2,
                Surface5::South => 3,
                Surface5::West => 4,
                Surface5::East => 5,
            }
        }

        fn facing5_xwensd(facing: &Surface5) -> u8 {
            match facing {
                Surface5::West => 1,
                Surface5::East => 2,
                Surface5::North => 3,
                Surface5::South => 4,
                Surface5::Down => 5,
            }
        }

        fn facing6_dunswe(facing: &Surface6) -> u8 {
            match facing {
                Surface6::Down => 0,
                Surface6::Up => 1,
                Surface6::North => 2,
                Surface6::South => 3,
                Surface6::West => 4,
                Surface6::East => 5,
            }
        }

        fn lever_facing(facing: &SurfaceRotation12) -> u8 {
            match facing {
                SurfaceRotation12::DownFacingEast => 0,
                SurfaceRotation12::East => 1,
                SurfaceRotation12::West => 2,
                SurfaceRotation12::South => 3,
                SurfaceRotation12::North => 4,
                SurfaceRotation12::UpFacingSouth => 5,
                SurfaceRotation12::UpFacingEast => 6,
                SurfaceRotation12::DownFacingSouth => 7,
                facing => panic!("Unknown lever facing: {:?}", facing),
            }
        }

        // NB This value mapping needs checking
        fn button_facing(facing: &SurfaceRotation12) -> u8 {
            match facing {
                SurfaceRotation12::DownFacingNorth
                | SurfaceRotation12::DownFacingSouth
                | SurfaceRotation12::DownFacingEast
                | SurfaceRotation12::DownFacingWest => 0,
                SurfaceRotation12::East => 1,
                SurfaceRotation12::West => 2,
                SurfaceRotation12::South => 3,
                SurfaceRotation12::North => 4,
                SurfaceRotation12::UpFacingNorth
                | SurfaceRotation12::UpFacingSouth
                | SurfaceRotation12::UpFacingEast
                | SurfaceRotation12::UpFacingWest => 5,
            }
        }

        fn mushroom_caps(caps: &DirectionFlags6) -> u8 {
            match (
                caps.north, caps.south, caps.east, caps.west, caps.up, caps.down,
            ) {
                //north south  east   west   top    bottom
                (false, false, false, false, false, false) => 0, // co caps (all pores)
                (true, false, false, true, _, false) => 1, // north west top
                (true, false, false, false, _, false) => 2, // north top
                (true, false, true, false, _, false) => 3, // north east top
                (false, false, false, true, _, false) => 4, // west top
                (false, false, false, false, true, false) => 5, // top
                (false, false, true, false, _, false) => 6, // east top
                (false, true, false, true, _, false) => 7, // south west top
                (false, true, false, false, _, false) => 8, // south top
                (false, true, true, false, _, false) => 9, // south east top
                (_, _, _, _, _, true) => 14, // all sides (only way to get bottom)
                _ => 14, // final fallback to all caps; could perhaps be improved
            }
        }

        fn mushroom_stems(stems: &DirectionFlags6) -> u8 {
            match (
                stems.north,
                stems.south,
                stems.east,
                stems.west,
                stems.up,
                stems.down,
            ) {
                //north south  east   west   top    bottom
                (false, false, false, false, false, false) => 0, // no stem (all pores)
                (true, true, true, true, false, false) => 10, // north south east west
                (true, true, true, true, true, true) => 15, // all sides
                // fallbacks
                (_, _, _, _, _, true) => 15, // bottom only available through pattern 15
                (_, _, _, _, true, _) => 15, // top only available through pattern 15
                (_, _, _, _, false, false) => 10, // best pattern for any side combo
                //_ => 0, // fallback to all pores; could perhaps be improved
            }
        }

        fn trapdoor_hinge_at(facing: &Edge8) -> u8 {
            match facing {
                Edge8::DownSouth => 0,
                Edge8::DownNorth => 1,
                Edge8::DownEast => 2,
                Edge8::DownWest => 3,
                Edge8::UpSouth => 8,
                Edge8::UpNorth => 9,
                Edge8::UpEast => 10,
                Edge8::UpWest => 11,
            }
        }
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
        let section_y_index = nbt_value_lookup_byte(section, "Y").unwrap() as i64;
        let blocks = nbt_value_lookup_byte_array(section, "Blocks").unwrap();
        let add = utils::packed_nibbles_to_bytes(
            &nbt_value_lookup_byte_array(section, "Add")
                .unwrap_or_else(|_| vec![0; blocks.len() / 2]),
        );
        let data = utils::packed_nibbles_to_bytes(&nbt_value_lookup_byte_array(section, "Data").unwrap());

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
    pub(crate) fn pre_flattening_fill_block_cuboid_from_section(
        section: &nbt::Value,
        block_entities: &HashMap<BlockCoord, BlockEntity>,
        chunk_position: &ChunkCoord,
        block_cuboid: &mut BlockCuboid,
    ) {
        let xz_offset: BlockCoord = chunk_position.into();
        let section_y_index = nbt_value_lookup_byte(section, "Y").unwrap() as i64;
        let blocks = nbt_value_lookup_byte_array(section, "Blocks").unwrap();
        let add = utils::packed_nibbles_to_bytes(
            &nbt_value_lookup_byte_array(section, "Add")
                .unwrap_or_else(|_| vec![0; blocks.len() / 2]),
        );
        let data = utils::packed_nibbles_to_bytes(&nbt_value_lookup_byte_array(section, "Data").unwrap());

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
                            bark_on_all_sides: false,
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
                                    pitch: *note,
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
                            n => {
                                eprintln!(
                                    "[warning] unknown red flower data variant {:?} at index {:?}",
                                    n,
                                    index,
                                    // TODO give info about coordinates?
                                );
                                Flower::Poppy
                            },
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
                        77 => Block::Button(ButtonMaterial::Stone, button_facing(data[index])),
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
                        86 => Block::CarvedPumpkin {
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
                        99 => match data[index] {
                            // Stems are separate blocks in the internal representation,
                            // as well as in later versions of the save format
                            stem @ 10 | stem @ 15 => Block::MushroomStem {
                                stem_directions: mushroom_caps(stem),
                            },
                            cap => Block::BrownMushroomBlock {
                                cap_directions: mushroom_caps(cap),
                            },
                        },
                        100 => match data[index] {
                            // Stems are separate blocks in the internal representation,
                            // as well as in later versions of the save format
                            stem @ 10 | stem @ 15 => Block::MushroomStem {
                                stem_directions: mushroom_caps(stem),
                            },
                            cap => Block::RedMushroomBlock {
                                cap_directions: mushroom_caps(cap),
                            },
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
                            anchored_at: DirectionFlags5 {
                                east: data[index] & 0x8 == 0x8,
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
                        118 => Block::Cauldron {
                            water_level: Int0Through3::new(data[index] & 0x3).unwrap(),
                        },
                        119 => Block::EndPortal,
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
                        127 => Block::Cocoa {
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
                                    primary: *primary,
                                    secondary: *secondary,
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
                        143 => Block::Button(ButtonMaterial::Oak, button_facing(data[index])),
                        144 => {
                            let coordinates = Self::coordinates(section_y_index, xz_offset, index);
                            let block_entity = block_entities.get(&coordinates).unwrap();

                            match block_entity {
                                BlockEntity::Skull { skull_type, facing, .. } => {
                                    let facing = facing.unwrap_or(Direction16::default());
                                    let skull_type = skull_type.unwrap_or(HeadVariant::default());
                                    Block::Head(Head {
                                        variant: skull_type,
                                        placement: match data[index] {
                                            1 => WallOrRotatedOnFloor::Floor(facing),
                                            2 => WallOrRotatedOnFloor::Wall(Surface4::North),
                                            3 => WallOrRotatedOnFloor::Wall(Surface4::South),
                                            4 => WallOrRotatedOnFloor::Wall(Surface4::West),
                                            5 => WallOrRotatedOnFloor::Wall(Surface4::East),
                                            n => {
                                                warn!("Unknown data value for skull: {}", n);
                                                WallOrRotatedOnFloor::Floor(Direction16::default())
                                            }
                                        },
                                        waterlogged: false,
                                    })
                                }
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
                            bark_on_all_sides: false,
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
                            match block_entities.get(&entity_coordinates) {
                                Some(BlockEntity::PseudoFlowerBottom(bottom_flower)) => {
                                    if (data[index] & 0x8) == 0x8 {
                                        let top_flower = match bottom_flower {
                                            Flower::LilacBottom => Flower::LilacTop,
                                            Flower::PeonyBottom => Flower::PeonyTop,
                                            Flower::RoseBushBottom => Flower::RoseBushTop,
                                            Flower::SunflowerBottom => Flower::SunflowerTop,
                                            variant => panic!(
                                                "Unexpected flower variant for bottom flower: {:?}",
                                                variant,
                                            ),
                                        };
                                        Block::Flower(top_flower)
                                    } else {
                                        Block::Flower(*bottom_flower)
                                    }
                                }
                                Some(BlockEntity::PseudoGrassBottom(bottom_grass)) => {
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
                                        Block::Grass(*bottom_grass)
                                    }
                                }
                                Some(_) => {
                                    eprintln!(
                                        "Wrong block entity variant for flower or grass, at {:?}",
                                        coordinates,
                                    );
                                    Block::Air
                                }
                                None => {
                                    eprintln!(
                                        "Missing block entity for flower or grass, at {:?}",
                                        coordinates,
                                    );
                                    Block::Air
                                }
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
                        199 => Block::ChorusPlant,
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

        fn wood_alignment(data: i8) -> Axis3 {
            match (data & 0xC) >> 2 {
                0 => Axis3::Y,
                1 => Axis3::X,
                2 => Axis3::Z,
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

        // NB This value mapping needs checking
        fn button_facing(data: i8) -> SurfaceRotation12 {
            match data & 0x7 {
                0 => SurfaceRotation12::DownFacingNorth,
                1 => SurfaceRotation12::East,
                2 => SurfaceRotation12::West,
                3 => SurfaceRotation12::South,
                4 => SurfaceRotation12::North,
                5 => SurfaceRotation12::UpFacingNorth,
                n @ 6..=7 => panic!("Unknown button facing value: {}", n),
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

    pub(crate) fn pre_flattening_fill_light_cuboids_from_section(
        section: &nbt::Value,
        block_light: &mut LightCuboid,
        sky_light: &mut LightCuboid,
    ) {
        // Parse relevant NBT data
        let section_y_index = nbt_value_lookup_byte(section, "Y").unwrap() as i64;
        let section_block_light = utils::packed_nibbles_to_bytes(
            &nbt_value_lookup_byte_array(section, "BlockLight")
                .unwrap_or_else(|_| vec![0; 2048]),
        );
        let section_sky_light = utils::packed_nibbles_to_bytes(
            &nbt_value_lookup_byte_array(section, "SkyLight")
                .unwrap_or_else(|_| vec![0; 2048]),
        );
        let local_xz_offset = BlockCoord(0, 0, 0);

        // Fill relevant areas of block_light and sky_light
        for (index, value) in section_block_light.iter().enumerate() {
            let coordinates = Self::coordinates(section_y_index, local_xz_offset, index);
            block_light.set_light_level_at(coordinates, *value as u8);
        }
        for (index, value) in section_sky_light.iter().enumerate() {
            let coordinates = Self::coordinates(section_y_index, local_xz_offset, index);
            sky_light.set_light_level_at(coordinates, *value as u8);
        }
    }
}

