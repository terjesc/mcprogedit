use log::warn;
use nbt::Value;

use crate::block::*;
use crate::bounded_ints::*;
use crate::colour::Colour;
use crate::material::*;
use crate::nbt_lookup::*;
use crate::positioning::*;

/// Structure for storing palette data for blocks that also have parts of its data stored in
/// a block entity. Those blocks have individual block entities, but share the data stored
/// in the palette. When parsing the Palette the shared data is stored in this "proto block".
/// Later, when parsing BlockStates, the proto block gets combined with the block entities to
/// form the actual blocks.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) enum ProtoBlock {
    Banner { colour: Colour, placement: WallOrRotatedOnFloor },
    Barrel { facing: Surface6 },
    Beacon,
    BrewingStand,
    Chest { facing: Surface4, variant: Option<ChestVariant>, waterlogged: bool },
    Dispenser { facing: Surface6 },
    Dropper { facing: Surface6 },
    EnchantingTable,
    Furnace { facing: Surface4, lit: bool },
    Hopper { facing: Surface5 },
    Jukebox,
    ShulkerBox { colour: Option<Colour>, facing: Surface6 },
    Sign { material: WoodMaterial, placement: WallOrRotatedOnFloor, waterlogged: bool },
    TrappedChest { facing: Surface4, variant: Option<ChestVariant>, waterlogged: bool },
}

/// The palette will contain some fully parsed blocks, and some blocks for which the block
/// entity has not been matched yet. The latter contain only information from the Palette
/// NBT value, and are named "proto blocks". They will later be combined with block entities
/// to form full blocks.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) enum PaletteItem {
    Block(Block),
    ProtoBlock(ProtoBlock),
}

impl PaletteItem {
    /// Convert from Block to PaletteItem.
    pub(super) fn from_block(block: &Block) -> Self {
        match block {
            Block::Banner(banner) => proto(ProtoBlock::Banner {
                colour: banner.colour,
                placement: banner.placement,
            }),
            Block::Beacon(_) => proto(ProtoBlock::Beacon),
            Block::Barrel(barrel) => proto(ProtoBlock::Barrel {
                facing: barrel.facing,
            }),
            Block::BrewingStand(_) => proto(ProtoBlock::BrewingStand),
            Block::Chest(chest) => proto(ProtoBlock::Chest {
                facing: chest.facing,
                variant: chest.variant,
                waterlogged: chest.waterlogged,
            }),
            Block::Dispenser(dispenser) => proto(ProtoBlock::Dispenser {
                facing: dispenser.facing
            }),
            Block::Dropper(dropper) => proto(ProtoBlock::Dropper {
                facing: dropper.facing
            }),
            Block::EnchantingTable { .. } => proto(ProtoBlock::EnchantingTable),
            Block::Furnace(furnace) => proto(ProtoBlock::Furnace {
                facing: furnace.facing,
                lit: furnace.lit,
            }),
            Block::Hopper(hopper) => proto(ProtoBlock::Hopper {
                facing: hopper.facing,
            }),
            // NB Jukebox has a ProtoBlock, but it is only used for import
            Block::ShulkerBox(shulker_box) => proto(ProtoBlock::ShulkerBox {
                colour: shulker_box.colour,
                facing: shulker_box.facing,
            }),
            Block::Sign(sign) => proto(ProtoBlock::Sign {
                material: sign.material,
                placement: sign.placement,
                waterlogged: sign.waterlogged,
            }),
            Block::TrappedChest(trapped_chest) => proto(ProtoBlock::TrappedChest {
                facing: trapped_chest.facing,
                variant: trapped_chest.variant,
                waterlogged: trapped_chest.waterlogged,
            }),
            _ => PaletteItem::Block(block.clone()),
        }
    }

    pub(super) fn to_nbt_value(&self) -> nbt::Value {
        let mut palette_item = nbt::Map::new();

        palette_item.insert("Name".into(), nbt::Value::String(self.name().into()));

        match self.properties() {
            Some(properties) => { palette_item.insert("Properties".into(), properties); }
            None => (),
        }

        nbt::Value::Compound(palette_item)
    }

    fn properties(&self) -> Option<nbt::Value> {
        match self {
            PaletteItem::Block(Block::Sapling { growth_stage, material }) => {
                if *material == SaplingMaterial::Bamboo {
                    None
                } else {
                    let mut properties = nbt::Map::new();
                    properties.insert("stage".into(), nbt::Value::String(growth_stage.to_string()));
                    Some(nbt::Value::Compound(properties))
                }
            }
            PaletteItem::Block(Block::Basalt { alignment })
            | PaletteItem::Block(Block::BoneBlock { alignment })
            | PaletteItem::Block(Block::Chain { alignment })
            | PaletteItem::Block(Block::Log(Log { alignment, .. }))
            | PaletteItem::Block(Block::HayBale { alignment })
            | PaletteItem::Block(Block::PolishedBasalt { alignment })
            | PaletteItem::Block(Block::PurpurPillar { alignment })
            | PaletteItem::Block(Block::QuartzPillar { alignment }) => {
                let mut properties = nbt::Map::new();
                properties.insert("axis".into(), nbt::Value::String(alignment.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::WaterSource)
            | PaletteItem::Block(Block::LavaSource) => {
                let mut properties = nbt::Map::new();
                properties.insert("level".into(), nbt::Value::String("0".into()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Water { falling, level })
            | PaletteItem::Block(Block::Lava { falling, level }) => {
                let mut properties = nbt::Map::new();
                let level = 8 - i8::from(*level);
                let level = if *falling { level | 0x8 } else { level };
                properties.insert("level".into(), nbt::Value::String(level.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Leaves { distance_to_trunk, persistent, .. }) => {
                let mut properties = nbt::Map::new();
                if let Some(distance) = distance_to_trunk {
                    properties.insert("distance".into(), nbt::Value::String(distance.to_string()));
                }
                let persistent = if *persistent { "true".into() } else { "false".into() };
                properties.insert("persistent".into(), nbt::Value::String(persistent));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::ProtoBlock(ProtoBlock::Dispenser { facing })
            | PaletteItem::ProtoBlock(ProtoBlock::Dropper { facing }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Noteblock(Noteblock { pitch })) => {
                let mut properties = nbt::Map::new();
                properties.insert("note".into(), nbt::Value::String(pitch.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Rail { shape, .. }) => {
                let mut properties = nbt::Map::new();
                properties.insert("shape".into(), nbt::Value::String(shape.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::StickyPiston { facing, extended }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                properties.insert("extended".into(), nbt::Value::String(extended.to_string()));
                properties.insert("type".into(), nbt::Value::String("sticky".into()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Piston { facing, extended }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                properties.insert("extended".into(), nbt::Value::String(extended.to_string()));
                properties.insert("type".into(), nbt::Value::String("normal".into()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::StickyPistonHead { facing }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                properties.insert("short".into(), nbt::Value::String("false".into()));
                properties.insert("type".into(), nbt::Value::String("sticky".into()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::PistonHead { facing }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                properties.insert("short".into(), nbt::Value::String("false".into()));
                properties.insert("type".into(), nbt::Value::String("normal".into()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Grass(Grass::TallGrassTop))
            | PaletteItem::Block(Block::Grass(Grass::LargeFernTop))
            | PaletteItem::Block(Block::Flower(Flower::SunflowerTop))
            | PaletteItem::Block(Block::Flower(Flower::LilacTop))
            | PaletteItem::Block(Block::Flower(Flower::PeonyTop))
            | PaletteItem::Block(Block::Flower(Flower::RoseBushTop)) => {
                let mut properties = nbt::Map::new();
                properties.insert("half".into(), nbt::Value::String("upper".into()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Grass(Grass::TallGrassBottom))
            | PaletteItem::Block(Block::Grass(Grass::LargeFernBottom))
            | PaletteItem::Block(Block::Flower(Flower::SunflowerBottom))
            | PaletteItem::Block(Block::Flower(Flower::LilacBottom))
            | PaletteItem::Block(Block::Flower(Flower::PeonyBottom))
            | PaletteItem::Block(Block::Flower(Flower::RoseBushBottom)) => {
                let mut properties = nbt::Map::new();
                properties.insert("half".into(), nbt::Value::String("lower".into()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Slab(Slab { position, waterlogged, .. })) => {
                let mut properties = nbt::Map::new();
                properties.insert("waterlogged".into(), nbt::Value::String(waterlogged.to_string()));
                properties.insert("type".into(), nbt::Value::String(position.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Torch { attached: Surface5::North })
            | PaletteItem::Block(Block::RedstoneTorch { attached: Surface5::North })
            | PaletteItem::Block(Block::SoulTorch { attached: Surface5::North }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String("south".into()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Torch { attached: Surface5::South })
            | PaletteItem::Block(Block::RedstoneTorch { attached: Surface5::South })
            | PaletteItem::Block(Block::SoulTorch { attached: Surface5::South }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String("north".into()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Torch { attached: Surface5::East })
            | PaletteItem::Block(Block::RedstoneTorch { attached: Surface5::East })
            | PaletteItem::Block(Block::SoulTorch { attached: Surface5::East }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String("west".into()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Torch { attached: Surface5::West })
            | PaletteItem::Block(Block::RedstoneTorch { attached: Surface5::West })
            | PaletteItem::Block(Block::SoulTorch { attached: Surface5::West }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String("east".into()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Fire { age }) => {
                let mut properties = nbt::Map::new();
                properties.insert("age".into(), nbt::Value::String(age.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Stairs(Stair { position, waterlogged, .. })) => {
                let facing = match position {
                    Edge8::DownEast
                    | Edge8::UpEast => "east".to_string(),
                    Edge8::DownNorth
                    | Edge8::UpNorth => "north".to_string(),
                    Edge8::DownSouth
                    | Edge8::UpSouth => "south".to_string(),
                    Edge8::DownWest
                    | Edge8::UpWest => "west".to_string(),
                };
                let half = match position {
                    Edge8::DownEast
                    | Edge8::DownNorth
                    | Edge8::DownSouth
                    | Edge8::DownWest => "bottom".to_string(),
                    Edge8::UpEast
                    | Edge8::UpNorth
                    | Edge8::UpSouth
                    | Edge8::UpWest => "top".to_string(),
                };

                let mut properties = nbt::Map::new();
                properties.insert("waterlogged".into(), nbt::Value::String(waterlogged.to_string()));
                properties.insert("facing".into(), nbt::Value::String(facing));
                properties.insert("half".into(), nbt::Value::String(half));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::ProtoBlock(ProtoBlock::Chest { facing, variant, waterlogged })
            | PaletteItem::ProtoBlock(ProtoBlock::TrappedChest { facing, variant, waterlogged }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                properties.insert("type".into(), nbt::Value::String(variant.unwrap_or(ChestVariant::Single).to_string()));
                properties.insert("waterlogged".into(), nbt::Value::String(waterlogged.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::GlazedTerracotta(GlazedTerracotta { facing, .. })) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::ProtoBlock(ProtoBlock::ShulkerBox { facing, .. }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            // TODO RedstoneWire: set connections
            PaletteItem::Block(Block::Farmland { wetness }) => {
                let mut properties = nbt::Map::new();
                properties.insert("moisture".into(), nbt::Value::String(wetness.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Beetroots { growth_stage }) => {
                let mut properties = nbt::Map::new();
                properties.insert("age".into(), nbt::Value::String(growth_stage.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Carrots { growth_stage })
            | PaletteItem::Block(Block::Potatoes { growth_stage })
            | PaletteItem::Block(Block::Wheat { growth_stage }) => {
                let mut properties = nbt::Map::new();
                properties.insert("age".into(), nbt::Value::String(growth_stage.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::PumpkinStem { state })
            | PaletteItem::Block(Block::MelonStem { state }) => {
                let mut properties = nbt::Map::new();
                match state {
                    StemState::Growing(age) => {
                        properties.insert("age".into(), nbt::Value::String(age.to_string()));
                    }
                    StemState::Attached(facing) => {
                        properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                    }
                }
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::ProtoBlock(ProtoBlock::Furnace { facing, lit }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                properties.insert("lit".into(), nbt::Value::String(lit.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::ProtoBlock(ProtoBlock::Sign { placement, waterlogged, .. })
            | PaletteItem::Block(Block::Head(Head { placement, waterlogged, .. })) => {
                let mut properties = nbt::Map::new();
                properties.insert("waterlogged".into(), nbt::Value::String(waterlogged.to_string()));
                match placement {
                    WallOrRotatedOnFloor::Floor(rotation) => {
                        let rotation = u8::from(*rotation);
                        properties.insert("rotation".into(), nbt::Value::String(rotation.to_string()));
                    }
                    WallOrRotatedOnFloor::Wall(facing) => {
                        properties.insert("facing".into(), nbt::Value::String(facing.opposite().to_string()));
                    }
                }
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Door(Door { facing, half, hinged_at, open, .. })) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                properties.insert("half".into(), nbt::Value::String(half.to_string()));
                properties.insert("hinge".into(), nbt::Value::String(hinged_at.to_string()));
                properties.insert("open".into(), nbt::Value::String(open.to_string()));
                properties.insert("powered".into(), nbt::Value::String("false".into()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Ladder { facing, waterlogged }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                properties.insert("waterlogged".into(), nbt::Value::String(waterlogged.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Lever(position, state)) => {
                let (face, facing) = match position {
                    SurfaceRotation12::DownFacingEast => ("floor", Surface4::East),
                    SurfaceRotation12::DownFacingNorth => ("floor", Surface4::North),
                    SurfaceRotation12::DownFacingSouth => ("floor", Surface4::South),
                    SurfaceRotation12::DownFacingWest => ("floor", Surface4::West),
                    SurfaceRotation12::East => ("wall", Surface4::East),
                    SurfaceRotation12::North => ("wall", Surface4::North),
                    SurfaceRotation12::South => ("wall", Surface4::South),
                    SurfaceRotation12::West => ("wall", Surface4::West),
                    SurfaceRotation12::UpFacingEast => ("ceiling", Surface4::East),
                    SurfaceRotation12::UpFacingNorth => ("ceiling", Surface4::North),
                    SurfaceRotation12::UpFacingSouth => ("ceiling", Surface4::South),
                    SurfaceRotation12::UpFacingWest => ("ceiling", Surface4::West),
                };
                let powered = match state {
                    OnOffState::On => true,
                    OnOffState::Off => false,
                };

                let mut properties = nbt::Map::new();
                properties.insert("face".into(), nbt::Value::String(face.to_string()));
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                properties.insert("powered".into(), nbt::Value::String(powered.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Button(_, position)) => {
                let (face, facing) = match position {
                    SurfaceRotation12::DownFacingEast => ("floor", Surface4::East),
                    SurfaceRotation12::DownFacingNorth => ("floor", Surface4::North),
                    SurfaceRotation12::DownFacingSouth => ("floor", Surface4::South),
                    SurfaceRotation12::DownFacingWest => ("floor", Surface4::West),
                    SurfaceRotation12::East => ("wall", Surface4::East),
                    SurfaceRotation12::North => ("wall", Surface4::North),
                    SurfaceRotation12::South => ("wall", Surface4::South),
                    SurfaceRotation12::West => ("wall", Surface4::West),
                    SurfaceRotation12::UpFacingEast => ("ceiling", Surface4::East),
                    SurfaceRotation12::UpFacingNorth => ("ceiling", Surface4::North),
                    SurfaceRotation12::UpFacingSouth => ("ceiling", Surface4::South),
                    SurfaceRotation12::UpFacingWest => ("ceiling", Surface4::West),
                };

                let mut properties = nbt::Map::new();
                properties.insert("face".into(), nbt::Value::String(face.to_string()));
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                properties.insert("powered".into(), nbt::Value::String("false".into()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Snow { thickness }) => {
                let mut properties = nbt::Map::new();
                properties.insert("layers".into(), nbt::Value::String(thickness.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Cactus { growth_stage })
            | PaletteItem::Block(Block::SugarCane { growth_stage }) => {
                let mut properties = nbt::Map::new();
                properties.insert("age".into(), nbt::Value::String(growth_stage.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Jukebox(jukebox)) => {
                let mut properties = nbt::Map::new();
                properties.insert("has_record".into(), nbt::Value::String(jukebox.record.is_some().to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Fence { waterlogged, .. }) => {
                let mut properties = nbt::Map::new();
                properties.insert("waterlogged".into(), nbt::Value::String(waterlogged.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::CarvedPumpkin { facing })
            | PaletteItem::Block(Block::JackOLantern { facing }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Cake { pieces }) => {
                let mut properties = nbt::Map::new();
                properties.insert("bites".into(), nbt::Value::String((7 - pieces).to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::NetherPortal { alignment }) => {
                let mut properties = nbt::Map::new();
                properties.insert("axis".into(), nbt::Value::String(alignment.unwrap_or(Axis2::X).to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::RedstoneRepeater(RedstoneRepeater { facing, delay })) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                properties.insert("delay".into(), nbt::Value::String(delay.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Trapdoor(Trapdoor { hinge_at, open, waterlogged, .. })) => {
                let facing = match hinge_at {
                    Edge8::DownEast
                    | Edge8::UpEast => "east".to_string(),
                    Edge8::DownNorth
                    | Edge8::UpNorth => "north".to_string(),
                    Edge8::DownSouth
                    | Edge8::UpSouth => "south".to_string(),
                    Edge8::DownWest
                    | Edge8::UpWest => "west".to_string(),
                };
                let half = match hinge_at {
                    Edge8::DownEast
                    | Edge8::DownNorth
                    | Edge8::DownSouth
                    | Edge8::DownWest => "bottom".to_string(),
                    Edge8::UpEast
                    | Edge8::UpNorth
                    | Edge8::UpSouth
                    | Edge8::UpWest => "top".to_string(),
                };

                let mut properties = nbt::Map::new();
                properties.insert("waterlogged".into(), nbt::Value::String(waterlogged.to_string()));
                properties.insert("facing".into(), nbt::Value::String(facing));
                properties.insert("half".into(), nbt::Value::String(half));
                properties.insert("open".into(), nbt::Value::String(open.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::BrownMushroomBlock { cap_directions: directions })
            | PaletteItem::Block(Block::RedMushroomBlock { cap_directions: directions })
            | PaletteItem::Block(Block::MushroomStem { stem_directions: directions }) => {
                let mut properties = nbt::Map::new();
                properties.insert("east".into(), nbt::Value::String(directions.east.to_string()));
                properties.insert("down".into(), nbt::Value::String(directions.down.to_string()));
                properties.insert("north".into(), nbt::Value::String(directions.north.to_string()));
                properties.insert("south".into(), nbt::Value::String(directions.south.to_string()));
                properties.insert("up".into(), nbt::Value::String(directions.up.to_string()));
                properties.insert("west".into(), nbt::Value::String(directions.west.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::IronBars { waterlogged })
            | PaletteItem::Block(Block::GlassPane { waterlogged, .. }) => {
                let mut properties = nbt::Map::new();
                properties.insert("waterlogged".into(), nbt::Value::String(waterlogged.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Vines(Vines { anchored_at })) => {
                let mut properties = nbt::Map::new();
                properties.insert("east".into(), nbt::Value::String(anchored_at.east.to_string()));
                properties.insert("north".into(), nbt::Value::String(anchored_at.north.to_string()));
                properties.insert("south".into(), nbt::Value::String(anchored_at.south.to_string()));
                properties.insert("up".into(), nbt::Value::String(anchored_at.up.to_string()));
                properties.insert("west".into(), nbt::Value::String(anchored_at.west.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::FenceGate { facing, open, .. }) => {
                let mut properties = nbt::Map::new();
                // TODO powered
                // TODO in_wall
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                properties.insert("open".into(), nbt::Value::String(open.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::NetherWart { growth_stage }) => {
                let mut properties = nbt::Map::new();
                properties.insert("age".into(), nbt::Value::String(growth_stage.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::ProtoBlock(ProtoBlock::BrewingStand) => {
                // TODO has_bottle_0
                // TODO has_bottle_1
                // TODO has_bottle_2
                None
            }
            PaletteItem::Block(Block::Cauldron { water_level }) => {
                let mut properties = nbt::Map::new();
                properties.insert("level".into(), nbt::Value::String(water_level.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::EndPortalFrame { facing, has_eye }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                properties.insert("eye".into(), nbt::Value::String(has_eye.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Cocoa { growth_stage, facing }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                properties.insert("age".into(), nbt::Value::String(growth_stage.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::EnderChest { facing, waterlogged }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                properties.insert("waterlogged".into(), nbt::Value::String(waterlogged.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::TripwireHook { facing }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                // TODO attached (bool as string)
                // TODO powered (bool as string)
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Wall { waterlogged, .. }) => {
                let mut properties = nbt::Map::new();
                properties.insert("waterlogged".into(), nbt::Value::String(waterlogged.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Anvil { facing, .. }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::RedstoneComparator { facing }) => {
                let mut properties = nbt::Map::new();
                properties.insert("mode".into(), nbt::Value::String("compare".into()));
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::RedstoneSubtractor { facing }) => {
                let mut properties = nbt::Map::new();
                properties.insert("mode".into(), nbt::Value::String("subtract".into()));
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::DaylightDetector) => {
                let mut properties = nbt::Map::new();
                properties.insert("inverted".into(), nbt::Value::String(false.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::InvertedDaylightDetector) => {
                let mut properties = nbt::Map::new();
                properties.insert("inverted".into(), nbt::Value::String(true.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::ProtoBlock(ProtoBlock::Hopper { facing }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Bed(Bed { facing, end, .. })) => {
                let mut properties = nbt::Map::new();
                // TODO occupied (bool as string)
                properties.insert("part".into(), nbt::Value::String(end.to_string()));
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::ProtoBlock(ProtoBlock::Banner{ placement, .. }) => {
                let mut properties = nbt::Map::new();
                match placement {
                    WallOrRotatedOnFloor::Floor(rotation) => {
                        let rotation = u8::from(*rotation);
                        properties.insert("rotation".into(), nbt::Value::String(rotation.to_string()));
                    }
                    WallOrRotatedOnFloor::Wall(facing) => {
                        properties.insert("facing".into(), nbt::Value::String(facing.opposite().to_string()));
                    }
                }
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::EndRod { facing }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            // TODO ChorusPlant
            PaletteItem::Block(Block::ChorusFlower { growth_stage }) => {
                let mut properties = nbt::Map::new();
                properties.insert("age".into(), nbt::Value::String(growth_stage.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Observer { facing }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Seagrass { variant }) => match variant {
                Seagrass::Seagrass => None,
                Seagrass::TallSeagrassTop => {
                    let mut properties = nbt::Map::new();
                    properties.insert("half".into(), nbt::Value::String("upper".into()));
                    Some(nbt::Value::Compound(properties))
                }
                Seagrass::TallSeagrassBottom => {
                    let mut properties = nbt::Map::new();
                    properties.insert("half".into(), nbt::Value::String("lower".into()));
                    Some(nbt::Value::Compound(properties))
                }
            }
            PaletteItem::Block(Block::SeaPickle { count, waterlogged }) => {
                let mut properties = nbt::Map::new();
                properties.insert("pickles".into(), nbt::Value::String(count.to_string()));
                properties.insert("waterlogged".into(), nbt::Value::String(waterlogged.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Kelp { growth_stage }) => {
                let mut properties = nbt::Map::new();
                properties.insert("age".into(), nbt::Value::String(growth_stage.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::TurtleEgg { count, age }) => {
                let mut properties = nbt::Map::new();
                properties.insert("hatch".into(), nbt::Value::String(age.to_string()));
                properties.insert("eggs".into(), nbt::Value::String(count.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Scaffolding { waterlogged }) => {
                let mut properties = nbt::Map::new();
                properties.insert("waterlogged".into(), nbt::Value::String(waterlogged.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Coral { waterlogged, .. }) => {
                let mut properties = nbt::Map::new();
                properties.insert("waterlogged".into(), nbt::Value::String(waterlogged.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::CoralFan { waterlogged, facing, .. }) => {
                let mut properties = nbt::Map::new();
                properties.insert("waterlogged".into(), nbt::Value::String(waterlogged.to_string()));
                match facing {
                    Surface5::Down => (),
                    facing => {
                        properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                    }
                }
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Loom { facing })
            | PaletteItem::Block(Block::Stonecutter { facing}) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Lantern { mounted_at, waterlogged })
            | PaletteItem::Block(Block::SoulLantern { mounted_at, waterlogged }) => {
                let hanging = match mounted_at {
                    Surface2::Up => true,
                    Surface2::Down => false,
                };
                let mut properties = nbt::Map::new();
                properties.insert("waterlogged".into(), nbt::Value::String(waterlogged.to_string()));
                properties.insert("hanging".into(), nbt::Value::String(hanging.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Composter { fullness }) => {
                let mut properties = nbt::Map::new();
                properties.insert("level".into(), nbt::Value::String(fullness.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Grindstone(position)) => {
                let (face, facing) = match position {
                    SurfaceRotation12::DownFacingEast => ("floor", Surface4::East),
                    SurfaceRotation12::DownFacingNorth => ("floor", Surface4::North),
                    SurfaceRotation12::DownFacingSouth => ("floor", Surface4::South),
                    SurfaceRotation12::DownFacingWest => ("floor", Surface4::West),
                    SurfaceRotation12::East => ("wall", Surface4::East),
                    SurfaceRotation12::North => ("wall", Surface4::North),
                    SurfaceRotation12::South => ("wall", Surface4::South),
                    SurfaceRotation12::West => ("wall", Surface4::West),
                    SurfaceRotation12::UpFacingEast => ("ceiling", Surface4::East),
                    SurfaceRotation12::UpFacingNorth => ("ceiling", Surface4::North),
                    SurfaceRotation12::UpFacingSouth => ("ceiling", Surface4::South),
                    SurfaceRotation12::UpFacingWest => ("ceiling", Surface4::West),
                };

                let mut properties = nbt::Map::new();
                properties.insert("face".into(), nbt::Value::String(face.to_string()));
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::SweetBerryBush{ growth_stage }) => {
                let mut properties = nbt::Map::new();
                properties.insert("age".into(), nbt::Value::String(growth_stage.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::RespawnAnchor { charges }) => {
                let mut properties = nbt::Map::new();
                properties.insert("charges".into(), nbt::Value::String(charges.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::Block(Block::Campfire { facing, lit, waterlogged })
            | PaletteItem::Block(Block::SoulCampfire { facing, lit, waterlogged }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                properties.insert("lit".into(), nbt::Value::String(lit.to_string()));
                properties.insert("waterlogged".into(), nbt::Value::String(waterlogged.to_string()));
                Some(nbt::Value::Compound(properties))
            }
            PaletteItem::ProtoBlock(ProtoBlock::Barrel { facing }) => {
                let mut properties = nbt::Map::new();
                properties.insert("facing".into(), nbt::Value::String(facing.to_string()));
                Some(nbt::Value::Compound(properties))
            }

            /*
            */

            // TODO Figure out what to do with:
            //      ChorusPlant, Fence, FenceGate, GlassPane, IronBars, RedstoneWire, Tripwire, Wall.
            //      They all have connection info in the Properties tag.
            //      Also:
            //      Campfire, SoulCampfire: "signal_fire" = "true" if on top of HayBale

            _ => None, // TODO should have compile time warning for unhandled block types. Perhaps list all?
        }
    }

    fn name(&self) -> &str {
        match self {
            PaletteItem::Block(Block::None)
            | PaletteItem::Block(Block::Unknown(_))
            | PaletteItem::Block(Block::Air) => "minecraft:air",
            PaletteItem::Block(Block::CaveAir) => "minecraft:cave_air",
            PaletteItem::Block(Block::Stone) => "minecraft:stone",
            PaletteItem::Block(Block::Granite) => "minecraft:granite",
            PaletteItem::Block(Block::PolishedGranite) => "minecraft:polished_granite",
            PaletteItem::Block(Block::Diorite) => "minecraft:diorite",
            PaletteItem::Block(Block::PolishedDiorite) => "minecraft:polished_diorite",
            PaletteItem::Block(Block::Andesite) => "minecraft:andesite",
            PaletteItem::Block(Block::PolishedAndesite) => "minecraft:polished_andesite",
            PaletteItem::Block(Block::GrassBlock) => "minecraft:grass_block",
            PaletteItem::Block(Block::Dirt) => "minecraft:dirt",
            PaletteItem::Block(Block::CoarseDirt) => "minecraft:coarse_dirt",
            PaletteItem::Block(Block::Podzol) => "minecraft:podzol",
            PaletteItem::Block(Block::Cobblestone) => "minecraft:cobblestone",
            PaletteItem::Block(Block::Planks { material }) => match material {
                WoodMaterial::Oak => "minecraft:oak_planks",
                WoodMaterial::Spruce => "minecraft:spruce_planks",
                WoodMaterial::Birch => "minecraft:birch_planks",
                WoodMaterial::Jungle => "minecraft:jungle_planks",
                WoodMaterial::Acacia => "minecraft:acacia_planks",
                WoodMaterial::DarkOak => "minecraft:dark_oak_planks",
                WoodMaterial::Crimson => "minecraft:crimson_planks",
                WoodMaterial::Warped => "minecraft:warped_planks",
            },
            PaletteItem::Block(Block::Sapling { material, .. }) => match material {
                SaplingMaterial::Oak => "minecraft:oak_sapling",
                SaplingMaterial::Spruce => "minecraft:spruce_sapling",
                SaplingMaterial::Birch => "minecraft:birch_sapling",
                SaplingMaterial::Jungle => "minecraft:jungle_sapling",
                SaplingMaterial::Acacia => "minecraft:acacia_sapling",
                SaplingMaterial::DarkOak => "minecraft:dark_oak_sapling",
                SaplingMaterial::Bamboo => "minecraft:bamboo_sapling",
            },
            PaletteItem::Block(Block::Bedrock) => "minecraft:bedrock",
            PaletteItem::Block(Block::Water { .. }) => "minecraft:water",
            PaletteItem::Block(Block::WaterSource) => "minecraft:water",
            PaletteItem::Block(Block::Lava { .. }) => "minecraft:lava",
            PaletteItem::Block(Block::LavaSource) => "minecraft:lava",
            PaletteItem::Block(Block::Sand) => "minecraft:sand",
            PaletteItem::Block(Block::RedSand) => "minecraft:red_sand",
            PaletteItem::Block(Block::Gravel) => "minecraft:gravel",
            PaletteItem::Block(Block::GoldOre) => "minecraft:gold_ore",
            PaletteItem::Block(Block::IronOre) => "minecraft:iron_ore",
            PaletteItem::Block(Block::CoalOre) => "minecraft:coal_ore",
            PaletteItem::Block(Block::Log(Log { material, stripped: false, bark_on_all_sides: false, .. })) => match material {
                    WoodMaterial::Oak => "minecraft:oak_log",
                    WoodMaterial::Spruce => "minecraft:spruce_log",
                    WoodMaterial::Birch => "minecraft:birch_log",
                    WoodMaterial::Jungle => "minecraft:jungle_log",
                    WoodMaterial::Acacia => "minecraft:acacia_log",
                    WoodMaterial::DarkOak => "minecraft:dark_oak_log",
                    WoodMaterial::Crimson => "minecraft:crimson_stem",
                    WoodMaterial::Warped => "minecraft:warped_stem",
            },
            PaletteItem::Block(Block::Log(Log { material, stripped: true, bark_on_all_sides: false, .. })) => match material {
                    WoodMaterial::Oak => "minecraft:stripped_oak_log",
                    WoodMaterial::Spruce => "minecraft:stripped_spruce_log",
                    WoodMaterial::Birch => "minecraft:stripped_birch_log",
                    WoodMaterial::Jungle => "minecraft:stripped_jungle_log",
                    WoodMaterial::Acacia => "minecraft:stripped_acacia_log",
                    WoodMaterial::DarkOak => "minecraft:stripped_dark_oak_log",
                    WoodMaterial::Crimson => "minecraft:stripped_crimson_stem",
                    WoodMaterial::Warped => "minecraft:stripped_warped_stem",
            },
            PaletteItem::Block(Block::Log(Log { material, stripped: false, bark_on_all_sides: true, .. })) => match material {
                    WoodMaterial::Oak => "minecraft:oak_wood",
                    WoodMaterial::Spruce => "minecraft:spruce_wood",
                    WoodMaterial::Birch => "minecraft:birch_wood",
                    WoodMaterial::Jungle => "minecraft:jungle_wood",
                    WoodMaterial::Acacia => "minecraft:acacia_wood",
                    WoodMaterial::DarkOak => "minecraft:dark_oak_wood",
                    WoodMaterial::Crimson => "minecraft:crimson_hyphae",
                    WoodMaterial::Warped => "minecraft:warped_hyphae",
            },
            PaletteItem::Block(Block::Log(Log { material, stripped: true, bark_on_all_sides: true, .. })) => match material {
                    WoodMaterial::Oak => "minecraft:stripped_oak_wood",
                    WoodMaterial::Spruce => "minecraft:stripped_spruce_wood",
                    WoodMaterial::Birch => "minecraft:stripped_birch_wood",
                    WoodMaterial::Jungle => "minecraft:stripped_jungle_wood",
                    WoodMaterial::Acacia => "minecraft:stripped_acacia_wood",
                    WoodMaterial::DarkOak => "minecraft:stripped_dark_oak_wood",
                    WoodMaterial::Crimson => "minecraft:stripped_crimson_hyphae",
                    WoodMaterial::Warped => "minecraft:stripped_warped_hyphae",
            },
            PaletteItem::Block(Block::Leaves { material, .. }) => match material {
                LeavesMaterial::Oak => "minecraft:oak_leaves",
                LeavesMaterial::Spruce => "minecraft:spruce_leaves",
                LeavesMaterial::Birch => "minecraft:birch_leaves",
                LeavesMaterial::Jungle => "minecraft:jungle_leaves",
                LeavesMaterial::Acacia => "minecraft:acacia_leaves",
                LeavesMaterial::DarkOak => "minecraft:dark_oak_leaves",
            }
            PaletteItem::Block(Block::Sponge) => "minecraft:sponge",
            PaletteItem::Block(Block::WetSponge) => "minecraft:wet_sponge",
            PaletteItem::Block(Block::Glass { colour }) => match colour {
                None => "minecraft:glass",
                Some(Colour::White) => "minecraft:white_stained_glass",
                Some(Colour::Orange) => "minecraft:orange_stained_glass",
                Some(Colour::Magenta) => "minecraft:magenta_stained_glass",
                Some(Colour::LightBlue) => "minecraft:light_blue_stained_glass",
                Some(Colour::Yellow) => "minecraft:yellow_stained_glass",
                Some(Colour::Lime) => "minecraft:lime_stained_glass",
                Some(Colour::Pink) => "minecraft:pink_stained_glass",
                Some(Colour::Gray) => "minecraft:gray_stained_glass",
                Some(Colour::LightGray) => "minecraft:light_gray_stained_glass",
                Some(Colour::Cyan) => "minecraft:cyan_stained_glass",
                Some(Colour::Purple) => "minecraft:purple_stained_glass",
                Some(Colour::Blue) => "minecraft:blue_stained_glass",
                Some(Colour::Brown) => "minecraft:brown_stained_glass",
                Some(Colour::Green) => "minecraft:green_stained_glass",
                Some(Colour::Red) => "minecraft:red_stained_glass",
                Some(Colour::Black) => "minecraft:black_stained_glass",
            }
            PaletteItem::Block(Block::LapisLazuliOre) => "minecraft:lapis_ore",
            PaletteItem::Block(Block::LapisLazuliBlock) => "minecraft:lapis_block",
            PaletteItem::ProtoBlock(ProtoBlock::Dispenser { .. }) => "minecraft:dispenser",
            PaletteItem::Block(Block::Sandstone) => "minecraft:sandstone",
            PaletteItem::Block(Block::ChiseledSandstone) => "minecraft:chiseled_sandstone",
            PaletteItem::Block(Block::SmoothSandstone) => "minecraft:smooth_sandstone",
            PaletteItem::Block(Block::CutSandstone) => "minecraft:cut_sandstone",
            PaletteItem::Block(Block::Noteblock(Noteblock { .. })) => "minecraft:note_block",
            PaletteItem::Block(Block::Rail { variant, .. }) => match variant {
                RailType::Activator => "minecraft:activator_rail",
                RailType::Detector => "minecraft:detector_rail",
                RailType::Normal => "minecraft:rail",
                RailType::Powered => "minecraft:powered_rail",
            }
            PaletteItem::Block(Block::PistonHead { .. })
            | PaletteItem::Block(Block::StickyPistonHead { .. }) => "minecraft:piston_head",
            PaletteItem::Block(Block::Piston { .. }) => "minecraft:piston",
            PaletteItem::Block(Block::StickyPiston { .. }) => "minecraft:sticky_piston",
            PaletteItem::Block(Block::Cobweb) => "minecraft:cobweb",
            PaletteItem::Block(Block::Grass(grass_variant)) => match grass_variant {
                Grass::Grass => "minecraft:grass",
                Grass::Fern => "minecraft:fern",
                Grass::TallGrassTop
                | Grass::TallGrassBottom => "minecraft:tall_grass",
                Grass::LargeFernTop
                | Grass::LargeFernBottom => "minecraft:large_fern",
            }
            PaletteItem::Block(Block::DeadBush) => "minecraft:dead_bush",
            PaletteItem::Block(Block::Wool { colour }) => match colour {
                Colour::White => "minecraft:white_wool",
                Colour::Orange => "minecraft:orange_wool",
                Colour::Magenta => "minecraft:magenta_wool",
                Colour::LightBlue => "minecraft:light_blue_wool",
                Colour::Yellow => "minecraft:yellow_wool",
                Colour::Lime => "minecraft:lime_wool",
                Colour::Pink => "minecraft:pink_wool",
                Colour::Gray => "minecraft:gray_wool",
                Colour::LightGray => "minecraft:light_gray_wool",
                Colour::Cyan => "minecraft:cyan_wool",
                Colour::Purple => "minecraft:purple_wool",
                Colour::Blue => "minecraft:blue_wool",
                Colour::Brown => "minecraft:brown_wool",
                Colour::Green => "minecraft:green_wool",
                Colour::Red => "minecraft:red_wool",
                Colour::Black => "minecraft:black_wool",
            }
            // TODO block 36 piston_extension ("Block moved by Piston")
            PaletteItem::Block(Block::Flower(flower_variant)) => match flower_variant {
                Flower::Dandelion => "minecraft:dandelion",
                Flower::Poppy => "minecraft:poppy",
                Flower::BlueOrchid => "minecraft:blue_orchid",
                Flower::Allium => "minecraft:allium",
                Flower::AzureBluet => "minecraft:azure_bluet",
                Flower::TulipRed => "minecraft:red_tulip",
                Flower::TulipOrange => "minecraft:orange_tulip",
                Flower::TulipWhite => "minecraft:white_tulip",
                Flower::TulipPink => "minecraft:pink_tulip",
                Flower::OxeyeDaisy => "minecraft:oxeye_daisy",
                Flower::Cornflower => "minecraft:cornflower",
                Flower::LilyOfTheValley => "minecraft:lily_of_the_valley",
                Flower::WitherRose => "minecraft:wither_rose",
                Flower::SunflowerTop
                | Flower::SunflowerBottom => "minecraft:sunflower",
                Flower::LilacTop
                | Flower::LilacBottom => "minecraft:lilac",
                Flower::PeonyTop
                | Flower::PeonyBottom => "minecraft:peony",
                Flower::RoseBushTop
                | Flower::RoseBushBottom => "minecraft:rose_bush",
            }
            PaletteItem::Block(Block::BrownMushroom) => "minecraft:brown_mushroom",
            PaletteItem::Block(Block::RedMushroom) => "minecraft:red_mushroom",
            PaletteItem::Block(Block::BlockOfGold) => "minecraft:gold_block",
            PaletteItem::Block(Block::BlockOfIron) => "minecraft:iron_block",
            PaletteItem::Block(Block::Slab(Slab { material, .. })) => match material {
                SlabMaterial::Oak => "minecraft:oak_slab",
                SlabMaterial::Spruce => "minecraft:spruce_slab",
                SlabMaterial::Birch => "minecraft:birch_slab",
                SlabMaterial::Jungle => "minecraft:jungle_slab",
                SlabMaterial::Acacia => "minecraft:acacia_slab",
                SlabMaterial::DarkOak => "minecraft:dark_oak_slab",
                SlabMaterial::Crimson => "minecraft:crimson_slab",
                SlabMaterial::Warped => "minecraft:warped_slab",
                SlabMaterial::Stone => "minecraft:stone_slab",
                SlabMaterial::Sandstone => "minecraft:sandstone_slab",
                SlabMaterial::PetrifiedOak => "minecraft:petrified_oak_slab",
                SlabMaterial::Cobblestone => "minecraft:cobblestone_slab",
                SlabMaterial::Brick => "minecraft:brick_slab",
                SlabMaterial::StoneBrick => "minecraft:stone_brick_slab",
                SlabMaterial::NetherBrick => "minecraft:nether_brick_slab",
                SlabMaterial::Quartz => "minecraft:quartz_slab",
                SlabMaterial::RedSandstone => "minecraft:red_sandstone_slab",
                SlabMaterial::Purpur => "minecraft:purpur_slab",
                SlabMaterial::Prismarine => "minecraft:prismarine_slab",
                SlabMaterial::PrismarineBrick => "minecraft:prismarine_brick_slab",
                SlabMaterial::DarkPrismarine => "minecraft:dark_prismarine_slab",
                SlabMaterial::Andesite => "minecraft:andesite_slab",
                SlabMaterial::Granite => "minecraft:granite_slab",
                SlabMaterial::Diorite => "minecraft:diorite_slab",
                SlabMaterial::PolishedAndesite => "minecraft:polished_andesite_slab",
                SlabMaterial::PolishedGranite => "minecraft:polished_granite_slab",
                SlabMaterial::PolishedDiorite => "minecraft:polished_diorite_slab",
                SlabMaterial::CutSandstone => "minecraft:cut_sandstone_slab",
                SlabMaterial::CutRedSandstone => "minecraft:cut_red_sandstone_slab",
                SlabMaterial::SmoothSandstone => "minecraft:smooth_sandstone_slab",
                SlabMaterial::SmoothRedSandstone => "minecraft:smooth_red_sandstone_slab",
                SlabMaterial::SmoothQuartz => "minecraft:smooth_quartz_slab",
                SlabMaterial::SmoothStone => "minecraft:smooth_stone_slab",
                SlabMaterial::RedNetherBrick => "minecraft:red_nether_brick_slab",
                SlabMaterial::EndStoneBrick => "minecraft:end_stone_brick_slab",
                SlabMaterial::Blackstone => "minecraft:blackstone_slab",
                SlabMaterial::PolishedBlackstone => "minecraft:polished_blackstone_slab",
                SlabMaterial::PolishedBlackstoneBrick => "minecraft:polished_blackstone_brick_slab",
                SlabMaterial::MossyCobblestone => "minecraft:mossy_cobblestone_slab",
                SlabMaterial::MossyStoneBrick => "minecraft:mossy_stone_brick_slab",
            }
            PaletteItem::Block(Block::SmoothQuartz) => "minecraft:smooth_quartz",
            PaletteItem::Block(Block::SmoothStone) => "minecraft:smooth_stone",
            PaletteItem::Block(Block::BrickBlock) => "minecraft:bricks",
            PaletteItem::Block(Block::TNT) => "minecraft:tnt",
            PaletteItem::Block(Block::Bookshelf) => "minecraft:bookshelf",
            PaletteItem::Block(Block::MossyCobblestone) => "minecraft:mossy_cobblestone",
            PaletteItem::Block(Block::Obsidian) => "minecraft:obsidian",
            PaletteItem::Block(Block::CryingObsidian) => "minecraft:crying_obsidian",
            PaletteItem::Block(Block::Torch { attached }) => match attached {
                Surface5::Down => "minecraft:torch",
                _ => "minecraft:wall_torch",
            }
            PaletteItem::Block(Block::RedstoneTorch { attached }) => match attached {
                Surface5::Down => "minecraft:redstone_torch",
                _ => "minecraft:redstone_wall_torch",
            }
            PaletteItem::Block(Block::SoulTorch { attached }) => match attached {
                Surface5::Down => "minecraft:soul_torch",
                _ => "minecraft:soul_wall_torch",
            }
            PaletteItem::Block(Block::Fire { .. }) => "minecraft:fire",
            // TODO block 52 / minecraft:spawner / mob spawner
            PaletteItem::Block(Block::DiamondOre) => "minecraft:diamond_ore",
            PaletteItem::Block(Block::BlockOfDiamond) => "minecraft:diamond_block",
            PaletteItem::Block(Block::CraftingTable) => "minecraft:crafting_table",
            PaletteItem::Block(Block::Stairs(Stair { material, .. })) => match material {
                StairMaterial::Acacia => "minecraft:acacia_stairs",
                StairMaterial::Andesite => "minecraft:andesite_stairs",
                StairMaterial::Birch => "minecraft:birch_stairs",
                StairMaterial::Blackstone => "minecraft:blackstone_stairs",
                StairMaterial::Brick => "minecraft:brick_stairs",
                StairMaterial::Cobblestone => "minecraft:cobblestone_stairs",
                StairMaterial::Crimson => "minecraft:crimson_stairs",
                StairMaterial::DarkOak => "minecraft:dark_oak_stairs",
                StairMaterial::DarkPrismarine => "minecraft:dark_prismarine_stairs",
                StairMaterial::Diorite => "minecraft:diorite_stairs",
                StairMaterial::EndStoneBrick => "minecraft:end_stone_brick_stairs",
                StairMaterial::Granite => "minecraft:granite_stairs",
                StairMaterial::Jungle => "minecraft:jungle_stairs",
                StairMaterial::MossyCobblestone => "minecraft:mossy_cobblestone_stairs",
                StairMaterial::MossyStoneBrick => "minecraft:mossy_stone_brick_stairs",
                StairMaterial::NetherBrick => "minecraft:nether_brick_stairs",
                StairMaterial::Oak => "minecraft:oak_stairs",
                StairMaterial::PolishedAndesite => "minecraft:polished_andesite_stairs",
                StairMaterial::PolishedBlackstone => "minecraft:polished_blackstone_stairs",
                StairMaterial::PolishedBlackstoneBrick => "minecraft:polished_blackstone_brick_stairs",
                StairMaterial::PolishedDiorite => "minecraft:polished_diorite_stairs",
                StairMaterial::PolishedGranite => "minecraft:polished_granite_stairs",
                StairMaterial::Prismarine => "minecraft:prismarine_stairs",
                StairMaterial::PrismarineBrick => "minecraft:prismarine_brick_stairs",
                StairMaterial::Purpur => "minecraft:purpur_stairs",
                StairMaterial::Quartz => "minecraft:quartz_stairs",
                StairMaterial::RedNetherBrick => "minecraft:red_nether_brick_stairs",
                StairMaterial::RedSandstone => "minecraft:red_sandstone_stairs",
                StairMaterial::Sandstone => "minecraft:sandstone_stairs",
                StairMaterial::SmoothQuartz => "minecraft:smooth_quartz_stairs",
                StairMaterial::SmoothRedSandstone => "minecraft:smooth_red_sandstone_stairs",
                StairMaterial::SmoothSandstone => "minecraft:smooth_sandstone_stairs",
                StairMaterial::Spruce => "minecraft:spruce_stairs",
                StairMaterial::Stone => "minecraft:stone_stairs",
                StairMaterial::StoneBrick => "minecraft:stone_brick_stairs",
                StairMaterial::Warped => "minecraft:warped_stairs",
            }
            PaletteItem::ProtoBlock(ProtoBlock::Chest { .. }) => "minecraft:chest",
            PaletteItem::Block(Block::RedstoneWire) => "minecraft:redstone_wire",
            PaletteItem::Block(Block::Farmland { .. }) => "minecraft:farmland",
            PaletteItem::Block(Block::Beetroots { .. }) => "minecraft:beetroots",
            PaletteItem::Block(Block::Carrots { .. }) => "minecraft:carrots",
            PaletteItem::Block(Block::Potatoes { .. }) => "minecraft:potatoes",
            PaletteItem::Block(Block::Wheat { .. }) => "minecraft:wheat",
            PaletteItem::Block(Block::PumpkinStem { state }) => match state {
                StemState::Growing(_) => "minecraft:pumpkin_stem",
                StemState::Attached(_) => "minecraft:attached_pumpkin_stem",
            }
            PaletteItem::Block(Block::MelonStem { state }) => match state {
                StemState::Growing(_) => "minecraft:melon_stem",
                StemState::Attached(_) => "minecraft:attached_melon_stem",
            }
            PaletteItem::Block(Block::Pumpkin) => "minecraft:pumpkin",
            PaletteItem::Block(Block::Melon) => "minecraft:melon",
            PaletteItem::ProtoBlock(ProtoBlock::Furnace { .. }) => "minecraft:furnace",
            PaletteItem::ProtoBlock(ProtoBlock::Sign { placement, material, .. }) => match placement {
                WallOrRotatedOnFloor::Floor(_) => match material {
                    WoodMaterial::Acacia => "minecraft:acacia_sign",
                    WoodMaterial::Birch => "minecraft:birch_sign",
                    WoodMaterial::Crimson => "minecraft:crimson_sign",
                    WoodMaterial::DarkOak => "minecraft:dark_oak_sign",
                    WoodMaterial::Jungle => "minecraft:jungle_sign",
                    WoodMaterial::Oak => "minecraft:oak_sign",
                    WoodMaterial::Spruce => "minecraft:spruce_sign",
                    WoodMaterial::Warped => "minecraft:warped_sign",
                }
                WallOrRotatedOnFloor::Wall(_) => match material {
                    WoodMaterial::Acacia => "minecraft:acacia_wall_sign",
                    WoodMaterial::Birch => "minecraft:birch_wall_sign",
                    WoodMaterial::Crimson => "minecraft:crimson_wall_sign",
                    WoodMaterial::DarkOak => "minecraft:dark_oak_wall_sign",
                    WoodMaterial::Jungle => "minecraft:jungle_wall_sign",
                    WoodMaterial::Oak => "minecraft:oak_wall_sign",
                    WoodMaterial::Spruce => "minecraft:spruce_wall_sign",
                    WoodMaterial::Warped => "minecraft:warped_wall_sign",
                }
            }
            PaletteItem::Block(Block::Door(Door { material, .. })) => match material {
                DoorMaterial::Acacia => "minecraft:acacia_door",
                DoorMaterial::Birch => "minecraft:birch_door",
                DoorMaterial::Crimson => "minecraft:crimson_door",
                DoorMaterial::DarkOak => "minecraft:dark_oak_door",
                DoorMaterial::Iron => "minecraft:iron_door",
                DoorMaterial::Jungle => "minecraft:jungle_door",
                DoorMaterial::Oak => "minecraft:oak_door",
                DoorMaterial::Spruce => "minecraft:spruce_door",
                DoorMaterial::Warped => "minecraft:warped_door",
            }
            PaletteItem::Block(Block::Ladder { .. }) => "minecraft:ladder",
            PaletteItem::Block(Block::Lever(..)) => "minecraft:lever",
            PaletteItem::Block(Block::PressurePlate { material }) => match material {
                PressurePlateMaterial::Acacia => "minecraft:acacia_pressure_plate",
                PressurePlateMaterial::Birch => "minecraft:birch_pressure_plate",
                PressurePlateMaterial::Crimson => "minecraft:crimson_pressure_plate",
                PressurePlateMaterial::DarkOak => "minecraft:dark_oak_pressure_plate",
                PressurePlateMaterial::Gold => "minecraft:light_weighted_pressure_plate",
                PressurePlateMaterial::Iron => "minecraft:heavy_weighted_pressure_plate",
                PressurePlateMaterial::Jungle => "minecraft:jungle_pressure_plate",
                PressurePlateMaterial::Oak => "minecraft:oak_pressure_plate",
                PressurePlateMaterial::PolishedBlackstone => "minecraft:polished_blackstone_pressure_plate",
                PressurePlateMaterial::Spruce => "minecraft:spruce_pressure_plate",
                PressurePlateMaterial::Stone => "minecraft:stone_pressure_plate",
                PressurePlateMaterial::Warped => "minecraft:warped_pressure_plate",
            }
            PaletteItem::Block(Block::RedstoneOre) => "minecraft:redstone_ore",
            PaletteItem::Block(Block::Button(material, _)) => match material {
                ButtonMaterial::Acacia => "minecraft:acacia_button",
                ButtonMaterial::Birch => "minecraft:birch_button",
                ButtonMaterial::Crimson => "minecraft:crimson_button",
                ButtonMaterial::DarkOak => "minecraft:dark_oak_button",
                ButtonMaterial::Jungle => "minecraft:jungle_button",
                ButtonMaterial::Oak => "minecraft:oak_button",
                ButtonMaterial::PolishedBlackstone => "minecraft:polished_blackstone_button",
                ButtonMaterial::Spruce => "minecraft:spruce_button",
                ButtonMaterial::Stone => "minecraft:stone_button",
                ButtonMaterial::Warped => "minecraft:warped_button",
            }
            PaletteItem::Block(Block::Snow { .. }) => "minecraft:snow",
            PaletteItem::Block(Block::SnowBlock) => "minecraft:snow_block",
            PaletteItem::Block(Block::Ice) => "minecraft:ice",
            PaletteItem::Block(Block::PackedIce) => "minecraft:packed_ice",
            PaletteItem::Block(Block::BlueIce) => "minecraft:blue_ice",
            PaletteItem::Block(Block::Clay) => "minecraft:clay",
            PaletteItem::Block(Block::Cactus { .. }) => "minecraft:cactus",
            PaletteItem::Block(Block::SugarCane { .. }) => "minecraft:sugar_cane",
            PaletteItem::Block(Block::Jukebox(..))
            | PaletteItem::ProtoBlock(ProtoBlock::Jukebox) => "minecraft:jukebox",
            PaletteItem::Block(Block::Fence { material, .. }) => match material {
                FenceMaterial::Acacia => "minecraft:acacia_fence",
                FenceMaterial::Birch => "minecraft:birch_fence",
                FenceMaterial::Crimson => "minecraft:crimson_fence",
                FenceMaterial::DarkOak => "minecraft:dark_oak_fence",
                FenceMaterial::Jungle => "minecraft:jungle_fence",
                FenceMaterial::NetherBrick => "minecraft:nether_brick_fence",
                FenceMaterial::Oak => "minecraft:oak_fence",
                FenceMaterial::Spruce => "minecraft:spruce_fence",
                FenceMaterial::Warped => "minecraft:warped_fence",
            }
            PaletteItem::Block(Block::CarvedPumpkin { .. }) => "minecraft:carved_pumpkin",
            PaletteItem::Block(Block::Netherrack) => "minecraft:netherrack",
            PaletteItem::Block(Block::SoulSand) => "minecraft:soul_sand",
            PaletteItem::Block(Block::SoulSoil) => "minecraft:soul_soil",
            PaletteItem::Block(Block::Glowstone) => "minecraft:glowstone",
            PaletteItem::Block(Block::JackOLantern { .. }) => "minecraft:jack_o_lantern",
            PaletteItem::Block(Block::Cake { .. }) => "minecraft:cake",
            PaletteItem::Block(Block::NetherPortal { .. }) => "minecraft:nether_portal",
            PaletteItem::Block(Block::RedstoneRepeater { .. }) => "minecraft:repeater",
            PaletteItem::Block(Block::Trapdoor(Trapdoor { material, .. })) => match material {
                DoorMaterial::Acacia => "minecraft:acacia_trapdoor",
                DoorMaterial::Birch => "minecraft:birch_trapdoor",
                DoorMaterial::Crimson => "minecraft:crimson_trapdoor",
                DoorMaterial::DarkOak => "minecraft:dark_oak_trapdoor",
                DoorMaterial::Iron => "minecraft:iron_trapdoor",
                DoorMaterial::Jungle => "minecraft:jungle_trapdoor",
                DoorMaterial::Oak => "minecraft:oak_trapdoor",
                DoorMaterial::Spruce => "minecraft:spruce_trapdoor",
                DoorMaterial::Warped => "minecraft:warped_trapdoor",
            }
            PaletteItem::Block(Block::InfestedStone) => "minecraft:infested_stone",
            PaletteItem::Block(Block::InfestedCobblestone) => "minecraft:infested_cobblestone",
            PaletteItem::Block(Block::InfestedStoneBricks) => "minecraft:infested_stone_bricks",
            PaletteItem::Block(Block::InfestedMossyStoneBricks) => "minecraft:infested_mossy_stone_bricks",
            PaletteItem::Block(Block::InfestedCrackedStoneBricks) => "minecraft:infested_cracked_stone_bricks",
            PaletteItem::Block(Block::InfestedChiseledStoneBricks) => "minecraft:infested_chiseled_stone_bricks",
            PaletteItem::Block(Block::StoneBricks) => "minecraft:stone_bricks",
            PaletteItem::Block(Block::MossyStoneBricks) => "minecraft:mossy_stone_bricks",
            PaletteItem::Block(Block::CrackedStoneBricks) => "minecraft:cracked_stone_bricks",
            PaletteItem::Block(Block::ChiseledStoneBricks) => "minecraft:chiseled_stone_bricks",
            PaletteItem::Block(Block::BrownMushroomBlock { .. }) => "minecraft:brown_mushroom_block",
            PaletteItem::Block(Block::RedMushroomBlock { .. }) => "minecraft:red_mushroom_block",
            PaletteItem::Block(Block::MushroomStem { .. }) => "minecraft:mushroom_stem",
            PaletteItem::Block(Block::IronBars { .. }) => "minecraft:iron_bars",
            PaletteItem::Block(Block::GlassPane { colour, .. }) => match colour {
                None => "minecraft:glass_pane",
                Some(Colour::White) => "minecraft:white_stained_glass_pane",
                Some(Colour::Orange) => "minecraft:orange_stained_glass_pane",
                Some(Colour::Magenta) => "minecraft:magenta_stained_glass_pane",
                Some(Colour::LightBlue) => "minecraft:light_blue_stained_glass_pane",
                Some(Colour::Yellow) => "minecraft:yellow_stained_glass_pane",
                Some(Colour::Lime) => "minecraft:lime_stained_glass_pane",
                Some(Colour::Pink) => "minecraft:pink_stained_glass_pane",
                Some(Colour::Gray) => "minecraft:gray_stained_glass_pane",
                Some(Colour::LightGray) => "minecraft:light_gray_stained_glass_pane",
                Some(Colour::Cyan) => "minecraft:cyan_stained_glass_pane",
                Some(Colour::Purple) => "minecraft:purple_stained_glass_pane",
                Some(Colour::Blue) => "minecraft:blue_stained_glass_pane",
                Some(Colour::Brown) => "minecraft:brown_stained_glass_pane",
                Some(Colour::Green) => "minecraft:green_stained_glass_pane",
                Some(Colour::Red) => "minecraft:red_stained_glass_pane",
                Some(Colour::Black) => "minecraft:black_stained_glass_pane",
            }
            PaletteItem::Block(Block::Vines(_)) => "minecraft:vine",
            PaletteItem::Block(Block::FenceGate { material, .. }) => match material {
                WoodMaterial::Acacia => "minecraft:acacia_fence_gate",
                WoodMaterial::Birch => "minecraft:birch_fence_gate",
                WoodMaterial::Crimson => "minecraft:crimson_fence_gate",
                WoodMaterial::DarkOak => "minecraft:dark_oak_fence_gate",
                WoodMaterial::Jungle => "minecraft:jungle_fence_gate",
                WoodMaterial::Oak => "minecraft:oak_fence_gate",
                WoodMaterial::Spruce => "minecraft:spruce_fence_gate",
                WoodMaterial::Warped => "minecraft:warped_fence_gate",
            },
            PaletteItem::Block(Block::Mycelium) => "minecraft:mycelium",
            PaletteItem::Block(Block::LilyPad) => "minecraft:lily_pad",
            PaletteItem::Block(Block::NetherBricks) => "minecraft:nether_bricks",
            PaletteItem::Block(Block::CrackedNetherBricks) => "minecraft:cracked_nether_bricks",
            PaletteItem::Block(Block::ChiseledNetherBricks) => "minecraft:chiseled_nether_bricks",
            PaletteItem::Block(Block::NetherWart { .. }) => "minecraft:nether_wart",
            PaletteItem::ProtoBlock(ProtoBlock::EnchantingTable) => "minecraft:enchanting_table",
            PaletteItem::ProtoBlock(ProtoBlock::BrewingStand) => "minecraft:brewing_stand",
            PaletteItem::Block(Block::Cauldron { .. }) => "minecraft:cauldron",
            PaletteItem::Block(Block::EndPortal) => "minecraft:end_portal",
            PaletteItem::Block(Block::EndPortalFrame { .. }) => "minecraft:end_portal_frame",
            PaletteItem::Block(Block::EndStone) => "minecraft:end_stone",
            PaletteItem::Block(Block::DragonEgg) => "minecraft:dragon_egg",
            PaletteItem::Block(Block::RedstoneLamp) => "minecraft:redstone_lamp",
            PaletteItem::Block(Block::Cocoa { .. }) => "minecraft:cocoa",
            PaletteItem::Block(Block::EmeraldOre) => "minecraft:emerald_ore",
            PaletteItem::Block(Block::EnderChest { .. }) => "minecraft:ender_chest",
            PaletteItem::Block(Block::TripwireHook { .. }) => "minecraft:tripwire_hook",
            PaletteItem::Block(Block::Tripwire) => "minecraft:tripwire",
            PaletteItem::Block(Block::BlockOfEmerald) => "minecraft:emerald_block",
            // TODO 137 command block // Deferred for now, too complicated
            PaletteItem::ProtoBlock(ProtoBlock::Beacon { .. }) => "minecraft:beacon",
            PaletteItem::Block(Block::Wall { material, .. }) => match material {
                WallMaterial::Andesite => "minecraft:andesite_wall",
                WallMaterial::Blackstone => "minecraft:blackstone_wall",
                WallMaterial::Brick => "minecraft:brick_wall",
                WallMaterial::Cobblestone => "minecraft:cobblestone_wall",
                WallMaterial::Diorite => "minecraft:diorite_wall",
                WallMaterial::EndStoneBrick => "minecraft:end_stone_brick_wall",
                WallMaterial::Granite => "minecraft:granite_wall",
                WallMaterial::MossyCobblestone => "minecraft:mossy_cobblestone_wall",
                WallMaterial::MossyStoneBrick => "minecraft:mossy_stone_brick_wall",
                WallMaterial::NetherBrick => "minecraft:nether_brick_wall",
                WallMaterial::PolishedBlackstone => "minecraft:polished_blackstone_wall",
                WallMaterial::PolishedBlackstoneBrick => "minecraft:polished_blackstone_brick_wall",
                WallMaterial::Prismarine => "minecraft:prismarine_wall",
                WallMaterial::RedNetherBrick => "minecraft:red_nether_brick_wall",
                WallMaterial::RedSandstone => "minecraft:red_sandstone_wall",
                WallMaterial::Sandstone => "minecraft:sandstone_wall",
                WallMaterial::StoneBrick => "minecraft:stone_brick_wall",
            }
            PaletteItem::Block(Block::FlowerPot(FlowerPot { plant })) => match plant {
                None => "minecraft:flower_pot",
                Some(plant) => match plant {
                    PottedPlant::AcaciaSapling => "minecraft:potted_acacia_sapling",
                    PottedPlant::Allium => "minecraft:potted_allium",
                    PottedPlant::AzureBluet => "minecraft:potted_azure_bluet",
                    PottedPlant::Bamboo => "minecraft:potted_bamboo",
                    PottedPlant::BirchSapling => "minecraft:potted_birch_sapling",
                    PottedPlant::BlueOrchid => "minecraft:potted_blue_orchid",
                    PottedPlant::BrownMushroom => "minecraft:potted_brown_mushroom",
                    PottedPlant::Cactus => "minecraft:potted_cactus",
                    PottedPlant::Cornflower => "minecraft:potted_cornflower",
                    PottedPlant::CrimsonFungus => "minecraft:potted_crimson_fungus",
                    PottedPlant::CrimsonRoots => "minecraft:potted_crimson_roots",
                    PottedPlant::Dandelion => "minecraft:potted_dandelion",
                    PottedPlant::DarkOakSapling => "minecraft:potted_dark_oak_sapling",
                    PottedPlant::DeadBush => "minecraft:potted_dead_bush",
                    PottedPlant::Fern => "minecraft:potted_fern",
                    PottedPlant::JungleSapling => "minecraft:potted_jungle_sapling",
                    PottedPlant::LilyOfTheValley => "minecraft:potted_lily_of_the_valley",
                    PottedPlant::OakSapling => "minecraft:potted_oak_sapling",
                    PottedPlant::OxeyeDaisy => "minecraft:potted_oxeye_daisy",
                    PottedPlant::Poppy => "minecraft:potted_poppy",
                    PottedPlant::RedMushroom => "minecraft:potted_red_mushroom",
                    PottedPlant::SpruceSapling => "minecraft:potted_spruce_sapling",
                    PottedPlant::TulipOrange => "minecraft:potted_orange_tulip",
                    PottedPlant::TulipPink => "minecraft:potted_pink_tulip",
                    PottedPlant::TulipRed => "minecraft:potted_red_tulip",
                    PottedPlant::TulipWhite => "minecraft:potted_white_tulip",
                    PottedPlant::WarpedFungus => "minecraft:potted_warped_fungus",
                    PottedPlant::WarpedRoots => "minecraft:potted_warped_roots",
                    PottedPlant::WitherRose => "minecraft:potted_wither_rose",
                }
            }
            PaletteItem::Block(Block::Head(Head { variant, placement, .. })) => match placement {
                WallOrRotatedOnFloor::Floor(_) => match variant {
                    HeadVariant::CreeperHead => "minecraft:creeper_head",
                    HeadVariant::DragonHead => "minecraft:dragon_head",
                    HeadVariant::PlayerHead => "minecraft:player_head",
                    HeadVariant::SkeletonSkull => "minecraft:skeleton_skull",
                    HeadVariant::WitherSkeletonSkull => "minecraft:wither_skeleton_skull",
                    HeadVariant::ZombieHead => "minecraft:zombie_head",
                }
                WallOrRotatedOnFloor::Wall(_) => match variant {
                    HeadVariant::CreeperHead => "minecraft:creeper_wall_head",
                    HeadVariant::DragonHead => "minecraft:dragon_wall_head",
                    HeadVariant::PlayerHead => "minecraft:player_wall_head",
                    HeadVariant::SkeletonSkull => "minecraft:skeleton_wall_skull",
                    HeadVariant::WitherSkeletonSkull => "minecraft:wither_skeleton_wall_skull",
                    HeadVariant::ZombieHead => "minecraft:zombie_wall_head",
                }
            }
            PaletteItem::Block(Block::Anvil { damage, .. }) => match damage {
               AnvilDamage::Intact => "minecraft:anvil",
               AnvilDamage::SlightlyDamaged => "minecraft:chipped_anvil",
               AnvilDamage::VeryDamaged => "minecraft:damaged_anvil",
            }
            PaletteItem::ProtoBlock(ProtoBlock::TrappedChest { .. }) => "minecraft:trapped_chest",
            PaletteItem::Block(Block::RedstoneComparator { .. })
            | PaletteItem::Block(Block::RedstoneSubtractor { .. }) => "minecraft:comparator",
            PaletteItem::Block(Block::DaylightDetector)
            | PaletteItem::Block(Block::InvertedDaylightDetector) => "minecraft:daylight_detector",
            PaletteItem::Block(Block::BlockOfRedstone) => "minecraft:redstone_block",
            PaletteItem::Block(Block::QuartzOre) => "minecraft:nether_quartz_ore",
            PaletteItem::ProtoBlock(ProtoBlock::Hopper { .. }) => "minecraft:hopper",
            PaletteItem::Block(Block::BlockOfQuartz) => "minecraft:quartz_block",
            PaletteItem::Block(Block::ChiseledQuartzBlock) => "minecraft:chiseled_quartz_block",
            PaletteItem::Block(Block::QuartzPillar { .. }) => "minecraft:quartz_pillar",
            PaletteItem::ProtoBlock(ProtoBlock::Dropper { .. }) => "minecraft:dropper",
            PaletteItem::Block(Block::Terracotta { colour }) => match colour {
                None => "minecraft:terracotta",
                Some(Colour::White) => "minecraft:white_terracotta",
                Some(Colour::Orange) => "minecraft:orange_terracotta",
                Some(Colour::Magenta) => "minecraft:magenta_terracotta",
                Some(Colour::LightBlue) => "minecraft:light_blue_terracotta",
                Some(Colour::Yellow) => "minecraft:yellow_terracotta",
                Some(Colour::Lime) => "minecraft:lime_terracotta",
                Some(Colour::Pink) => "minecraft:pink_terracotta",
                Some(Colour::Gray) => "minecraft:gray_terracotta",
                Some(Colour::LightGray) => "minecraft:light_gray_terracotta",
                Some(Colour::Cyan) => "minecraft:cyan_terracotta",
                Some(Colour::Purple) => "minecraft:purple_terracotta",
                Some(Colour::Blue) => "minecraft:blue_terracotta",
                Some(Colour::Brown) => "minecraft:brown_terracotta",
                Some(Colour::Green) => "minecraft:green_terracotta",
                Some(Colour::Red) => "minecraft:red_terracotta",
                Some(Colour::Black) => "minecraft:black_terracotta",
            }
            PaletteItem::Block(Block::BlockOfSlime) => "minecraft:slime_block",
            PaletteItem::Block(Block::Barrier) => "minecraft:barrier",
            PaletteItem::Block(Block::Prismarine) => "minecraft:prismarine",
            PaletteItem::Block(Block::PrismarineBricks) => "minecraft:prismarine_bricks",
            PaletteItem::Block(Block::DarkPrismarine) => "minecraft:dark_prismarine",
            PaletteItem::Block(Block::SeaLantern) => "minecraft:sea_lantern",
            PaletteItem::Block(Block::HayBale { .. }) => "minecraft:hay_block",
            PaletteItem::Block(Block::Carpet { colour }) => match colour {
                Colour::White => "minecraft:white_carpet",
                Colour::Orange => "minecraft:orange_carpet",
                Colour::Magenta => "minecraft:magenta_carpet",
                Colour::LightBlue => "minecraft:light_blue_carpet",
                Colour::Yellow => "minecraft:yellow_carpet",
                Colour::Lime => "minecraft:lime_carpet",
                Colour::Pink => "minecraft:pink_carpet",
                Colour::Gray => "minecraft:gray_carpet",
                Colour::LightGray => "minecraft:light_gray_carpet",
                Colour::Cyan => "minecraft:cyan_carpet",
                Colour::Purple => "minecraft:purple_carpet",
                Colour::Blue => "minecraft:blue_carpet",
                Colour::Brown => "minecraft:brown_carpet",
                Colour::Green => "minecraft:green_carpet",
                Colour::Red => "minecraft:red_carpet",
                Colour::Black => "minecraft:black_carpet",
            }
            PaletteItem::Block(Block::BlockOfCoal) => "minecraft:coal_block",
            PaletteItem::Block(Block::Bed(Bed { colour, .. })) => match colour {
                Colour::White => "minecraft:white_bed",
                Colour::Orange => "minecraft:orange_bed",
                Colour::Magenta => "minecraft:magenta_bed",
                Colour::LightBlue => "minecraft:light_blue_bed",
                Colour::Yellow => "minecraft:yellow_bed",
                Colour::Lime => "minecraft:lime_bed",
                Colour::Pink => "minecraft:pink_bed",
                Colour::Gray => "minecraft:gray_bed",
                Colour::LightGray => "minecraft:light_gray_bed",
                Colour::Cyan => "minecraft:cyan_bed",
                Colour::Purple => "minecraft:purple_bed",
                Colour::Blue => "minecraft:blue_bed",
                Colour::Brown => "minecraft:brown_bed",
                Colour::Green => "minecraft:green_bed",
                Colour::Red => "minecraft:red_bed",
                Colour::Black => "minecraft:black_bed",
            }
            PaletteItem::ProtoBlock(ProtoBlock::Banner { placement, colour, .. }) => match placement {
                WallOrRotatedOnFloor::Floor(_) => match colour {
                    Colour::White => "minecraft:white_banner",
                    Colour::Orange => "minecraft:orange_banner",
                    Colour::Magenta => "minecraft:magenta_banner",
                    Colour::LightBlue => "minecraft:light_blue_banner",
                    Colour::Yellow => "minecraft:yellow_banner",
                    Colour::Lime => "minecraft:lime_banner",
                    Colour::Pink => "minecraft:pink_banner",
                    Colour::Gray => "minecraft:gray_banner",
                    Colour::LightGray => "minecraft:light_gray_banner",
                    Colour::Cyan => "minecraft:cyan_banner",
                    Colour::Purple => "minecraft:purple_banner",
                    Colour::Blue => "minecraft:blue_banner",
                    Colour::Brown => "minecraft:brown_banner",
                    Colour::Green => "minecraft:green_banner",
                    Colour::Red => "minecraft:red_banner",
                    Colour::Black => "minecraft:black_banner",
                }
                WallOrRotatedOnFloor::Wall(_) => match colour {
                    Colour::White => "minecraft:white_wall_banner",
                    Colour::Orange => "minecraft:orange_wall_banner",
                    Colour::Magenta => "minecraft:magenta_wall_banner",
                    Colour::LightBlue => "minecraft:light_blue_wall_banner",
                    Colour::Yellow => "minecraft:yellow_wall_banner",
                    Colour::Lime => "minecraft:lime_wall_banner",
                    Colour::Pink => "minecraft:pink_wall_banner",
                    Colour::Gray => "minecraft:gray_wall_banner",
                    Colour::LightGray => "minecraft:light_gray_wall_banner",
                    Colour::Cyan => "minecraft:cyan_wall_banner",
                    Colour::Purple => "minecraft:purple_wall_banner",
                    Colour::Blue => "minecraft:blue_wall_banner",
                    Colour::Brown => "minecraft:brown_wall_banner",
                    Colour::Green => "minecraft:green_wall_banner",
                    Colour::Red => "minecraft:red_wall_banner",
                    Colour::Black => "minecraft:black_wall_banner",
                }
            }
            PaletteItem::Block(Block::RedSandstone) => "minecraft:red_sandstone",
            PaletteItem::Block(Block::ChiseledRedSandstone) => "minecraft:chiseled_red_sandstone",
            PaletteItem::Block(Block::SmoothRedSandstone) => "minecraft:smooth_red_sandstone",
            PaletteItem::Block(Block::CutRedSandstone) => "minecraft:cut_red_sandstone",
            PaletteItem::Block(Block::EndRod { .. }) => "minecraft:end_rod",
            PaletteItem::Block(Block::ChorusPlant) => "minecraft:chorus_plant",
            PaletteItem::Block(Block::ChorusFlower { .. }) => "minecraft:chorus_flower",
            PaletteItem::Block(Block::PurpurBlock) => "minecraft:purpur_block",
            PaletteItem::Block(Block::PurpurPillar { .. }) => "minecraft:purpur_pillar",
            PaletteItem::Block(Block::EndStoneBricks) => "minecraft:end_stone_bricks",
            PaletteItem::Block(Block::GrassPath) => "minecraft:grass_path",
            // TODO 209 EndGateway
            // TODO 210 repeating command block
            // TODO 211 chain command block
            // TODO 212 FrostedIce
            PaletteItem::Block(Block::MagmaBlock) => "minecraft:magma_block",
            PaletteItem::Block(Block::NetherWartBlock) => "minecraft:nether_wart_block",
            PaletteItem::Block(Block::WarpedWartBlock) => "minecraft:warped_wart_block",
            PaletteItem::Block(Block::RedNetherBricks) => "minecraft:red_nether_bricks",
            PaletteItem::Block(Block::BoneBlock { .. }) => "minecraft:bone_block",
            // TODO 217 StructureVoid
            PaletteItem::Block(Block::Observer { .. }) => "minecraft:observer",
            //PaletteItem::Block(Block::) => "minecraft:",

            PaletteItem::ProtoBlock(ProtoBlock::ShulkerBox { colour, .. }) => match colour {
                None => "minecraft:shulker_box",
                Some(Colour::White) => "minecraft:white_shulker_box",
                Some(Colour::Orange) => "minecraft:orange_shulker_box",
                Some(Colour::Magenta) => "minecraft:magenta_shulker_box",
                Some(Colour::LightBlue) => "minecraft:light_blue_shulker_box",
                Some(Colour::Yellow) => "minecraft:yellow_shulker_box",
                Some(Colour::Lime) => "minecraft:lime_shulker_box",
                Some(Colour::Pink) => "minecraft:pink_shulker_box",
                Some(Colour::Gray) => "minecraft:gray_shulker_box",
                Some(Colour::LightGray) => "minecraft:light_gray_shulker_box",
                Some(Colour::Cyan) => "minecraft:cyan_shulker_box",
                Some(Colour::Purple) => "minecraft:purple_shulker_box",
                Some(Colour::Blue) => "minecraft:blue_shulker_box",
                Some(Colour::Brown) => "minecraft:brown_shulker_box",
                Some(Colour::Green) => "minecraft:green_shulker_box",
                Some(Colour::Red) => "minecraft:red_shulker_box",
                Some(Colour::Black) => "minecraft:black_shulker_box",
            }
            PaletteItem::Block(Block::GlazedTerracotta(GlazedTerracotta { colour, .. })) => match colour {
                Colour::White => "minecraft:white_glazed_terracotta",
                Colour::Orange => "minecraft:orange_glazed_terracotta",
                Colour::Magenta => "minecraft:magenta_glazed_terracotta",
                Colour::LightBlue => "minecraft:light_blue_glazed_terracotta",
                Colour::Yellow => "minecraft:yellow_glazed_terracotta",
                Colour::Lime => "minecraft:lime_glazed_terracotta",
                Colour::Pink => "minecraft:pink_glazed_terracotta",
                Colour::Gray => "minecraft:gray_glazed_terracotta",
                Colour::LightGray => "minecraft:light_gray_glazed_terracotta",
                Colour::Cyan => "minecraft:cyan_glazed_terracotta",
                Colour::Purple => "minecraft:purple_glazed_terracotta",
                Colour::Blue => "minecraft:blue_glazed_terracotta",
                Colour::Brown => "minecraft:brown_glazed_terracotta",
                Colour::Green => "minecraft:green_glazed_terracotta",
                Colour::Red => "minecraft:red_glazed_terracotta",
                Colour::Black => "minecraft:black_glazed_terracotta",
            }
            PaletteItem::Block(Block::Concrete { colour }) => match colour {
                Colour::White => "minecraft:white_concrete",
                Colour::Orange => "minecraft:orange_concrete",
                Colour::Magenta => "minecraft:magenta_concrete",
                Colour::LightBlue => "minecraft:light_blue_concrete",
                Colour::Yellow => "minecraft:yellow_concrete",
                Colour::Lime => "minecraft:lime_concrete",
                Colour::Pink => "minecraft:pink_concrete",
                Colour::Gray => "minecraft:gray_concrete",
                Colour::LightGray => "minecraft:light_gray_concrete",
                Colour::Cyan => "minecraft:cyan_concrete",
                Colour::Purple => "minecraft:purple_concrete",
                Colour::Blue => "minecraft:blue_concrete",
                Colour::Brown => "minecraft:brown_concrete",
                Colour::Green => "minecraft:green_concrete",
                Colour::Red => "minecraft:red_concrete",
                Colour::Black => "minecraft:black_concrete",
            }
            PaletteItem::Block(Block::ConcretePowder { colour }) => match colour {
                Colour::White => "minecraft:white_concrete_powder",
                Colour::Orange => "minecraft:orange_concrete_powder",
                Colour::Magenta => "minecraft:magenta_concrete_powder",
                Colour::LightBlue => "minecraft:light_blue_concrete_powder",
                Colour::Yellow => "minecraft:yellow_concrete_powder",
                Colour::Lime => "minecraft:lime_concrete_powder",
                Colour::Pink => "minecraft:pink_concrete_powder",
                Colour::Gray => "minecraft:gray_concrete_powder",
                Colour::LightGray => "minecraft:light_gray_concrete_powder",
                Colour::Cyan => "minecraft:cyan_concrete_powder",
                Colour::Purple => "minecraft:purple_concrete_powder",
                Colour::Blue => "minecraft:blue_concrete_powder",
                Colour::Brown => "minecraft:brown_concrete_powder",
                Colour::Green => "minecraft:green_concrete_powder",
                Colour::Red => "minecraft:red_concrete_powder",
                Colour::Black => "minecraft:black_concrete_powder",
            }
            // TODO 255 structure block
            PaletteItem::Block(Block::CrimsonNylium) => "minecraft:crimson_nylium",
            PaletteItem::Block(Block::WarpedNylium) => "minecraft:warped_nylium",
            PaletteItem::Block(Block::NetherGoldOre) => "minecraft:nether_gold_ore",
            PaletteItem::Block(Block::Seagrass { variant }) => match variant {
                Seagrass::Seagrass => "minecraft:seagrass",
                Seagrass::TallSeagrassBottom
                | Seagrass::TallSeagrassTop => "minecraft:tall_seagrass",
            }
            PaletteItem::Block(Block::SeaPickle { .. }) => "minecraft:sea_pickle",
            PaletteItem::Block(Block::Kelp { .. }) => "minecraft:kelp",
            PaletteItem::Block(Block::KelpPlant) => "minecraft:kelp_plant",
            PaletteItem::Block(Block::Basalt { .. }) => "minecraft:basalt",
            PaletteItem::Block(Block::PolishedBasalt { .. }) => "minecraft:polished_basalt",
            PaletteItem::Block(Block::QuartzBricks) => "minecraft:quartz_bricks",
            PaletteItem::Block(Block::Chain { .. }) => "minecraft:chain",
            PaletteItem::Block(Block::TurtleEgg { .. }) => "minecraft:turtle_egg",
            PaletteItem::Block(Block::DriedKelpBlock) => "minecraft:dried_kelp_block",
            PaletteItem::Block(Block::Scaffolding { .. }) => "minecraft:scaffolding",

            PaletteItem::Block(Block::Coral { material, dead: false, .. }) => match material {
                CoralMaterial::Bubble => "minecraft:bubble_coral",
                CoralMaterial::Brain => "minecraft:brain_coral",
                CoralMaterial::Fire => "minecraft:fire_coral",
                CoralMaterial::Horn => "minecraft:horn_coral",
                CoralMaterial::Tube => "minecraft:tube_coral",
            }
            PaletteItem::Block(Block::Coral { material, dead: true, .. }) => match material {
                CoralMaterial::Bubble => "minecraft:dead_bubble_coral",
                CoralMaterial::Brain => "minecraft:dead_brain_coral",
                CoralMaterial::Fire => "minecraft:dead_fire_coral",
                CoralMaterial::Horn => "minecraft:dead_horn_coral",
                CoralMaterial::Tube => "minecraft:dead_tube_coral",
            }
            PaletteItem::Block(Block::CoralBlock { material, dead: false }) => match material {
                CoralMaterial::Bubble => "minecraft:bubble_coral_block",
                CoralMaterial::Brain => "minecraft:brain_coral_block",
                CoralMaterial::Fire => "minecraft:fire_coral_block",
                CoralMaterial::Horn => "minecraft:horn_coral_block",
                CoralMaterial::Tube => "minecraft:tube_coral_block",
            }
            PaletteItem::Block(Block::CoralBlock { material, dead: true }) => match material {
                CoralMaterial::Bubble => "minecraft:dead_bubble_coral_block",
                CoralMaterial::Brain => "minecraft:dead_brain_coral_block",
                CoralMaterial::Fire => "minecraft:dead_fire_coral_block",
                CoralMaterial::Horn => "minecraft:dead_horn_coral_block",
                CoralMaterial::Tube => "minecraft:dead_tube_coral_block",
            }
            PaletteItem::Block(Block::CoralFan { material, dead: false, facing, .. }) => match facing {
                Surface5::Down => match material {
                    CoralMaterial::Bubble => "minecraft:bubble_coral_fan",
                    CoralMaterial::Brain => "minecraft:brain_coral_fan",
                    CoralMaterial::Fire => "minecraft:fire_coral_fan",
                    CoralMaterial::Horn => "minecraft:horn_coral_fan",
                    CoralMaterial::Tube => "minecraft:tube_coral_fan",
                }
                _facing => match material {
                    CoralMaterial::Bubble => "minecraft:bubble_coral_wall_fan",
                    CoralMaterial::Brain => "minecraft:brain_coral_wall_fan",
                    CoralMaterial::Fire => "minecraft:fire_coral_wall_fan",
                    CoralMaterial::Horn => "minecraft:horn_coral_wall_fan",
                    CoralMaterial::Tube => "minecraft:tube_coral_wall_fan",
                }
            }
            PaletteItem::Block(Block::CoralFan { material, dead: true, facing, .. }) => match facing {
                Surface5::Down => match material {
                    CoralMaterial::Bubble => "minecraft:dead_bubble_coral_fan",
                    CoralMaterial::Brain => "minecraft:dead_brain_coral_fan",
                    CoralMaterial::Fire => "minecraft:dead_fire_coral_fan",
                    CoralMaterial::Horn => "minecraft:dead_horn_coral_fan",
                    CoralMaterial::Tube => "minecraft:dead_tube_coral_fan",
                }
                _facing => match material {
                    CoralMaterial::Bubble => "minecraft:dead_bubble_coral_wall_fan",
                    CoralMaterial::Brain => "minecraft:dead_brain_coral_wall_fan",
                    CoralMaterial::Fire => "minecraft:dead_fire_coral_wall_fan",
                    CoralMaterial::Horn => "minecraft:dead_horn_coral_wall_fan",
                    CoralMaterial::Tube => "minecraft:dead_tube_coral_wall_fan",
                }
            }
            PaletteItem::Block(Block::CartographyTable) => "minecraft:cartography_table",
            PaletteItem::Block(Block::SmithingTable) => "minecraft:smithing_table",
            PaletteItem::Block(Block::FletchingTable) => "minecraft:fletching_table",
            PaletteItem::Block(Block::Shroomlight) => "minecraft:shroomlight",
            PaletteItem::Block(Block::HoneyBlock) => "minecraft:honey_block",
            PaletteItem::Block(Block::HoneycombBlock) => "minecraft:honeycomb_block",
            PaletteItem::Block(Block::Lodestone) => "minecraft:lodestone",
            PaletteItem::Block(Block::BlockOfNetherite) => "minecraft:netherite_block",
            PaletteItem::Block(Block::AncientDebris) => "minecraft:ancient_debris",
            PaletteItem::Block(Block::Blackstone) => "minecraft:blackstone",
            PaletteItem::Block(Block::GildedBlackstone) => "minecraft:gilded_blackstone",
            PaletteItem::Block(Block::PolishedBlackstone) => "minecraft:polished_blackstone",
            PaletteItem::Block(Block::ChiseledPolishedBlackstone) => "minecraft:chiseled_polished_blackstone",
            PaletteItem::Block(Block::PolishedBlackstoneBricks) => "minecraft:polished_blackstone_bricks",
            PaletteItem::Block(Block::CrackedPolishedBlackstoneBricks) => "minecraft:cracked_polished_blackstone_bricks",
            PaletteItem::Block(Block::Loom { .. }) => "minecraft:loom",
            PaletteItem::Block(Block::Stonecutter { .. }) => "minecraft:stonecutter",
            PaletteItem::Block(Block::Lantern { .. }) => "minecraft:lantern",
            PaletteItem::Block(Block::SoulLantern { .. }) => "minecraft:soul_lantern",
            PaletteItem::Block(Block::Composter { .. }) => "minecraft:composter",
            PaletteItem::Block(Block::Grindstone(_)) => "minecraft:grindstone",
            PaletteItem::Block(Block::SweetBerryBush { .. }) => "minecraft:sweet_berry_bush",
            PaletteItem::Block(Block::RespawnAnchor { .. }) => "minecraft:respawn_anchor",
            PaletteItem::Block(Block::Target) => "minecraft:target",
            PaletteItem::Block(Block::WarpedFungus) => "minecraft:warped_fungus",
            PaletteItem::Block(Block::CrimsonFungus) => "minecraft:crimson_fungus",
            PaletteItem::Block(Block::WarpedRoots) => "minecraft:warped_roots",
            PaletteItem::Block(Block::CrimsonRoots) => "minecraft:crimson_roots",
            PaletteItem::Block(Block::Campfire { .. }) => "minecraft:campfire",
            PaletteItem::Block(Block::SoulCampfire { .. }) => "minecraft:soul_campfire",
            PaletteItem::ProtoBlock(ProtoBlock::Barrel { .. }) => "minecraft:barrel",
            //PaletteItem::Block(Block::) => "minecraft:",

            /*
                // Missing blocks with tile entity (and possibly also properties)
                smoker
                blast_furnace

                lectern
                conduit
                bell
                beehive
                bee_nest (beehive)

                // Missing plant classes:
                // TwistingVines & TwistingVinesPlant
                // WeepingVines & WeepingVinesPlant
                // Bamboo & BambooPlant
                // -- They are probably just like Kelp and KelpPlant,
                //    in that the "plant" variant is the full-grown variant.
            */

            // Blocks that should only appear as proto blocks
            PaletteItem::Block(Block::Banner(_))
            | PaletteItem::Block(Block::Barrel(_))
            | PaletteItem::Block(Block::Beacon(_))
            | PaletteItem::Block(Block::BrewingStand(_))
            | PaletteItem::Block(Block::Chest(_))
            | PaletteItem::Block(Block::Dispenser(_))
            | PaletteItem::Block(Block::Dropper(_))
            | PaletteItem::Block(Block::EnchantingTable { .. })
            | PaletteItem::Block(Block::Furnace(_))
            | PaletteItem::Block(Block::Hopper(_))
            | PaletteItem::Block(Block::ShulkerBox(_))
            | PaletteItem::Block(Block::Sign(_))
            | PaletteItem::Block(Block::TrappedChest(_)) => {
                warn!("Unexpected PaletteItem::Block variant: {:?}", self);
                "minecraft:sponge"
            }

            // Blocks that should not appear as proto blocks
            PaletteItem::ProtoBlock(ProtoBlock::Jukebox) => {
                warn!("Unexpected PaletteItem::ProtoBlock variant: {:?}", self);
                "minecraft:sponge"
            }

            // Catch-all // TODO remove when all variants are handled!
            _ => "minecraft:sponge", // TODO handle all variants!
        }
    }
}

/// From a section NBT value, generate a palette in the form of a vector of PaletteItems.
pub(super) fn from_section(section: &nbt::Value) -> Option<Vec<PaletteItem>> {
    // Import Palette. It contains a list of compounds, each with a Name:String (Namespaced block ID)
    // and optionally a Properties:Compound which contains pairs of Name:String, value (for
    // e.g. facing.) Essentially holding all info previously encoded in blocks + add + data.
    let mut palette: Vec<PaletteItem> = Vec::new();
    let raw_palette = if let Ok(p) = nbt_value_lookup_list(section, "Palette") { p } else { return None };

//    println!("Raw palette: {:#?}", raw_palette);
    for raw_block in raw_palette {
        let name = nbt_value_lookup_string(&raw_block, "Name").unwrap();
        let properties = nbt_value_lookup(&raw_block, "Properties").ok();

        // Source for (hopefully exhaustive) list of IDs: https://minecraftitemids.com
        let palette_item = match name.as_str() {
            "minecraft:air" => block(Block::Air),
            "minecraft:cave_air" => block(Block::CaveAir),
            "minecraft:stone" => block(Block::Stone),
            "minecraft:granite" => block(Block::Granite),
            "minecraft:polished_granite" => block(Block::PolishedGranite),
            "minecraft:diorite" => block(Block::Diorite),
            "minecraft:polished_diorite" => block(Block::PolishedDiorite),
            "minecraft:andesite" => block(Block::Andesite),
            "minecraft:polished_andesite" => block(Block::PolishedAndesite),
            "minecraft:grass_block" => block(Block::GrassBlock),
            "minecraft:dirt" => block(Block::Dirt),
            "minecraft:coarse_dirt" => block(Block::CoarseDirt),
            "minecraft:podzol" => block(Block::Podzol),
            "minecraft:cobblestone" => block(Block::Cobblestone),
            "minecraft:oak_planks" => block(Block::Planks { material: WoodMaterial::Oak }),
            "minecraft:spruce_planks" => block(Block::Planks { material: WoodMaterial::Spruce }),
            "minecraft:birch_planks" => block(Block::Planks { material: WoodMaterial::Birch }),
            "minecraft:jungle_planks" => block(Block::Planks { material: WoodMaterial::Jungle }),
            "minecraft:acacia_planks" => block(Block::Planks { material: WoodMaterial::Acacia }),
            "minecraft:dark_oak_planks" => block(Block::Planks { material: WoodMaterial::DarkOak }),
            "minecraft:crimson_planks" => block(Block::Planks { material: WoodMaterial::Crimson }),
            "minecraft:warped_planks" => block(Block::Planks { material: WoodMaterial::Warped }),
            "minecraft:oak_sapling" => block(sapling(SaplingMaterial::Oak, &properties)),
            "minecraft:spruce_sapling" => block(sapling(SaplingMaterial::Spruce, &properties)),
            "minecraft:birch_sapling" => block(sapling(SaplingMaterial::Birch, &properties)),
            "minecraft:jungle_sapling" => block(sapling(SaplingMaterial::Jungle, &properties)),
            "minecraft:acacia_sapling" => block(sapling(SaplingMaterial::Acacia, &properties)),
            "minecraft:dark_oak_sapling" => block(sapling(SaplingMaterial::DarkOak, &properties)),
            "minecraft:bamboo_sapling" => block(sapling(SaplingMaterial::Bamboo, &properties)),
            "minecraft:bedrock" => block(Block::Bedrock),
            "minecraft:water" => block(water(&properties)),
            "minecraft:lava" => block(lava(&properties)),
            "minecraft:sand" => block(Block::Sand),
            "minecraft:red_sand" => block(Block::RedSand),
            "minecraft:gravel" => block(Block::Gravel),
            "minecraft:gold_ore" => block(Block::GoldOre),
            "minecraft:iron_ore" => block(Block::IronOre),
            "minecraft:coal_ore" => block(Block::CoalOre),
            "minecraft:oak_log" => block(log(WoodMaterial::Oak, &properties)),
            "minecraft:spruce_log" => block(log(WoodMaterial::Spruce, &properties)),
            "minecraft:birch_log" => block(log(WoodMaterial::Birch, &properties)),
            "minecraft:jungle_log" => block(log(WoodMaterial::Jungle, &properties)),
            "minecraft:acacia_log" => block(log(WoodMaterial::Acacia, &properties)),
            "minecraft:dark_oak_log" => block(log(WoodMaterial::DarkOak, &properties)),
            "minecraft:crimson_stem" => block(log(WoodMaterial::Crimson, &properties)),
            "minecraft:warped_stem" => block(log(WoodMaterial::Warped, &properties)),
            "minecraft:stripped_oak_log" => block(stripped_log(WoodMaterial::Oak, &properties)),
            "minecraft:stripped_spruce_log" => block(stripped_log(WoodMaterial::Spruce, &properties)),
            "minecraft:stripped_birch_log" => block(stripped_log(WoodMaterial::Birch, &properties)),
            "minecraft:stripped_jungle_log" => block(stripped_log(WoodMaterial::Jungle, &properties)),
            "minecraft:stripped_acacia_log" => block(stripped_log(WoodMaterial::Acacia, &properties)),
            "minecraft:stripped_dark_oak_log" => block(stripped_log(WoodMaterial::DarkOak, &properties)),
            "minecraft:stripped_crimson_stem" => block(stripped_log(WoodMaterial::Crimson, &properties)),
            "minecraft:stripped_warped_stem" => block(stripped_log(WoodMaterial::Warped, &properties)),
            "minecraft:oak_wood" => block(wood(WoodMaterial::Oak, &properties)),
            "minecraft:spruce_wood" => block(wood(WoodMaterial::Spruce, &properties)),
            "minecraft:birch_wood" => block(wood(WoodMaterial::Birch, &properties)),
            "minecraft:jungle_wood" => block(wood(WoodMaterial::Jungle, &properties)),
            "minecraft:acacia_wood" => block(wood(WoodMaterial::Acacia, &properties)),
            "minecraft:dark_oak_wood" => block(wood(WoodMaterial::DarkOak, &properties)),
            "minecraft:crimson_hyphae" => block(wood(WoodMaterial::Crimson, &properties)),
            "minecraft:warped_hyphae" => block(wood(WoodMaterial::Warped, &properties)),
            "minecraft:stripped_oak_wood" => block(stripped_wood(WoodMaterial::Oak, &properties)),
            "minecraft:stripped_spruce_wood" => block(stripped_wood(WoodMaterial::Spruce, &properties)),
            "minecraft:stripped_birch_wood" => block(stripped_wood(WoodMaterial::Birch, &properties)),
            "minecraft:stripped_jungle_wood" => block(stripped_wood(WoodMaterial::Jungle, &properties)),
            "minecraft:stripped_acacia_wood" => block(stripped_wood(WoodMaterial::Acacia, &properties)),
            "minecraft:stripped_dark_oak_wood" => block(stripped_wood(WoodMaterial::DarkOak, &properties)),
            "minecraft:stripped_crimson_hyphae" => block(stripped_wood(WoodMaterial::Crimson, &properties)),
            "minecraft:stripped_warped_hyphae" => block(stripped_wood(WoodMaterial::Warped, &properties)),
            "minecraft:oak_leaves" => block(leaves(LeavesMaterial::Oak, &properties)),
            "minecraft:spruce_leaves" => block(leaves(LeavesMaterial::Spruce, &properties)),
            "minecraft:birch_leaves" => block(leaves(LeavesMaterial::Birch, &properties)),
            "minecraft:jungle_leaves" => block(leaves(LeavesMaterial::Jungle, &properties)),
            "minecraft:acacia_leaves" => block(leaves(LeavesMaterial::Acacia, &properties)),
            "minecraft:dark_oak_leaves" => block(leaves(LeavesMaterial::DarkOak, &properties)),
            "minecraft:sponge" => block(Block::Sponge),
            "minecraft:wet_sponge" => block(Block::WetSponge),
            "minecraft:glass" => block(Block::Glass { colour: None }),
            "minecraft:lapis_ore" => block(Block::LapisLazuliOre),
            "minecraft:lapis_block" => block(Block::LapisLazuliBlock),
            "minecraft:dispenser" => proto(proto_dispenser(&properties)),
            "minecraft:sandstone" => block(Block::Sandstone),
            "minecraft:chiseled_sandstone" => block(Block::ChiseledSandstone),
            "minecraft:smooth_sandstone" => block(Block::SmoothSandstone),
            "minecraft:cut_sandstone" => block(Block::CutSandstone),
            "minecraft:note_block" => block(noteblock(&properties)),
            "minecraft:powered_rail" => block(rail(RailType::Powered, &properties)),
            "minecraft:detector_rail" => block(rail(RailType::Detector, &properties)),
            "minecraft:rail" => block(rail(RailType::Normal, &properties)),
            "minecraft:activator_rail" => block(rail(RailType::Activator, &properties)),
            "minecraft:sticky_piston" => block(piston(true, &properties)),
            "minecraft:piston_head" => block(piston_head(&properties)),
            "minecraft:piston" => block(piston(false, &properties)),
            "minecraft:cobweb" => block(Block::Cobweb),
            "minecraft:grass" => block(Block::Grass(Grass::Grass)),
            "minecraft:fern" => block(Block::Grass(Grass::Fern)),
            "minecraft:dead_bush" => block(Block::DeadBush),
            "minecraft:white_wool" => block(Block::Wool { colour: Colour::White }),
            "minecraft:orange_wool" => block(Block::Wool { colour: Colour::Orange }),
            "minecraft:magenta_wool" => block(Block::Wool { colour: Colour::Magenta }),
            "minecraft:light_blue_wool" => block(Block::Wool { colour: Colour::LightBlue }),
            "minecraft:yellow_wool" => block(Block::Wool { colour: Colour::Yellow }),
            "minecraft:lime_wool" => block(Block::Wool { colour: Colour::Lime }),
            "minecraft:pink_wool" => block(Block::Wool { colour: Colour::Pink }),
            "minecraft:gray_wool" => block(Block::Wool { colour: Colour::Gray }),
            "minecraft:light_gray_wool" => block(Block::Wool { colour: Colour::LightGray }),
            "minecraft:cyan_wool" => block(Block::Wool { colour: Colour::Cyan }),
            "minecraft:purple_wool" => block(Block::Wool { colour: Colour::Purple }),
            "minecraft:blue_wool" => block(Block::Wool { colour: Colour::Blue }),
            "minecraft:brown_wool" => block(Block::Wool { colour: Colour::Brown }),
            "minecraft:green_wool" => block(Block::Wool { colour: Colour::Green }),
            "minecraft:red_wool" => block(Block::Wool { colour: Colour::Red }),
            "minecraft:black_wool" => block(Block::Wool { colour: Colour::Black }),
            // TODO block 36 piston_extension ("Block moved by Piston")
            "minecraft:dandelion" => block(Block::Flower(Flower::Dandelion)),
            "minecraft:poppy" => block(Block::Flower(Flower::Poppy)),
            "minecraft:blue_orchid" => block(Block::Flower(Flower::BlueOrchid)),
            "minecraft:allium" => block(Block::Flower(Flower::Allium)),
            "minecraft:azure_bluet" => block(Block::Flower(Flower::AzureBluet)),
            "minecraft:red_tulip" => block(Block::Flower(Flower::TulipRed)),
            "minecraft:orange_tulip" => block(Block::Flower(Flower::TulipOrange)),
            "minecraft:white_tulip" => block(Block::Flower(Flower::TulipWhite)),
            "minecraft:pink_tulip" => block(Block::Flower(Flower::TulipPink)),
            "minecraft:oxeye_daisy" => block(Block::Flower(Flower::OxeyeDaisy)),
            "minecraft:cornflower" => block(Block::Flower(Flower::Cornflower)),
            "minecraft:lily_of_the_valley" => block(Block::Flower(Flower::LilyOfTheValley)),
            "minecraft:wither_rose" => block(Block::Flower(Flower::WitherRose)),
            "minecraft:brown_mushroom" => block(Block::BrownMushroom),
            "minecraft:red_mushroom" => block(Block::RedMushroom),
            "minecraft:gold_block" => block(Block::BlockOfGold),
            "minecraft:iron_block" => block(Block::BlockOfIron),
            "minecraft:oak_slab" => block(slab(SlabMaterial::Oak, &properties)),
            "minecraft:spruce_slab" => block(slab(SlabMaterial::Spruce, &properties)),
            "minecraft:birch_slab" => block(slab(SlabMaterial::Birch, &properties)),
            "minecraft:jungle_slab" => block(slab(SlabMaterial::Jungle, &properties)),
            "minecraft:acacia_slab" => block(slab(SlabMaterial::Acacia, &properties)),
            "minecraft:dark_oak_slab" => block(slab(SlabMaterial::DarkOak, &properties)),
            "minecraft:crimson_slab" => block(slab(SlabMaterial::Crimson, &properties)),
            "minecraft:warped_slab" => block(slab(SlabMaterial::Warped, &properties)),
            "minecraft:stone_slab" => block(slab(SlabMaterial::Stone, &properties)),
            "minecraft:smooth_stone_slab" => block(slab(SlabMaterial::SmoothStone, &properties)),
            "minecraft:sandstone_slab" => block(slab(SlabMaterial::Sandstone, &properties)),
            "minecraft:petrified_oak_slab" => block(slab(SlabMaterial::PetrifiedOak, &properties)),
            "minecraft:cobblestone_slab" => block(slab(SlabMaterial::Cobblestone, &properties)),
            "minecraft:brick_slab" => block(slab(SlabMaterial::Brick, &properties)),
            "minecraft:stone_brick_slab" => block(slab(SlabMaterial::StoneBrick, &properties)),
            "minecraft:nether_brick_slab" => block(slab(SlabMaterial::NetherBrick, &properties)),
            "minecraft:quartz_slab" => block(slab(SlabMaterial::Quartz, &properties)),
            "minecraft:red_sandstone_slab" => block(slab(SlabMaterial::RedSandstone, &properties)),
            "minecraft:purpur_slab" => block(slab(SlabMaterial::Purpur, &properties)),
            "minecraft:prismarine_slab" => block(slab(SlabMaterial::Prismarine, &properties)),
            "minecraft:prismarine_brick_slab" => block(slab(SlabMaterial::PrismarineBrick, &properties)),
            "minecraft:dark_prismarine_slab" => block(slab(SlabMaterial::DarkPrismarine, &properties)),
            "minecraft:andesite_slab" => block(slab(SlabMaterial::Andesite, &properties)),
            "minecraft:diorite_slab" => block(slab(SlabMaterial::Diorite, &properties)),
            "minecraft:granite_slab" => block(slab(SlabMaterial::Granite, &properties)),
            "minecraft:polished_andesite_slab" => block(slab(SlabMaterial::PolishedAndesite, &properties)),
            "minecraft:polished_diorite_slab" => block(slab(SlabMaterial::PolishedDiorite, &properties)),
            "minecraft:polished_granite_slab" => block(slab(SlabMaterial::PolishedGranite, &properties)),
            "minecraft:cut_sandstone_slab" => block(slab(SlabMaterial::CutSandstone, &properties)),
            "minecraft:cut_red_sandstone_slab" => block(slab(SlabMaterial::CutRedSandstone, &properties)),
            "minecraft:smooth_sandstone_slab" => block(slab(SlabMaterial::SmoothSandstone, &properties)),
            "minecraft:smooth_red_sandstone_slab" => block(slab(SlabMaterial::SmoothRedSandstone, &properties)),
            "minecraft:smooth_quartz_slab" => block(slab(SlabMaterial::SmoothQuartz, &properties)),
            "minecraft:red_nether_brick_slab" => block(slab(SlabMaterial::RedNetherBrick, &properties)),
            "minecraft:end_stone_brick_slab" => block(slab(SlabMaterial::EndStoneBrick, &properties)),
            "minecraft:mossy_cobblestone_slab" => block(slab(SlabMaterial::MossyCobblestone, &properties)),
            "minecraft:mossy_stone_brick_slab" => block(slab(SlabMaterial::MossyStoneBrick, &properties)),
            "minecraft:blackstone_slab" => block(slab(SlabMaterial::Blackstone, &properties)),
            "minecraft:polished_blackstone_slab" => block(slab(SlabMaterial::PolishedBlackstone, &properties)),
            "minecraft:polished_blackstone_brick_slab" => block(slab(SlabMaterial::PolishedBlackstoneBrick, &properties)),
            "minecraft:smooth_quartz" => block(Block::SmoothQuartz),
            "minecraft:smooth_stone" => block(Block::SmoothStone),
            "minecraft:bricks" => block(Block::BrickBlock),
            "minecraft:tnt" => block(Block::TNT),
            "minecraft:bookshelf" => block(Block::Bookshelf),
            "minecraft:mossy_cobblestone" => block(Block::MossyCobblestone),
            "minecraft:obsidian" => block(Block::Obsidian),
            "minecraft:crying_obsidian" => block(Block::CryingObsidian),
            "minecraft:torch" => block(Block::Torch { attached: Surface5::Down }),
            "minecraft:wall_torch" => block(wall_torch(&properties)),
            "minecraft:redstone_torch" => block(Block::RedstoneTorch { attached: Surface5::Down }),
            "minecraft:redstone_wall_torch" => block(redstone_wall_torch(&properties)),
            "minecraft:soul_torch" => block(Block::SoulTorch { attached: Surface5::Down }),
            "minecraft:soul_wall_torch" => block(soul_wall_torch(&properties)),
            "minecraft:fire" => block(fire(&properties)),
            // TODO block 52 / minecraft:spawner / mob spawner
            "minecraft:oak_stairs" => block(stairs(StairMaterial::Oak, &properties)),
            "minecraft:spruce_stairs" => block(stairs(StairMaterial::Spruce, &properties)),
            "minecraft:birch_stairs" => block(stairs(StairMaterial::Birch, &properties)),
            "minecraft:jungle_stairs" => block(stairs(StairMaterial::Jungle, &properties)),
            "minecraft:acacia_stairs" => block(stairs(StairMaterial::Acacia, &properties)),
            "minecraft:dark_oak_stairs" => block(stairs(StairMaterial::DarkOak, &properties)),
            "minecraft:cobblestone_stairs" => block(stairs(StairMaterial::Cobblestone, &properties)),
            "minecraft:brick_stairs" => block(stairs(StairMaterial::Brick, &properties)),
            "minecraft:stone_brick_stairs" => block(stairs(StairMaterial::StoneBrick, &properties)),
            "minecraft:nether_brick_stairs" => block(stairs(StairMaterial::NetherBrick, &properties)),
            "minecraft:sandstone_stairs" => block(stairs(StairMaterial::Sandstone, &properties)),
            "minecraft:quartz_stairs" => block(stairs(StairMaterial::Quartz, &properties)),
            "minecraft:red_sandstone_stairs" => block(stairs(StairMaterial::RedSandstone, &properties)),
            "minecraft:purpur_stairs" => block(stairs(StairMaterial::Purpur, &properties)),
            "minecraft:dark_prismarine_stairs" => block(stairs(StairMaterial::DarkPrismarine, &properties)),
            "minecraft:smooth_sandstone_stairs" => block(stairs(StairMaterial::SmoothSandstone, &properties)),
            "minecraft:polished_blackstone_brick_stairs" => block(stairs(StairMaterial::PolishedBlackstoneBrick, &properties)),
            "minecraft:prismarine_brick_stairs" => block(stairs(StairMaterial::PrismarineBrick, &properties)),
            "minecraft:stone_stairs" => block(stairs(StairMaterial::Stone, &properties)),
            "minecraft:polished_blackstone_stairs" => block(stairs(StairMaterial::PolishedBlackstone, &properties)),
            "minecraft:prismarine_stairs" => block(stairs(StairMaterial::Prismarine, &properties)),
            "minecraft:end_stone_brick_stairs" => block(stairs(StairMaterial::EndStoneBrick, &properties)),
            "minecraft:blackstone_stairs" => block(stairs(StairMaterial::Blackstone, &properties)),
            "minecraft:mossy_cobblestone_stairs" => block(stairs(StairMaterial::MossyCobblestone, &properties)),
            "minecraft:diorite_stairs" => block(stairs(StairMaterial::Diorite, &properties)),
            "minecraft:polished_diorite_stairs" => block(stairs(StairMaterial::PolishedDiorite, &properties)),
            "minecraft:polished_andesite_stairs" => block(stairs(StairMaterial::PolishedAndesite, &properties)),
            "minecraft:mossy_stone_brick_stairs" => block(stairs(StairMaterial::MossyStoneBrick, &properties)),
            "minecraft:red_nether_brick_stairs" => block(stairs(StairMaterial::RedNetherBrick, &properties)),
            "minecraft:warped_stairs" => block(stairs(StairMaterial::Warped, &properties)),
            "minecraft:smooth_red_sandstone_stairs" => block(stairs(StairMaterial::SmoothRedSandstone, &properties)),
            "minecraft:andesite_stairs" => block(stairs(StairMaterial::Andesite, &properties)),
            "minecraft:crimson_stairs" => block(stairs(StairMaterial::Crimson, &properties)),
            "minecraft:polished_granite_stairs" => block(stairs(StairMaterial::PolishedGranite, &properties)),
            "minecraft:granite_stairs" => block(stairs(StairMaterial::Granite, &properties)),
            "minecraft:smooth_quartz_stairs" => block(stairs(StairMaterial::SmoothQuartz, &properties)),
            "minecraft:chest" => proto(proto_chest(&properties)),
            "minecraft:redstone_wire" => block(Block::RedstoneWire),
            "minecraft:diamond_ore" => block(Block::DiamondOre),
            "minecraft:diamond_block" => block(Block::BlockOfDiamond),
            "minecraft:crafting_table" => block(Block::CraftingTable),
            "minecraft:wheat" => block(Block::Wheat { growth_stage: age0_7(&properties) }),
            "minecraft:farmland" => block(Block::Farmland { wetness: moisture0_7(&properties) }),
            "minecraft:furnace" => proto(proto_furnace(&properties)),
            "minecraft:oak_sign" => proto(proto_sign(WoodMaterial::Oak, &properties)),
            "minecraft:oak_wall_sign" => proto(proto_wall_sign(WoodMaterial::Oak, &properties)),
            "minecraft:spruce_sign" => proto(proto_sign(WoodMaterial::Spruce, &properties)),
            "minecraft:spruce_wall_sign" => proto(proto_wall_sign(WoodMaterial::Spruce, &properties)),
            "minecraft:birch_sign" => proto(proto_sign(WoodMaterial::Birch, &properties)),
            "minecraft:birch_wall_sign" => proto(proto_wall_sign(WoodMaterial::Birch, &properties)),
            "minecraft:jungle_sign" => proto(proto_sign(WoodMaterial::Jungle, &properties)),
            "minecraft:jungle_wall_sign" => proto(proto_wall_sign(WoodMaterial::Jungle, &properties)),
            "minecraft:acacia_sign" => proto(proto_sign(WoodMaterial::Acacia, &properties)),
            "minecraft:acacia_wall_sign" => proto(proto_wall_sign(WoodMaterial::Acacia, &properties)),
            "minecraft:dark_oak_sign" => proto(proto_sign(WoodMaterial::DarkOak, &properties)),
            "minecraft:dark_oak_wall_sign" => proto(proto_wall_sign(WoodMaterial::DarkOak, &properties)),
            "minecraft:crimson_sign" => proto(proto_sign(WoodMaterial::Crimson, &properties)),
            "minecraft:crimson_wall_sign" => proto(proto_wall_sign(WoodMaterial::Crimson, &properties)),
            "minecraft:warped_sign" => proto(proto_sign(WoodMaterial::Warped, &properties)),
            "minecraft:warped_wall_sign" => proto(proto_wall_sign(WoodMaterial::Warped, &properties)),
            "minecraft:oak_door" => block(door(DoorMaterial::Oak, &properties)),
            "minecraft:spruce_door" => block(door(DoorMaterial::Spruce, &properties)),
            "minecraft:birch_door" => block(door(DoorMaterial::Birch, &properties)),
            "minecraft:jungle_door" => block(door(DoorMaterial::Jungle, &properties)),
            "minecraft:acacia_door" => block(door(DoorMaterial::Acacia, &properties)),
            "minecraft:dark_oak_door" => block(door(DoorMaterial::DarkOak, &properties)),
            "minecraft:crimson_door" => block(door(DoorMaterial::Crimson, &properties)),
            "minecraft:warped_door" => block(door(DoorMaterial::Warped, &properties)),
            "minecraft:iron_door" => block(door(DoorMaterial::Iron, &properties)),
            "minecraft:ladder" => block(ladder(&properties)),
            "minecraft:lever" => block(lever(&properties)),
            "minecraft:oak_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Oak)),
            "minecraft:spruce_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Spruce)),
            "minecraft:birch_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Birch)),
            "minecraft:jungle_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Jungle)),
            "minecraft:acacia_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Acacia)),
            "minecraft:dark_oak_pressure_plate" => block(pressure_plate(PressurePlateMaterial::DarkOak)),
            "minecraft:crimson_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Crimson)),
            "minecraft:warped_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Warped)),
            "minecraft:stone_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Stone)),
            "minecraft:polished_blackstone_pressure_plate" => block(pressure_plate(PressurePlateMaterial::PolishedBlackstone)),
            "minecraft:heavy_weighted_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Iron)),
            "minecraft:light_weighted_pressure_plate" => block(pressure_plate(PressurePlateMaterial::Gold)),
            "minecraft:redstone_ore" => block(Block::RedstoneOre),
            "minecraft:oak_button" => block(button(ButtonMaterial::Oak, &properties)),
            "minecraft:spruce_button" => block(button(ButtonMaterial::Spruce, &properties)),
            "minecraft:birch_button" => block(button(ButtonMaterial::Birch, &properties)),
            "minecraft:jungle_button" => block(button(ButtonMaterial::Jungle, &properties)),
            "minecraft:acacia_button" => block(button(ButtonMaterial::Acacia, &properties)),
            "minecraft:dark_oak_button" => block(button(ButtonMaterial::DarkOak, &properties)),
            "minecraft:crimson_button" => block(button(ButtonMaterial::Crimson, &properties)),
            "minecraft:warped_button" => block(button(ButtonMaterial::Warped, &properties)),
            "minecraft:stone_button" => block(button(ButtonMaterial::Stone, &properties)),
            "minecraft:polished_blackstone_button" => block(button(ButtonMaterial::PolishedBlackstone, &properties)),
            "minecraft:snow" => block(snow(&properties)),
            "minecraft:ice" => block(Block::Ice),
            "minecraft:packed_ice" => block(Block::PackedIce),
            "minecraft:blue_ice" => block(Block::BlueIce),
            "minecraft:snow_block" => block(Block::SnowBlock),
            "minecraft:cactus" => block(cactus(&properties)),
            "minecraft:clay" => block(Block::Clay),
            "minecraft:sugar_cane" => block(sugar_cane(&properties)),
            "minecraft:jukebox" => jukebox(&properties),
            "minecraft:oak_fence" => block(fence(FenceMaterial::Oak, &properties)),
            "minecraft:spruce_fence" => block(fence(FenceMaterial::Spruce, &properties)),
            "minecraft:birch_fence" => block(fence(FenceMaterial::Birch, &properties)),
            "minecraft:jungle_fence" => block(fence(FenceMaterial::Jungle, &properties)),
            "minecraft:acacia_fence" => block(fence(FenceMaterial::Acacia, &properties)),
            "minecraft:dark_oak_fence" => block(fence(FenceMaterial::DarkOak, &properties)),
            "minecraft:crimson_fence" => block(fence(FenceMaterial::Crimson, &properties)),
            "minecraft:warped_fence" => block(fence(FenceMaterial::Warped, &properties)),
            "minecraft:nether_brick_fence" => block(fence(FenceMaterial::NetherBrick, &properties)),
            "minecraft:pumpkin" => block(Block::Pumpkin),
            "minecraft:carved_pumpkin" => block(carved_pumpkin(&properties)),
            "minecraft:netherrack" => block(Block::Netherrack),
            "minecraft:soul_sand" => block(Block::SoulSand),
            "minecraft:soul_soil" => block(Block::SoulSoil),
            "minecraft:glowstone" => block(Block::Glowstone),
            "minecraft:nether_portal" => block(nether_portal(&properties)),
            "minecraft:jack_o_lantern" => block(jack_o_lantern(&properties)),
            "minecraft:cake" => block(cake(&properties)),
            "minecraft:repeater" => block(repeater(&properties)),
            "minecraft:white_stained_glass" => block(Block::Glass { colour: Some(Colour::White )}),
            "minecraft:orange_stained_glass" => block(Block::Glass { colour: Some(Colour::Orange )}),
            "minecraft:magenta_stained_glass" => block(Block::Glass { colour: Some(Colour::Magenta )}),
            "minecraft:light_blue_stained_glass" => block(Block::Glass { colour: Some(Colour::LightBlue )}),
            "minecraft:yellow_stained_glass" => block(Block::Glass { colour: Some(Colour::Yellow )}),
            "minecraft:lime_stained_glass" => block(Block::Glass { colour: Some(Colour::Lime )}),
            "minecraft:pink_stained_glass" => block(Block::Glass { colour: Some(Colour::Pink )}),
            "minecraft:gray_stained_glass" => block(Block::Glass { colour: Some(Colour::Gray )}),
            "minecraft:light_gray_stained_glass" => block(Block::Glass { colour: Some(Colour::LightGray )}),
            "minecraft:cyan_stained_glass" => block(Block::Glass { colour: Some(Colour::Cyan )}),
            "minecraft:purple_stained_glass" => block(Block::Glass { colour: Some(Colour::Purple )}),
            "minecraft:blue_stained_glass" => block(Block::Glass { colour: Some(Colour::Blue )}),
            "minecraft:brown_stained_glass" => block(Block::Glass { colour: Some(Colour::Brown )}),
            "minecraft:green_stained_glass" => block(Block::Glass { colour: Some(Colour::Green )}),
            "minecraft:red_stained_glass" => block(Block::Glass { colour: Some(Colour::Red )}),
            "minecraft:black_stained_glass" => block(Block::Glass { colour: Some(Colour::Black )}),
            "minecraft:oak_trapdoor" => block(trapdoor(DoorMaterial::Oak, &properties)),
            "minecraft:spruce_trapdoor" => block(trapdoor(DoorMaterial::Spruce, &properties)),
            "minecraft:birch_trapdoor" => block(trapdoor(DoorMaterial::Birch, &properties)),
            "minecraft:jungle_trapdoor" => block(trapdoor(DoorMaterial::Jungle, &properties)),
            "minecraft:acacia_trapdoor" => block(trapdoor(DoorMaterial::Acacia, &properties)),
            "minecraft:dark_oak_trapdoor" => block(trapdoor(DoorMaterial::DarkOak, &properties)),
            "minecraft:crimson_trapdoor" => block(trapdoor(DoorMaterial::Crimson, &properties)),
            "minecraft:warped_trapdoor" => block(trapdoor(DoorMaterial::Warped, &properties)),
            "minecraft:iron_trapdoor" => block(trapdoor(DoorMaterial::Iron, &properties)),
            "minecraft:infested_stone" => block(Block::InfestedStone),
            "minecraft:infested_cobblestone" => block(Block::InfestedCobblestone),
            "minecraft:infested_stone_bricks" => block(Block::InfestedStoneBricks),
            "minecraft:infested_mossy_stone_bricks" => block(Block::InfestedMossyStoneBricks),
            "minecraft:infested_cracked_stone_bricks" => block(Block::InfestedCrackedStoneBricks),
            "minecraft:infested_chiseled_stone_bricks" => block(Block::InfestedChiseledStoneBricks),
            "minecraft:stone_bricks" => block(Block::StoneBricks),
            "minecraft:mossy_stone_bricks" => block(Block::MossyStoneBricks),
            "minecraft:cracked_stone_bricks" => block(Block::CrackedStoneBricks),
            "minecraft:chiseled_stone_bricks" => block(Block::ChiseledStoneBricks),
            "minecraft:brown_mushroom_block" => block(brown_mushroom_block(&properties)),
            "minecraft:red_mushroom_block" => block(red_mushroom_block(&properties)),
            "minecraft:mushroom_stem" => block(mushroom_stem(&properties)),
            "minecraft:iron_bars" => block(Block::IronBars { waterlogged: waterlogged(&properties) }),
            "minecraft:glass_pane" => block(glass_pane(&properties)),
            "minecraft:melon" => block(Block::Melon),
            "minecraft:pumpkin_stem" => block(pumpkin_stem(&properties)),
            "minecraft:attached_pumpkin_stem" => block(attached_pumpkin_stem(&properties)),
            "minecraft:melon_stem" => block(melon_stem(&properties)),
            "minecraft:attached_melon_stem" => block(attached_melon_stem(&properties)),
            "minecraft:vine" => block(vine(&properties)),
            "minecraft:oak_fence_gate" => block(fence_gate(WoodMaterial::Oak, &properties)),
            "minecraft:spruce_fence_gate" => block(fence_gate(WoodMaterial::Spruce, &properties)),
            "minecraft:birch_fence_gate" => block(fence_gate(WoodMaterial::Birch, &properties)),
            "minecraft:jungle_fence_gate" => block(fence_gate(WoodMaterial::Jungle, &properties)),
            "minecraft:acacia_fence_gate" => block(fence_gate(WoodMaterial::Acacia, &properties)),
            "minecraft:dark_oak_fence_gate" => block(fence_gate(WoodMaterial::DarkOak, &properties)),
            "minecraft:crimson_fence_gate" => block(fence_gate(WoodMaterial::Crimson, &properties)),
            "minecraft:warped_fence_gate" => block(fence_gate(WoodMaterial::Warped, &properties)),
            "minecraft:mycelium" => block(Block::Mycelium),
            "minecraft:lily_pad" => block(Block::LilyPad),
            "minecraft:nether_bricks" => block(Block::NetherBricks),
            "minecraft:cracked_nether_bricks" => block(Block::CrackedNetherBricks),
            "minecraft:chiseled_nether_bricks" => block(Block::ChiseledNetherBricks),
            "minecraft:nether_wart" => block(nether_wart(&properties)),
            "minecraft:enchanting_table" => proto(ProtoBlock::EnchantingTable),
            "minecraft:brewing_stand" => proto(ProtoBlock::BrewingStand),
            "minecraft:cauldron" => block(cauldron(&properties)),
            "minecraft:end_portal" => block(Block::EndPortal),
            "minecraft:end_portal_frame" => block(end_portal_frame(&properties)),
            "minecraft:end_stone" => block(Block::EndStone),
            "minecraft:dragon_egg" => block(Block::DragonEgg),
            "minecraft:redstone_lamp" => block(Block::RedstoneLamp),
            "minecraft:cocoa" => block(cocoa(&properties)),
            "minecraft:emerald_ore" => block(Block::EmeraldOre),
            "minecraft:ender_chest" => block(ender_chest(&properties)),
            "minecraft:tripwire_hook" => block(tripwire_hook(&properties)),
            "minecraft:tripwire" => block(Block::Tripwire),
            "minecraft:emerald_block" => block(Block::BlockOfEmerald),
            // TODO 137 command block // Deferred for now, too complicated
            "minecraft:beacon" => proto(ProtoBlock::Beacon),
            "minecraft:cobblestone_wall" => block(wall(WallMaterial::Cobblestone, &properties)),
            "minecraft:mossy_cobblestone_wall" => block(wall(WallMaterial::MossyCobblestone, &properties)),
            "minecraft:brick_wall" => block(wall(WallMaterial::Brick, &properties)),
            "minecraft:prismarine_wall" => block(wall(WallMaterial::Prismarine, &properties)),
            "minecraft:red_sandstone_wall" => block(wall(WallMaterial::RedSandstone, &properties)),
            "minecraft:mossy_stone_brick_wall" => block(wall(WallMaterial::MossyStoneBrick, &properties)),
            "minecraft:granite_wall" => block(wall(WallMaterial::Granite, &properties)),
            "minecraft:stone_brick_wall" => block(wall(WallMaterial::StoneBrick, &properties)),
            "minecraft:nether_brick_wall" => block(wall(WallMaterial::NetherBrick, &properties)),
            "minecraft:andesite_wall" => block(wall(WallMaterial::Andesite, &properties)),
            "minecraft:red_nether_brick_wall" => block(wall(WallMaterial::RedNetherBrick, &properties)),
            "minecraft:sandstone_wall" => block(wall(WallMaterial::Sandstone, &properties)),
            "minecraft:end_stone_brick_wall" => block(wall(WallMaterial::EndStoneBrick, &properties)),
            "minecraft:diorite_wall" => block(wall(WallMaterial::Diorite, &properties)),
            "minecraft:blackstone_wall" => block(wall(WallMaterial::Blackstone, &properties)),
            "minecraft:polished_blackstone_wall" => block(wall(WallMaterial::PolishedBlackstone, &properties)),
            "minecraft:polished_blackstone_brick_wall" => block(wall(WallMaterial::PolishedBlackstoneBrick, &properties)),
            "minecraft:flower_pot" => block(Block::FlowerPot(FlowerPot { plant: None })),
            "minecraft:potted_dandelion" => block(potted_plant(PottedPlant::Dandelion)),
            "minecraft:potted_poppy" => block(potted_plant(PottedPlant::Poppy)),
            "minecraft:potted_blue_orchid" => block(potted_plant(PottedPlant::BlueOrchid)),
            "minecraft:potted_allium" => block(potted_plant(PottedPlant::Allium)),
            "minecraft:potted_azure_bluet" => block(potted_plant(PottedPlant::AzureBluet)),
            "minecraft:potted_red_tulip" => block(potted_plant(PottedPlant::TulipRed)),
            "minecraft:potted_orange_tulip" => block(potted_plant(PottedPlant::TulipOrange)),
            "minecraft:potted_white_tulip" => block(potted_plant(PottedPlant::TulipWhite)),
            "minecraft:potted_pink_tulip" => block(potted_plant(PottedPlant::TulipPink)),
            "minecraft:potted_oxeye_daisy" => block(potted_plant(PottedPlant::OxeyeDaisy)),
            "minecraft:potted_cornflower" => block(potted_plant(PottedPlant::Cornflower)),
            "minecraft:potted_lily_of_the_valley" => block(potted_plant(PottedPlant::LilyOfTheValley)),
            "minecraft:potted_wither_rose" => block(potted_plant(PottedPlant::WitherRose)),
            "minecraft:potted_oak_sapling" => block(potted_plant(PottedPlant::OakSapling)),
            "minecraft:potted_spruce_sapling" => block(potted_plant(PottedPlant::SpruceSapling)),
            "minecraft:potted_birch_sapling" => block(potted_plant(PottedPlant::BirchSapling)),
            "minecraft:potted_jungle_sapling" => block(potted_plant(PottedPlant::JungleSapling)),
            "minecraft:potted_acacia_sapling" => block(potted_plant(PottedPlant::AcaciaSapling)),
            "minecraft:potted_dark_oak_sapling" => block(potted_plant(PottedPlant::DarkOakSapling)),
            "minecraft:potted_red_mushroom" => block(potted_plant(PottedPlant::RedMushroom)),
            "minecraft:potted_brown_mushroom" => block(potted_plant(PottedPlant::BrownMushroom)),
            "minecraft:potted_fern" => block(potted_plant(PottedPlant::Fern)),
            "minecraft:potted_dead_bush" => block(potted_plant(PottedPlant::DeadBush)),
            "minecraft:potted_cactus" => block(potted_plant(PottedPlant::Cactus)),
            "minecraft:potted_bamboo" => block(potted_plant(PottedPlant::Bamboo)),
            "minecraft:potted_crimson_fungus" => block(potted_plant(PottedPlant::CrimsonFungus)),
            "minecraft:potted_warped_fungus" => block(potted_plant(PottedPlant::WarpedFungus)),
            "minecraft:potted_crimson_roots" => block(potted_plant(PottedPlant::CrimsonRoots)),
            "minecraft:potted_warped_roots" => block(potted_plant(PottedPlant::WarpedRoots)),
            "minecraft:carrots" => block(Block::Carrots { growth_stage: age0_7(&properties) }),
            "minecraft:potatoes" => block(Block::Potatoes { growth_stage: age0_7(&properties) }),
            "minecraft:skeleton_skull" => block(head(HeadVariant::SkeletonSkull, &properties)),
            "minecraft:wither_skeleton_skull" => block(head(HeadVariant::WitherSkeletonSkull, &properties)),
            "minecraft:player_head" => block(head(HeadVariant::PlayerHead, &properties)),
            "minecraft:zombie_head" => block(head(HeadVariant::ZombieHead, &properties)),
            "minecraft:creeper_head" => block(head(HeadVariant::CreeperHead, &properties)),
            "minecraft:dragon_head" => block(head(HeadVariant::DragonHead, &properties)),
            "minecraft:skeleton_wall_skull" => block(wall_head(HeadVariant::SkeletonSkull, &properties)),
            "minecraft:wither_skeleton_wall_skull" => block(wall_head(HeadVariant::WitherSkeletonSkull, &properties)),
            "minecraft:player_wall_head" => block(wall_head(HeadVariant::PlayerHead, &properties)),
            "minecraft:zombie_wall_head" => block(wall_head(HeadVariant::ZombieHead, &properties)),
            "minecraft:creeper_wall_head" => block(wall_head(HeadVariant::CreeperHead, &properties)),
            "minecraft:dragon_wall_head" => block(wall_head(HeadVariant::DragonHead, &properties)),
            "minecraft:anvil" => block(anvil(AnvilDamage::Intact, &properties)),
            "minecraft:chipped_anvil" => block(anvil(AnvilDamage::SlightlyDamaged, &properties)),
            "minecraft:damaged_anvil" => block(anvil(AnvilDamage::VeryDamaged, &properties)),
            "minecraft:trapped_chest" => proto(proto_trapped_chest(&properties)),
            "minecraft:comparator" => block(comparator(&properties)),
            "minecraft:daylight_detector" => block(daylight_detector(&properties)),
            "minecraft:redstone_block" => block(Block::BlockOfRedstone),
            "minecraft:nether_quartz_ore" => block(Block::QuartzOre),
            "minecraft:hopper" => proto(proto_hopper(&properties)),
            "minecraft:quartz_block" => block(Block::BlockOfQuartz),
            "minecraft:chiseled_quartz_block" => block(Block::ChiseledQuartzBlock),
            "minecraft:quartz_pillar" => block(quartz_pillar(&properties)),
            "minecraft:dropper" => proto(proto_dropper(&properties)),
            "minecraft:terracotta" => block(Block::Terracotta { colour: None }),
            "minecraft:white_terracotta" => block(Block::Terracotta { colour: Some(Colour::White) }),
            "minecraft:orange_terracotta" => block(Block::Terracotta { colour: Some(Colour::Orange) }),
            "minecraft:magenta_terracotta" => block(Block::Terracotta { colour: Some(Colour::Magenta) }),
            "minecraft:light_blue_terracotta" => block(Block::Terracotta { colour: Some(Colour::LightBlue) }),
            "minecraft:yellow_terracotta" => block(Block::Terracotta { colour: Some(Colour::Yellow) }),
            "minecraft:lime_terracotta" => block(Block::Terracotta { colour: Some(Colour::Lime) }),
            "minecraft:pink_terracotta" => block(Block::Terracotta { colour: Some(Colour::Pink) }),
            "minecraft:gray_terracotta" => block(Block::Terracotta { colour: Some(Colour::Gray) }),
            "minecraft:light_gray_terracotta" => block(Block::Terracotta { colour: Some(Colour::LightGray) }),
            "minecraft:cyan_terracotta" => block(Block::Terracotta { colour: Some(Colour::Cyan) }),
            "minecraft:purple_terracotta" => block(Block::Terracotta { colour: Some(Colour::Purple) }),
            "minecraft:blue_terracotta" => block(Block::Terracotta { colour: Some(Colour::Blue) }),
            "minecraft:brown_terracotta" => block(Block::Terracotta { colour: Some(Colour::Brown) }),
            "minecraft:green_terracotta" => block(Block::Terracotta { colour: Some(Colour::Green) }),
            "minecraft:red_terracotta" => block(Block::Terracotta { colour: Some(Colour::Red) }),
            "minecraft:black_terracotta" => block(Block::Terracotta { colour: Some(Colour::Black) }),
            "minecraft:white_stained_glass_pane" => block(stained_glass_pane(Colour::White, &properties)),
            "minecraft:orange_stained_glass_pane" => block(stained_glass_pane(Colour::Orange, &properties)),
            "minecraft:magenta_stained_glass_pane" => block(stained_glass_pane(Colour::Magenta, &properties)),
            "minecraft:light_blue_stained_glass_pane" => block(stained_glass_pane(Colour::LightBlue, &properties)),
            "minecraft:yellow_stained_glass_pane" => block(stained_glass_pane(Colour::Yellow, &properties)),
            "minecraft:lime_stained_glass_pane" => block(stained_glass_pane(Colour::Lime, &properties)),
            "minecraft:pink_stained_glass_pane" => block(stained_glass_pane(Colour::Pink, &properties)),
            "minecraft:gray_stained_glass_pane" => block(stained_glass_pane(Colour::Gray, &properties)),
            "minecraft:light_gray_stained_glass_pane" => block(stained_glass_pane(Colour::LightGray, &properties)),
            "minecraft:cyan_stained_glass_pane" => block(stained_glass_pane(Colour::Cyan, &properties)),
            "minecraft:purple_stained_glass_pane" => block(stained_glass_pane(Colour::Purple, &properties)),
            "minecraft:blue_stained_glass_pane" => block(stained_glass_pane(Colour::Blue, &properties)),
            "minecraft:brown_stained_glass_pane" => block(stained_glass_pane(Colour::Brown, &properties)),
            "minecraft:green_stained_glass_pane" => block(stained_glass_pane(Colour::Green, &properties)),
            "minecraft:red_stained_glass_pane" => block(stained_glass_pane(Colour::Red, &properties)),
            "minecraft:black_stained_glass_pane" => block(stained_glass_pane(Colour::Black, &properties)),
            "minecraft:slime_block" => block(Block::BlockOfSlime),
            "minecraft:barrier" => block(Block::Barrier),
            "minecraft:prismarine" => block(Block::Prismarine),
            "minecraft:prismarine_bricks" => block(Block::PrismarineBricks),
            "minecraft:dark_prismarine" => block(Block::DarkPrismarine),
            "minecraft:sea_lantern" => block(Block::SeaLantern),
            "minecraft:hay_block" => block(hay_bale(&properties)),
            "minecraft:white_carpet" => block(Block::Carpet { colour: Colour::White }),
            "minecraft:orange_carpet" => block(Block::Carpet { colour: Colour::Orange }),
            "minecraft:magenta_carpet" => block(Block::Carpet { colour: Colour::Magenta }),
            "minecraft:light_blue_carpet" => block(Block::Carpet { colour: Colour::LightBlue }),
            "minecraft:yellow_carpet" => block(Block::Carpet { colour: Colour::Yellow }),
            "minecraft:lime_carpet" => block(Block::Carpet { colour: Colour::Lime }),
            "minecraft:pink_carpet" => block(Block::Carpet { colour: Colour::Pink }),
            "minecraft:gray_carpet" => block(Block::Carpet { colour: Colour::Gray }),
            "minecraft:light_gray_carpet" => block(Block::Carpet { colour: Colour::LightGray }),
            "minecraft:cyan_carpet" => block(Block::Carpet { colour: Colour::Cyan }),
            "minecraft:purple_carpet" => block(Block::Carpet { colour: Colour::Purple }),
            "minecraft:blue_carpet" => block(Block::Carpet { colour: Colour::Blue }),
            "minecraft:brown_carpet" => block(Block::Carpet { colour: Colour::Brown }),
            "minecraft:green_carpet" => block(Block::Carpet { colour: Colour::Green }),
            "minecraft:red_carpet" => block(Block::Carpet { colour: Colour::Red }),
            "minecraft:black_carpet" => block(Block::Carpet { colour: Colour::Black }),
            "minecraft:coal_block" => block(Block::BlockOfCoal),
            "minecraft:sunflower" => block(tall("sunflower", &properties)),
            "minecraft:lilac" => block(tall("lilac", &properties)),
            "minecraft:rose_bush" => block(tall("rose_bush", &properties)),
            "minecraft:peony" => block(tall("peony", &properties)),
            "minecraft:tall_grass" => block(tall("tall_grass", &properties)),
            "minecraft:large_fern" => block(tall("large_fern", &properties)),
            "minecraft:white_bed" => block(bed(Colour::White, &properties)),
            "minecraft:orange_bed" => block(bed(Colour::Orange, &properties)),
            "minecraft:magenta_bed" => block(bed(Colour::Magenta, &properties)),
            "minecraft:light_blue_bed" => block(bed(Colour::LightBlue, &properties)),
            "minecraft:yellow_bed" => block(bed(Colour::Yellow, &properties)),
            "minecraft:lime_bed" => block(bed(Colour::Lime, &properties)),
            "minecraft:pink_bed" => block(bed(Colour::Pink, &properties)),
            "minecraft:gray_bed" => block(bed(Colour::Gray, &properties)),
            "minecraft:light_gray_bed" => block(bed(Colour::LightGray, &properties)),
            "minecraft:cyan_bed" => block(bed(Colour::Cyan, &properties)),
            "minecraft:purple_bed" => block(bed(Colour::Purple, &properties)),
            "minecraft:blue_bed" => block(bed(Colour::Blue, &properties)),
            "minecraft:brown_bed" => block(bed(Colour::Brown, &properties)),
            "minecraft:green_bed" => block(bed(Colour::Green, &properties)),
            "minecraft:red_bed" => block(bed(Colour::Red, &properties)),
            "minecraft:black_bed" => block(bed(Colour::Black, &properties)),
            "minecraft:white_banner" => proto(proto_banner(Colour::White, &properties)),
            "minecraft:orange_banner" => proto(proto_banner(Colour::Orange, &properties)),
            "minecraft:magenta_banner" => proto(proto_banner(Colour::Magenta, &properties)),
            "minecraft:light_blue_banner" => proto(proto_banner(Colour::LightBlue, &properties)),
            "minecraft:yellow_banner" => proto(proto_banner(Colour::Yellow, &properties)),
            "minecraft:lime_banner" => proto(proto_banner(Colour::Lime, &properties)),
            "minecraft:pink_banner" => proto(proto_banner(Colour::Pink, &properties)),
            "minecraft:gray_banner" => proto(proto_banner(Colour::Gray, &properties)),
            "minecraft:light_gray_banner" => proto(proto_banner(Colour::LightGray, &properties)),
            "minecraft:cyan_banner" => proto(proto_banner(Colour::Cyan, &properties)),
            "minecraft:purple_banner" => proto(proto_banner(Colour::Purple, &properties)),
            "minecraft:blue_banner" => proto(proto_banner(Colour::Blue, &properties)),
            "minecraft:brown_banner" => proto(proto_banner(Colour::Brown, &properties)),
            "minecraft:green_banner" => proto(proto_banner(Colour::Green, &properties)),
            "minecraft:red_banner" => proto(proto_banner(Colour::Red, &properties)),
            "minecraft:black_banner" => proto(proto_banner(Colour::Black, &properties)),
            "minecraft:white_wall_banner" => proto(proto_wall_banner(Colour::White, &properties)),
            "minecraft:orange_wall_banner" => proto(proto_wall_banner(Colour::Orange, &properties)),
            "minecraft:magenta_wall_banner" => proto(proto_wall_banner(Colour::Magenta, &properties)),
            "minecraft:light_blue_wall_banner" => proto(proto_wall_banner(Colour::LightBlue, &properties)),
            "minecraft:yellow_wall_banner" => proto(proto_wall_banner(Colour::Yellow, &properties)),
            "minecraft:lime_wall_banner" => proto(proto_wall_banner(Colour::Lime, &properties)),
            "minecraft:pink_wall_banner" => proto(proto_wall_banner(Colour::Pink, &properties)),
            "minecraft:gray_wall_banner" => proto(proto_wall_banner(Colour::Gray, &properties)),
            "minecraft:light_gray_wall_banner" => proto(proto_wall_banner(Colour::LightGray, &properties)),
            "minecraft:cyan_wall_banner" => proto(proto_wall_banner(Colour::Cyan, &properties)),
            "minecraft:purple_wall_banner" => proto(proto_wall_banner(Colour::Purple, &properties)),
            "minecraft:blue_wall_banner" => proto(proto_wall_banner(Colour::Blue, &properties)),
            "minecraft:brown_wall_banner" => proto(proto_wall_banner(Colour::Brown, &properties)),
            "minecraft:green_wall_banner" => proto(proto_wall_banner(Colour::Green, &properties)),
            "minecraft:red_wall_banner" => proto(proto_wall_banner(Colour::Red, &properties)),
            "minecraft:black_wall_banner" => proto(proto_wall_banner(Colour::Black, &properties)),
            "minecraft:red_sandstone" => block(Block::RedSandstone),
            "minecraft:chiseled_red_sandstone" => block(Block::ChiseledRedSandstone),
            "minecraft:smooth_red_sandstone" => block(Block::SmoothRedSandstone),
            "minecraft:cut_red_sandstone" => block(Block::CutRedSandstone),
            "minecraft:end_rod" => block(Block::EndRod { facing: facing_surface6(&properties)}),
            "minecraft:chorus_plant" => block(Block::ChorusPlant),
            "minecraft:chorus_flower" => block(chorus_flower(&properties)),
            "minecraft:purpur_block" => block(Block::PurpurBlock),
            "minecraft:purpur_pillar" => block(purpur_pillar(&properties)),
            "minecraft:end_stone_bricks" => block(Block::EndStoneBricks),
            "minecraft:beetroots" => block(Block::Beetroots { growth_stage: age0_3(&properties) }),
            "minecraft:grass_path" => block(Block::GrassPath),
            // TODO 209 EndGateway
            // TODO 210 repeating command block
            // TODO 211 chain command block
            // TODO 212 FrostedIce
            "minecraft:magma_block" => block(Block::MagmaBlock),
            "minecraft:nether_wart_block" => block(Block::NetherWartBlock),
            "minecraft:warped_wart_block" => block(Block::WarpedWartBlock),
            "minecraft:red_nether_bricks" => block(Block::RedNetherBricks),
            "minecraft:bone_block" => block(bone_block(&properties)),
            // TODO 217 StructureVoid
            "minecraft:observer" => block(observer(&properties)),
            "minecraft:shulker_box" => proto(proto_shulker_box(None, &properties)),
            "minecraft:white_shulker_box" => proto(proto_shulker_box(Some(Colour::White), &properties)),
            "minecraft:orange_shulker_box" => proto(proto_shulker_box(Some(Colour::Orange), &properties)),
            "minecraft:magenta_shulker_box" => proto(proto_shulker_box(Some(Colour::Magenta), &properties)),
            "minecraft:light_blue_shulker_box" => proto(proto_shulker_box(Some(Colour::LightBlue), &properties)),
            "minecraft:yellow_shulker_box" => proto(proto_shulker_box(Some(Colour::Yellow), &properties)),
            "minecraft:lime_shulker_box" => proto(proto_shulker_box(Some(Colour::Lime), &properties)),
            "minecraft:pink_shulker_box" => proto(proto_shulker_box(Some(Colour::Pink), &properties)),
            "minecraft:gray_shulker_box" => proto(proto_shulker_box(Some(Colour::Gray), &properties)),
            "minecraft:light_gray_shulker_box" => proto(proto_shulker_box(Some(Colour::LightGray), &properties)),
            "minecraft:cyan_shulker_box" => proto(proto_shulker_box(Some(Colour::Cyan), &properties)),
            "minecraft:purple_shulker_box" => proto(proto_shulker_box(Some(Colour::Purple), &properties)),
            "minecraft:blue_shulker_box" => proto(proto_shulker_box(Some(Colour::Blue), &properties)),
            "minecraft:brown_shulker_box" => proto(proto_shulker_box(Some(Colour::Brown), &properties)),
            "minecraft:green_shulker_box" => proto(proto_shulker_box(Some(Colour::Green), &properties)),
            "minecraft:red_shulker_box" => proto(proto_shulker_box(Some(Colour::Red), &properties)),
            "minecraft:black_shulker_box" => proto(proto_shulker_box(Some(Colour::Black), &properties)),
            "minecraft:white_glazed_terracotta" => block(glazed_terracotta(Colour::White, &properties)),
            "minecraft:orange_glazed_terracotta" => block(glazed_terracotta(Colour::Orange, &properties)),
            "minecraft:magenta_glazed_terracotta" => block(glazed_terracotta(Colour::Magenta, &properties)),
            "minecraft:light_blue_glazed_terracotta" => block(glazed_terracotta(Colour::LightBlue, &properties)),
            "minecraft:yellow_glazed_terracotta" => block(glazed_terracotta(Colour::Yellow, &properties)),
            "minecraft:lime_glazed_terracotta" => block(glazed_terracotta(Colour::Lime, &properties)),
            "minecraft:pink_glazed_terracotta" => block(glazed_terracotta(Colour::Pink, &properties)),
            "minecraft:gray_glazed_terracotta" => block(glazed_terracotta(Colour::Gray, &properties)),
            "minecraft:light_gray_glazed_terracotta" => block(glazed_terracotta(Colour::LightGray, &properties)),
            "minecraft:cyan_glazed_terracotta" => block(glazed_terracotta(Colour::Cyan, &properties)),
            "minecraft:purple_glazed_terracotta" => block(glazed_terracotta(Colour::Purple, &properties)),
            "minecraft:blue_glazed_terracotta" => block(glazed_terracotta(Colour::Blue, &properties)),
            "minecraft:brown_glazed_terracotta" => block(glazed_terracotta(Colour::Brown, &properties)),
            "minecraft:green_glazed_terracotta" => block(glazed_terracotta(Colour::Green, &properties)),
            "minecraft:red_glazed_terracotta" => block(glazed_terracotta(Colour::Red, &properties)),
            "minecraft:black_glazed_terracotta" => block(glazed_terracotta(Colour::Black, &properties)),
            "minecraft:white_concrete" => block(concrete(Colour::White)),
            "minecraft:orange_concrete" => block(concrete(Colour::Orange)),
            "minecraft:magenta_concrete" => block(concrete(Colour::Magenta)),
            "minecraft:light_blue_concrete" => block(concrete(Colour::LightBlue)),
            "minecraft:yellow_concrete" => block(concrete(Colour::Yellow)),
            "minecraft:lime_concrete" => block(concrete(Colour::Lime)),
            "minecraft:pink_concrete" => block(concrete(Colour::Pink)),
            "minecraft:gray_concrete" => block(concrete(Colour::Gray)),
            "minecraft:light_gray_concrete" => block(concrete(Colour::LightGray)),
            "minecraft:cyan_concrete" => block(concrete(Colour::Cyan)),
            "minecraft:purple_concrete" => block(concrete(Colour::Purple)),
            "minecraft:blue_concrete" => block(concrete(Colour::Blue)),
            "minecraft:brown_concrete" => block(concrete(Colour::Brown)),
            "minecraft:green_concrete" => block(concrete(Colour::Green)),
            "minecraft:red_concrete" => block(concrete(Colour::Red)),
            "minecraft:black_concrete" => block(concrete(Colour::Black)),
            "minecraft:white_concrete_powder" => block(concrete_powder(Colour::White)),
            "minecraft:orange_concrete_powder" => block(concrete_powder(Colour::Orange)),
            "minecraft:magenta_concrete_powder" => block(concrete_powder(Colour::Magenta)),
            "minecraft:light_blue_concrete_powder" => block(concrete_powder(Colour::LightBlue)),
            "minecraft:yellow_concrete_powder" => block(concrete_powder(Colour::Yellow)),
            "minecraft:lime_concrete_powder" => block(concrete_powder(Colour::Lime)),
            "minecraft:pink_concrete_powder" => block(concrete_powder(Colour::Pink)),
            "minecraft:gray_concrete_powder" => block(concrete_powder(Colour::Gray)),
            "minecraft:light_gray_concrete_powder" => block(concrete_powder(Colour::LightGray)),
            "minecraft:cyan_concrete_powder" => block(concrete_powder(Colour::Cyan)),
            "minecraft:purple_concrete_powder" => block(concrete_powder(Colour::Purple)),
            "minecraft:blue_concrete_powder" => block(concrete_powder(Colour::Blue)),
            "minecraft:brown_concrete_powder" => block(concrete_powder(Colour::Brown)),
            "minecraft:green_concrete_powder" => block(concrete_powder(Colour::Green)),
            "minecraft:red_concrete_powder" => block(concrete_powder(Colour::Red)),
            "minecraft:black_concrete_powder" => block(concrete_powder(Colour::Black)),
            // TODO 255 structure block
            "minecraft:crimson_nylium" => block(Block::CrimsonNylium),
            "minecraft:warped_nylium" => block(Block::WarpedNylium),
            "minecraft:nether_gold_ore" => block(Block::NetherGoldOre),
            "minecraft:seagrass" => block(seagrass(Seagrass::Seagrass)),
            "minecraft:tall_seagrass" => block(tall("tall_seagrass", &properties)),
            "minecraft:sea_pickle" => block(sea_pickle(&properties)),
            "minecraft:kelp" => block(kelp(&properties)),
            "minecraft:kelp_plant" => block(Block::KelpPlant),
            "minecraft:basalt" => block(basalt(&properties)),
            "minecraft:polished_basalt" => block(polished_basalt(&properties)),
            "minecraft:quartz_bricks" => block(Block::QuartzBricks),
            "minecraft:chain" => block(chain(&properties)),
            "minecraft:turtle_egg" => block(turtle_egg(&properties)),
            "minecraft:dried_kelp_block" => block(Block::DriedKelpBlock),
            "minecraft:scaffolding" => block(scaffolding(&properties)),

            "minecraft:tube_coral" => block(coral(CoralMaterial::Tube , &properties)),
            "minecraft:brain_coral" => block(coral(CoralMaterial::Brain , &properties)),
            "minecraft:bubble_coral" => block(coral(CoralMaterial::Bubble , &properties)),
            "minecraft:fire_coral" => block(coral(CoralMaterial::Fire , &properties)),
            "minecraft:horn_coral" => block(coral(CoralMaterial::Horn , &properties)),
            "minecraft:dead_tube_coral" => block(dead_coral(CoralMaterial::Tube , &properties)),
            "minecraft:dead_brain_coral" => block(dead_coral(CoralMaterial::Brain , &properties)),
            "minecraft:dead_bubble_coral" => block(dead_coral(CoralMaterial::Bubble , &properties)),
            "minecraft:dead_fire_coral" => block(dead_coral(CoralMaterial::Fire , &properties)),
            "minecraft:dead_horn_coral" => block(dead_coral(CoralMaterial::Horn , &properties)),
            "minecraft:tube_coral_block" => block(coral_block(CoralMaterial::Tube)),
            "minecraft:brain_coral_block" => block(coral_block(CoralMaterial::Brain)),
            "minecraft:bubble_coral_block" => block(coral_block(CoralMaterial::Bubble)),
            "minecraft:fire_coral_block" => block(coral_block(CoralMaterial::Fire)),
            "minecraft:horn_coral_block" => block(coral_block(CoralMaterial::Horn)),
            "minecraft:dead_tube_coral_block" => block(dead_coral_block(CoralMaterial::Tube)),
            "minecraft:dead_brain_coral_block" => block(dead_coral_block(CoralMaterial::Brain)),
            "minecraft:dead_bubble_coral_block" => block(dead_coral_block(CoralMaterial::Bubble)),
            "minecraft:dead_fire_coral_block" => block(dead_coral_block(CoralMaterial::Fire)),
            "minecraft:dead_horn_coral_block" => block(dead_coral_block(CoralMaterial::Horn)),
            "minecraft:tube_coral_fan" => block(coral_fan(CoralMaterial::Tube , &properties)),
            "minecraft:brain_coral_fan" => block(coral_fan(CoralMaterial::Brain , &properties)),
            "minecraft:bubble_coral_fan" => block(coral_fan(CoralMaterial::Bubble , &properties)),
            "minecraft:fire_coral_fan" => block(coral_fan(CoralMaterial::Fire , &properties)),
            "minecraft:horn_coral_fan" => block(coral_fan(CoralMaterial::Horn , &properties)),
            "minecraft:dead_tube_coral_fan" => block(dead_coral_fan(CoralMaterial::Tube , &properties)),
            "minecraft:dead_brain_coral_fan" => block(dead_coral_fan(CoralMaterial::Brain , &properties)),
            "minecraft:dead_bubble_coral_fan" => block(dead_coral_fan(CoralMaterial::Bubble , &properties)),
            "minecraft:dead_fire_coral_fan" => block(dead_coral_fan(CoralMaterial::Fire , &properties)),
            "minecraft:dead_horn_coral_fan" => block(dead_coral_fan(CoralMaterial::Horn , &properties)),
            "minecraft:tube_coral_wall_fan" => block(coral_wall_fan(CoralMaterial::Tube , &properties)),
            "minecraft:brain_coral_wall_fan" => block(coral_wall_fan(CoralMaterial::Brain , &properties)),
            "minecraft:bubble_coral_wall_fan" => block(coral_wall_fan(CoralMaterial::Bubble , &properties)),
            "minecraft:fire_coral_wall_fan" => block(coral_wall_fan(CoralMaterial::Fire , &properties)),
            "minecraft:horn_coral_wall_fan" => block(coral_wall_fan(CoralMaterial::Horn , &properties)),
            "minecraft:dead_tube_coral_wall_fan" => block(dead_coral_wall_fan(CoralMaterial::Tube , &properties)),
            "minecraft:dead_brain_coral_wall_fan" => block(dead_coral_wall_fan(CoralMaterial::Brain , &properties)),
            "minecraft:dead_bubble_coral_wall_fan" => block(dead_coral_wall_fan(CoralMaterial::Bubble , &properties)),
            "minecraft:dead_fire_coral_wall_fan" => block(dead_coral_wall_fan(CoralMaterial::Fire , &properties)),
            "minecraft:dead_horn_coral_wall_fan" => block(dead_coral_wall_fan(CoralMaterial::Horn , &properties)),
            "minecraft:cartography_table" => block(Block::CartographyTable),
            "minecraft:smithing_table" => block(Block::SmithingTable),
            "minecraft:fletching_table" => block(Block::FletchingTable),
            "minecraft:shroomlight" => block(Block::Shroomlight),
            "minecraft:honey_block" => block(Block::HoneyBlock),
            "minecraft:honeycomb_block" => block(Block::HoneycombBlock),
            "minecraft:lodestone" => block(Block::Lodestone),
            "minecraft:netherite_block" => block(Block::BlockOfNetherite),
            "minecraft:ancient_debris" => block(Block::AncientDebris),
            "minecraft:blackstone" => block(Block::Blackstone),
            "minecraft:gilded_blackstone" => block(Block::GildedBlackstone),
            "minecraft:polished_blackstone" => block(Block::PolishedBlackstone),
            "minecraft:chiseled_polished_blackstone" => block(Block::ChiseledPolishedBlackstone),
            "minecraft:polished_blackstone_bricks" => block(Block::PolishedBlackstoneBricks),
            "minecraft:cracked_polished_blackstone_bricks" => block(Block::CrackedPolishedBlackstoneBricks),
            "minecraft:loom" => block(Block::Loom { facing: facing_surface4(&properties) }),
            "minecraft:stonecutter" => block(Block::Stonecutter { facing: facing_surface4(&properties) }),
            "minecraft:lantern" => block(lantern(&properties)),
            "minecraft:soul_lantern" => block(soul_lantern(&properties)),
            "minecraft:composter" => block(Block::Composter { fullness: level0_8(&properties)}),
            "minecraft:grindstone" => block(Block::Grindstone(surface_rotation12(&properties))),
            "minecraft:sweet_berry_bush" => block(Block::SweetBerryBush { growth_stage: age0_3(&properties) }),
            "minecraft:respawn_anchor" => block(Block::RespawnAnchor { charges: charges0_15(&properties) }),
            "minecraft:target" => block(Block::Target),
            "minecraft:warped_fungus" => block(Block::WarpedFungus),
            "minecraft:crimson_fungus" => block(Block::CrimsonFungus),
            "minecraft:warped_roots" => block(Block::WarpedRoots),
            "minecraft:crimson_roots" => block(Block::CrimsonRoots),
            "minecraft:campfire" => block(campfire(&properties)),
            "minecraft:soul_campfire" => block(soul_campfire(&properties)),
            "minecraft:barrel" => proto(proto_barrel(&properties)),
            //"minecraft:" => block(Block::),

            /*
                // Missing blocks with tile entity (and possibly also properties)
                smoker
                blast_furnace

                lectern
                conduit
                bell
                beehive
                bee_nest (beehive)
            */
            // Catch-all
            _ => {
                warn!("Unknown block id: {}", name.as_str());
                block(Block::Unknown(None))
            }
        };
        palette.push(palette_item);
    }
//    println!("Palette: {:#?}", palette);

    Some(palette)
}

/// Convenience function for wrapping a Block in a PaletteItem.
fn block(block: Block) -> PaletteItem {
    PaletteItem::Block(block)
}

/// Convenience function for wrapping a ProtoBlock into a PaletteItem.
fn proto(proto_block: ProtoBlock) -> PaletteItem {
    PaletteItem::ProtoBlock(proto_block)
}

//
// Convenience functions for extracting values from properties
//

/// Get the string value at tag `name` within `properties`, if it exists.
fn properties_lookup_string(properties: &Option<Value>, name: &'static str) -> Option<String> {
    properties
        .as_ref()
        .and_then(|p| nbt_value_lookup_string(p, name).ok())
}

fn properties_lookup_bool(properties: &Option<Value>, name: &'static str, fallback: bool) -> bool {
    properties_lookup_string(properties, name)
        .and_then(boolean)
        .unwrap_or_else(|| {
            warn!("Using \"{}\" as fallback value for a boolean \"{}\" property.", fallback, name);
            fallback
        })
}

fn fluid_raw_level(properties: &Option<Value>) -> i8 {
    properties_lookup_string(properties, "level")
        .and_then(|s| s.parse::<i8>().ok())
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"level\" property of fluid.");
            0x0
        })
}

fn wood_alignment(properties: &Option<Value>) -> Axis3 {
    properties_lookup_string(properties, "axis")
        .and_then(|s| {
            match s.as_str() {
                "x" => Some(Axis3::X),
                "y" => Some(Axis3::Y),
                "z" => Some(Axis3::Z),
                s => {
                    warn!("Unknown \"axis\" value for axis aligned block: \"{}\".", s);
                    None
                }
            }
        })
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"axis\" property of axis aligned block.");
            Axis3::Y
        })
}

//
// Convenience functions for blocks
//

fn sapling(material: SaplingMaterial, properties: &Option<Value>) -> Block {
    let growth_stage = properties_lookup_string(properties, "stage")
        .and_then(i_0_through_1)
        .unwrap_or_else(|| {
            if material != SaplingMaterial::Bamboo {
                warn!("Using fallback value for \"stage\" property of sapling.");
            }
            Int0Through1::new(0).unwrap()
        });

    Block::Sapling { material, growth_stage }
}

fn water(properties: &Option<Value>) -> Block {
    let raw_level = fluid_raw_level(properties);

    #[allow(clippy::verbose_bit_mask)]
    if (raw_level & 0x7) == 0x0 {
        Block::WaterSource
    } else {
        Block::Water {
            falling: fluid_falling(raw_level),
            level: fluid_level(raw_level),
        }
    }
}

fn lava(properties: &Option<Value>) -> Block {
    let raw_level = fluid_raw_level(properties);

    #[allow(clippy::verbose_bit_mask)]
    if (raw_level & 0x7) == 0x0 {
        Block::LavaSource
    } else {
        Block::Lava {
            falling: fluid_falling(raw_level),
            level: fluid_level(raw_level),
        }
    }
}

fn fluid_falling(raw_level: i8) -> bool {
    (raw_level & 0x8) == 0x8
}

fn fluid_level(raw_level: i8) -> Int1Through7 {
    Int1Through7::new(8 - (raw_level & 0x7)).unwrap()
}

fn log(material: WoodMaterial, properties: &Option<Value>) -> Block {
    Block::Log(Log {
        material,
        alignment: wood_alignment(&properties),
        stripped: false,
        bark_on_all_sides: false,
    })
}

fn stripped_log(material: WoodMaterial, properties: &Option<Value>) -> Block {
    Block::Log(Log {
        material,
        alignment: wood_alignment(&properties),
        stripped: true,
        bark_on_all_sides: false,
    })
}

fn wood(material: WoodMaterial, properties: &Option<Value>) -> Block {
    Block::Log(Log {
        material,
        alignment: wood_alignment(&properties),
        stripped: false,
        bark_on_all_sides: true,
    })
}

fn stripped_wood(material: WoodMaterial, properties: &Option<Value>) -> Block {
    Block::Log(Log {
        material,
        alignment: wood_alignment(&properties),
        stripped: true,
        bark_on_all_sides: true,
    })
}

fn leaves(material: LeavesMaterial, properties: &Option<Value>) -> Block {
    let distance_to_trunk = properties_lookup_string(properties, "distance")
        .and_then(i_0_through_7);

    let persistent = properties_lookup_string(properties, "persistent")
        .and_then(boolean)
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"persistent\" property of leaves.");
            false
        });

    Block::Leaves { material, distance_to_trunk, persistent }
}

fn noteblock(properties: &Option<Value>) -> Block {
    let pitch = properties_lookup_string(properties, "note")
        .and_then(|note_string| note_string.parse().ok())
        .and_then(|note_integer| Some(Pitch::from_value(note_integer)))
        .unwrap_or_else(|| {
            let pitch = Pitch::from_value(0);
            warn!("Using fallback value \"{:?}\" for \"note\" property of note block.", pitch);
            pitch
        });

    Block::Noteblock(Noteblock { pitch })
}

fn rail(rail_type: RailType, properties: &Option<Value>) -> Block {
    let shape = properties_lookup_string(properties, "shape")
        .and_then(|shape| match shape.as_str() {
            "north_south" => Some(RailShape::NorthSouth),
            "east_west" => Some(RailShape::EastWest),
            "north_east" => Some(RailShape::NorthEast),
            "north_west" => Some(RailShape::NorthWest),
            "south_east" => Some(RailShape::SouthEast),
            "south_west" => Some(RailShape::SouthWest),
            "ascending_north" => Some(RailShape::AscendingNorth),
            "ascending_south" => Some(RailShape::AscendingSouth),
            "ascending_east" => Some(RailShape::AscendingEast),
            "ascending_west" => Some(RailShape::AscendingWest),
            _ => None,
        })
        .unwrap_or_else(|| {
            let shape = RailShape::NorthSouth;
            warn!("Using fallback value \"{:?}\" for \"shape\" property of rails.", shape);
            shape
        });

    Block::Rail { variant: rail_type, shape }
}

fn piston(sticky: bool, properties: &Option<Value>) -> Block {
    let facing = facing_surface6(properties);
    let extended = properties_lookup_bool(properties, "extended", false);

    if sticky {
        Block::StickyPiston { facing, extended }
    } else {
        Block::Piston { facing, extended }
    }
}

fn piston_head(properties: &Option<Value>) -> Block {
    let facing = facing_surface6(properties);

    let sticky = properties_lookup_string(properties, "type")
        .and_then(|t| match t.as_str() {
            "sticky" => Some(true),
            "normal" => Some(false),
            _ => None,
        })
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"type\" property of piston head");
            false
        });

    if sticky {
        Block::StickyPistonHead { facing }
    } else {
        Block::PistonHead { facing }
    }
}

fn slab(material: SlabMaterial, properties: &Option<Value>) -> Block {
    let position = properties_lookup_string(properties, "type")
        .and_then(|t| match t.as_str() {
            "bottom" => Some(SlabVariant::Bottom),
            "double" => Some(SlabVariant::Double),
            "top" => Some(SlabVariant::Top),
            _ => None,
        })
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"type\" property of slab");
            SlabVariant::Bottom
        });

    Block::Slab(Slab { material, position, waterlogged: waterlogged(properties) })
}

fn wall_torch(properties: &Option<Value>) -> Block {
    Block::Torch { attached: wall_torch_attached(properties) }
}

fn redstone_wall_torch(properties: &Option<Value>) -> Block {
    Block::RedstoneTorch { attached: wall_torch_attached(properties) }
}

fn soul_wall_torch(properties: &Option<Value>) -> Block {
    Block::SoulTorch { attached: wall_torch_attached(properties) }
}

fn wall_torch_attached(properties: &Option<Value>) -> Surface5 {
    properties_lookup_string(properties, "facing")
        .and_then(|facing| match facing.as_str() {
            "north" => Some(Surface5::South),
            "south" => Some(Surface5::North),
            "east" => Some(Surface5::West),
            "west" => Some(Surface5::East),
            _ => None,
        })
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"facing\" property of wall torch.");
            Surface5::North
        })
}

fn fire(properties: &Option<Value>) -> Block {
    Block::Fire { age: age0_15(properties) }
}

fn stairs(material: StairMaterial, properties: &Option<Value>) -> Block {
    let half = half_surface2(properties);
    let facing = facing_surface4(properties);
    let position = match (half, facing) {
            (Surface2::Down, Surface4::North) => Edge8::DownNorth,
            (Surface2::Down, Surface4::South) => Edge8::DownSouth,
            (Surface2::Down, Surface4::East) => Edge8::DownEast,
            (Surface2::Down, Surface4::West) => Edge8::DownWest,
            (Surface2::Up, Surface4::North) => Edge8::UpNorth,
            (Surface2::Up, Surface4::South) => Edge8::UpSouth,
            (Surface2::Up, Surface4::East) => Edge8::UpEast,
            (Surface2::Up, Surface4::West) => Edge8::UpWest,
    };

    Block::Stairs(Stair { material, position, waterlogged: waterlogged(properties) })
}

fn door(material: DoorMaterial, properties: &Option<Value>) -> Block {

    Block::Door(Door {
        material,
        facing: facing_surface4(properties),
        half: door_half(properties),
        hinged_at: door_hinge(properties),
        open: open(properties),
    })
}

fn ladder(properties: &Option<Value>) -> Block {
    Block::Ladder {
        facing: facing_surface4(properties),
        waterlogged: waterlogged(properties),
    }
}

fn lever(properties: &Option<Value>) -> Block {
    let surface_rotation = surface_rotation12(properties);
    let on_off_state = if powered(properties) { OnOffState::On } else { OnOffState::Off };

    Block::Lever(surface_rotation, on_off_state)
}

fn button(material: ButtonMaterial, properties: &Option<Value>) -> Block {
    Block::Button(material, surface_rotation12(properties))
}

fn pressure_plate(material: PressurePlateMaterial) -> Block {
    Block::PressurePlate { material }
}

fn snow(properties: &Option<Value>) -> Block {
    Block::Snow { thickness: layers1_8(properties) }
}

fn cactus(properties: &Option<Value>) -> Block {
    Block::Cactus { growth_stage: age0_15(properties) }
}

fn sugar_cane(properties: &Option<Value>) -> Block {
    Block::SugarCane { growth_stage: age0_15(properties) }
}

fn fence(material: FenceMaterial, properties: &Option<Value>) -> Block {
    Block::Fence { material, waterlogged: waterlogged(properties) }
}

fn carved_pumpkin(properties: &Option<Value>) -> Block {
    Block::CarvedPumpkin { facing: facing_surface4(properties) }
}

fn jack_o_lantern(properties: &Option<Value>) -> Block {
    Block::JackOLantern { facing: facing_surface4(properties) }
}

fn cake(properties: &Option<Value>) -> Block {
    Block::Cake { pieces: pieces1_7(properties) }
}

fn nether_portal(properties: &Option<Value>) -> Block {
    Block::NetherPortal { alignment: Some(portal_alignment(properties)) }
}

fn repeater(properties: &Option<Value>) -> Block {
    Block::RedstoneRepeater(RedstoneRepeater {
            facing: facing_surface4(properties),
            delay: delay1_4(properties),
    })
}

fn trapdoor(material: DoorMaterial, properties: &Option<Value>) -> Block {
    Block::Trapdoor(Trapdoor {
        material: material,
        hinge_at: trapdoor_hinge(properties),
        open: open(properties),
        waterlogged: waterlogged(properties),
    })
}

fn brown_mushroom_block(properties: &Option<Value>) -> Block {
    Block::BrownMushroomBlock { cap_directions: direction_flags6(properties) }
}

fn red_mushroom_block(properties: &Option<Value>) -> Block {
    Block::RedMushroomBlock { cap_directions: direction_flags6(properties) }
}

fn mushroom_stem(properties: &Option<Value>) -> Block {
    Block::MushroomStem { stem_directions: direction_flags6(properties) }
}

fn glass_pane(properties: &Option<Value>) -> Block {
    Block::GlassPane { colour: None, waterlogged: waterlogged(properties) }
}

fn pumpkin_stem(properties: &Option<Value>) -> Block {
    Block::PumpkinStem { state: StemState::Growing(age0_7(properties)) }
}

fn attached_pumpkin_stem(properties: &Option<Value>) -> Block {
    Block::PumpkinStem { state: StemState::Attached(facing_surface4(properties)) }
}

fn melon_stem(properties: &Option<Value>) -> Block {
    Block::MelonStem { state: StemState::Growing(age0_7(properties)) }
}

fn attached_melon_stem(properties: &Option<Value>) -> Block {
    Block::MelonStem { state: StemState::Attached(facing_surface4(properties)) }
}

fn vine(properties: &Option<Value>) -> Block {
    Block::Vines(Vines { anchored_at: direction_flags5(properties) })
}

fn fence_gate(material: WoodMaterial, properties: &Option<Value>) -> Block {
    Block::FenceGate { material, facing: facing_surface4(properties), open: open(properties) }
}

fn nether_wart(properties: &Option<Value>) -> Block {
    Block::NetherWart { growth_stage: age0_3(properties) }
}

fn cauldron(properties: &Option<Value>) -> Block {
    Block::Cauldron { water_level: level0_3(properties) }
}

fn end_portal_frame(properties: &Option<Value>) -> Block {
    let facing = facing_surface4(properties);

    let has_eye = properties_lookup_string(properties, "eye")
        .and_then(boolean)
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"eye\" property of end portal frame.");
            false
        });

    Block::EndPortalFrame { facing, has_eye }
}

fn cocoa(properties: &Option<Value>) -> Block {
    Block::Cocoa {
        growth_stage: age0_2(properties),
        facing: facing_surface4(properties),
    }
}

fn ender_chest(properties: &Option<Value>) -> Block {
    Block::EnderChest {
        facing: facing_surface4(properties),
        waterlogged: waterlogged(properties),
    }
}

fn tripwire_hook(properties: &Option<Value>) -> Block {
    Block::TripwireHook {
        facing: facing_surface4(properties),
    }
}

fn wall(material: WallMaterial, properties: &Option<Value>) -> Block {
    Block::Wall { material, waterlogged: waterlogged(properties) }
}

fn potted_plant(plant: PottedPlant) -> Block {
    Block::FlowerPot(FlowerPot { plant: Some(plant) })
}

fn head(variant: HeadVariant, properties: &Option<Value>) -> Block {
    Block::Head(Head {
        variant,
        placement: WallOrRotatedOnFloor::Floor(floor_sign_facing_direction16(properties)),
        waterlogged: false,
    })
}

fn wall_head(variant: HeadVariant, properties: &Option<Value>) -> Block {
    Block::Head(Head {
        variant,
        placement: WallOrRotatedOnFloor::Wall(wall_sign_facing_surface4(properties)),
        waterlogged: false,
    })
}

fn anvil(damage: AnvilDamage, properties: &Option<Value>) -> Block {
    Block::Anvil {
        facing: facing_surface4(properties),
        damage,
    }
}

fn comparator(properties: &Option<Value>) -> Block {
    let facing = facing_surface4(properties);

    let subtract = properties_lookup_string(properties, "mode")
        .and_then(|mode| match mode.as_str() {
            "subtract" => Some(true),
            "compare" => Some(false),
            _ => None,
        })
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"mode\" property of comparator.");
            false
        });

        if subtract {
            Block::RedstoneSubtractor { facing }
        } else {
            Block::RedstoneComparator { facing }
        }
}

fn daylight_detector(properties: &Option<Value>) -> Block {
    let inverted = properties_lookup_string(properties, "inverted")
        .and_then(boolean)
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"inverted\" property of daylight detector.");
            false
        });

    if inverted {
        Block::InvertedDaylightDetector
    } else {
        Block::DaylightDetector
    }
}

fn quartz_pillar(properties: &Option<Value>) -> Block {
    Block::QuartzPillar { alignment: wood_alignment(&properties) }
}

fn stained_glass_pane(colour: Colour, properties: &Option<Value>) -> Block {
    Block::GlassPane { colour: Some(colour), waterlogged: waterlogged(properties) }
}

fn hay_bale(properties: &Option<Value>) -> Block {
    Block::HayBale { alignment: wood_alignment(&properties) }
}

fn tall(name: &'static str, properties: &Option<Value>) -> Block {
    match name {
        "sunflower" => tall_helper(
                Block::Flower(Flower::SunflowerTop),
                Block::Flower(Flower::SunflowerBottom),
                properties,
            ),
        "lilac" => tall_helper(
                Block::Flower(Flower::LilacTop),
                Block::Flower(Flower::LilacBottom),
                properties,
            ),
        "peony" => tall_helper(
                Block::Flower(Flower::PeonyTop),
                Block::Flower(Flower::PeonyBottom),
                properties,
            ),
        "rose_bush" => tall_helper(
                Block::Flower(Flower::RoseBushTop),
                Block::Flower(Flower::RoseBushBottom),
                properties,
            ),
        "tall_grass" => tall_helper(
                Block::Grass(Grass::TallGrassTop),
                Block::Grass(Grass::TallGrassBottom),
                properties,
            ),
        "large_fern" => tall_helper(
                Block::Grass(Grass::LargeFernTop),
                Block::Grass(Grass::LargeFernBottom),
                properties,
            ),
        "tall_seagrass" => tall_helper(
                Block::Seagrass { variant: Seagrass::TallSeagrassTop },
                Block::Seagrass { variant: Seagrass::TallSeagrassBottom },
                properties,
            ),
        _ => unreachable!(),
    }
}

fn tall_helper(top: Block, bottom: Block, properties: &Option<Value>) -> Block {
    match door_half(properties) {
        DoorHalf::Upper => top,
        DoorHalf::Lower => bottom,
    }
}

fn bed(colour: Colour, properties: &Option<Value>) -> Block {
    Block::Bed(Bed {
        colour,
        facing: facing_surface4(properties),
        end: bed_part(properties),
    })
}

fn chorus_flower(properties: &Option<Value>) -> Block {
    Block::ChorusFlower { growth_stage: age0_5(properties) }
}

fn purpur_pillar(properties: &Option<Value>) -> Block {
    Block::PurpurPillar { alignment: wood_alignment(&properties) }
}

fn bone_block(properties: &Option<Value>) -> Block {
    Block::BoneBlock { alignment: wood_alignment(&properties) }
}

fn observer(properties: &Option<Value>) -> Block {
    Block::Observer { facing: facing_surface6(properties) }
}

fn glazed_terracotta(colour: Colour, properties: &Option<Value>) -> Block {
    Block::GlazedTerracotta(GlazedTerracotta {
        colour,
        facing: facing_surface4(properties),
    })
}

fn concrete(colour: Colour) -> Block {
    Block::Concrete { colour }
}

fn concrete_powder(colour: Colour) -> Block {
    Block::ConcretePowder { colour }
}

fn seagrass(variant: Seagrass) -> Block {
    Block::Seagrass{ variant }
}

fn sea_pickle(properties: &Option<Value>) -> Block {
    Block::SeaPickle{
        count: pickles1_4(properties),
        waterlogged: waterlogged(properties),
    }
}

fn kelp(properties: &Option<Value>) -> Block {
    Block::Kelp{ growth_stage: age0_25(properties) }
}

fn basalt(properties: &Option<Value>) -> Block {
    Block::Basalt { alignment: wood_alignment(&properties) }
}

fn polished_basalt(properties: &Option<Value>) -> Block {
    Block::PolishedBasalt { alignment: wood_alignment(&properties) }
}

fn chain(properties: &Option<Value>) -> Block {
    Block::Chain { alignment: wood_alignment(&properties) }
}

fn turtle_egg(properties: &Option<Value>) -> Block {
    Block::TurtleEgg { count: eggs1_4(properties), age: age0_2(properties) }
}

fn scaffolding(properties: &Option<Value>) -> Block {
    Block::Scaffolding { waterlogged: waterlogged(properties) }
}

fn coral(material: CoralMaterial, properties: &Option<Value>) -> Block {
    Block::Coral { material, dead: false, waterlogged: waterlogged(properties) }
}

fn dead_coral(material: CoralMaterial, properties: &Option<Value>) -> Block {
    Block::Coral { material, dead: true, waterlogged: waterlogged(properties) }
}

fn coral_block(material: CoralMaterial) -> Block {
    Block::CoralBlock { material, dead: false }
}

fn dead_coral_block(material: CoralMaterial) -> Block {
    Block::CoralBlock { material, dead: true }
}

fn coral_fan(material: CoralMaterial, properties: &Option<Value>) -> Block {
    Block::CoralFan {
        material,
        facing: Surface5::Down,
        dead: false,
        waterlogged: waterlogged(properties),
    }
}

fn dead_coral_fan(material: CoralMaterial, properties: &Option<Value>) -> Block {
    Block::CoralFan {
        material,
        facing: Surface5::Down,
        dead: true,
        waterlogged: waterlogged(properties),
    }
}

fn coral_wall_fan(material: CoralMaterial, properties: &Option<Value>) -> Block {
    Block::CoralFan {
        material,
        facing: facing_surface5(properties),
        dead: false,
        waterlogged: waterlogged(properties),
    }
}

fn dead_coral_wall_fan(material: CoralMaterial, properties: &Option<Value>) -> Block {
    Block::CoralFan {
        material,
        facing: facing_surface5(properties),
        dead: true,
        waterlogged: waterlogged(properties),
    }
}

fn lantern(properties: &Option<Value>) -> Block {
    Block::Lantern {
        mounted_at: lantern_mounting(properties),
        waterlogged: waterlogged(properties),
    }
}

fn soul_lantern(properties: &Option<Value>) -> Block {
    Block::SoulLantern {
        mounted_at: lantern_mounting(properties),
        waterlogged: waterlogged(properties),
    }
}

fn campfire(properties: &Option<Value>) -> Block {
    Block::Campfire {
        facing: facing_surface4(properties),
        lit: lit(properties),
        waterlogged: waterlogged(properties),
    }
}

fn soul_campfire(properties: &Option<Value>) -> Block {
    Block::SoulCampfire {
        facing: facing_surface4(properties),
        lit: lit(properties),
        waterlogged: waterlogged(properties),
    }
}

/*
*/

//
// Convenience functions for proto blocks
//

fn proto_banner(colour: Colour, properties: &Option<Value>) -> ProtoBlock {
    ProtoBlock::Banner {
        colour,
        placement: WallOrRotatedOnFloor::Floor(floor_sign_facing_direction16(properties)),
    }
}

fn proto_barrel(properties: &Option<Value>) -> ProtoBlock {
    ProtoBlock::Barrel { facing: facing_surface6(properties) }
}

fn proto_wall_banner(colour: Colour, properties: &Option<Value>) -> ProtoBlock {
    ProtoBlock::Banner {
        colour,
        placement: WallOrRotatedOnFloor::Wall(wall_sign_facing_surface4(properties)),
    }
}

fn proto_dispenser(properties: &Option<Value>) -> ProtoBlock {
    ProtoBlock::Dispenser { facing: facing_surface6(properties) }
}

fn proto_dropper(properties: &Option<Value>) -> ProtoBlock {
    ProtoBlock::Dropper { facing: facing_surface6(properties) }
}

fn proto_chest(properties: &Option<Value>) -> ProtoBlock {
    ProtoBlock::Chest {
        facing: facing_surface4(properties),
        variant: chest_variant(properties),
        waterlogged: waterlogged(properties),
    }
}

fn proto_trapped_chest(properties: &Option<Value>) -> ProtoBlock {
    ProtoBlock::TrappedChest {
        facing: facing_surface4(properties),
        variant: chest_variant(properties),
        waterlogged: waterlogged(properties),
    }
}

fn proto_furnace(properties: &Option<Value>) -> ProtoBlock {
    ProtoBlock::Furnace {
        facing: facing_surface4(properties),
        lit: lit(properties),
    }
}

fn proto_shulker_box(colour: Option<Colour>, properties: &Option<Value>) -> ProtoBlock {
    ProtoBlock::ShulkerBox { colour, facing: facing_surface6(properties) }
}

fn proto_sign(material: WoodMaterial, properties: &Option<Value>) -> ProtoBlock {
    ProtoBlock::Sign {
        material,
        placement: WallOrRotatedOnFloor::Floor(floor_sign_facing_direction16(properties)),
        waterlogged: waterlogged(properties),
    }
}

fn proto_wall_sign(material: WoodMaterial, properties: &Option<Value>) -> ProtoBlock {
    ProtoBlock::Sign {
        material,
        placement: WallOrRotatedOnFloor::Wall(wall_sign_facing_surface4(properties)),
        waterlogged: waterlogged(properties),
    }
}

fn proto_hopper(properties: &Option<Value>) -> ProtoBlock {
    ProtoBlock::Hopper { facing: facing_surface5(properties) }
}

//
// Convenience functions for undecided block/proto block
//

fn jukebox(properties: &Option<Value>) -> PaletteItem {
    let has_record = properties_lookup_string(properties, "has_record")
        .and_then(boolean)
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"has_record\" property of jukebox.");
            false
        });

    if has_record {
        PaletteItem::ProtoBlock(ProtoBlock::Jukebox)
    } else {
        PaletteItem::Block(Block::Jukebox(Box::new(Jukebox { record: None })))
    }
}

//
// Convenience functions for value import
//

// TODO check, and consider using the facing of the sign, instead of its attachment
// surface.
fn wall_sign_facing_surface4(properties: &Option<Value>) -> Surface4 {
    properties_lookup_string(properties, "facing")
        .and_then(|facing| match facing.as_str() {
            "north" => Some(Surface4::South),
            "south" => Some(Surface4::North),
            "east" => Some(Surface4::West),
            "west" => Some(Surface4::East),
            _ => None,
        })
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"facing\" property of a wall mounted block.");
            Surface4::North
        })
}

fn direction_flags5(properties: &Option<Value>) -> DirectionFlags5 {
    DirectionFlags5 {
        east: properties_lookup_bool(properties, "east", false),
        north: properties_lookup_bool(properties, "north", false),
        south: properties_lookup_bool(properties, "south", false),
        up: properties_lookup_bool(properties, "up", false),
        west: properties_lookup_bool(properties, "west", false),
    }
}

fn direction_flags6(properties: &Option<Value>) -> DirectionFlags6 {
    DirectionFlags6 {
        east: properties_lookup_bool(properties, "east", false),
        down: properties_lookup_bool(properties, "down", false),
        north: properties_lookup_bool(properties, "north", false),
        south: properties_lookup_bool(properties, "south", false),
        up: properties_lookup_bool(properties, "up", false),
        west: properties_lookup_bool(properties, "west", false),
    }
}

fn floor_sign_facing_direction16(properties: &Option<Value>) -> Direction16 {
    properties_lookup_string(properties, "rotation")
        .and_then(|i| i.as_str().parse::<i8>().ok())
        .and_then(|i| Some(Direction16::from(i)))
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"rotation\" property of a free standing block.");
            Direction16::North
        })
}

/// Convert a string to an Int0Through1 value
fn i_0_through_1(string: String) -> Option<Int0Through1> {
    string.parse::<i8>().ok()
        .and_then(|i| Int0Through1::new(i))
}

/// Convert a string to an Int0Through2 value
fn i_0_through_2(string: String) -> Option<Int0Through2> {
    string.parse::<i8>().ok()
        .and_then(|i| Int0Through2::new(i))
}

/// Convert a string to an Int0Through3 value
fn i_0_through_3(string: String) -> Option<Int0Through3> {
    string.parse::<i8>().ok()
        .and_then(|i| Int0Through3::new(i))
}

/// Convert a string to an Int0Through5 value
fn i_0_through_5(string: String) -> Option<Int0Through5> {
    string.parse::<i8>().ok()
        .and_then(|i| Int0Through5::new(i))
}

/// Convert a string to an Int0Through6 value
fn i_0_through_6(string: String) -> Option<Int0Through6> {
    string.parse::<i8>().ok()
        .and_then(|i| Int0Through6::new(i))
}

/// Convert a string to an Int0Through7 value
fn i_0_through_7(string: String) -> Option<Int0Through7> {
    string.parse::<i8>().ok()
        .and_then(|i| Int0Through7::new(i))
}

/// Convert a string to an Int0Through8 value
fn i_0_through_8(string: String) -> Option<Int0Through8> {
    string.parse::<i8>().ok()
        .and_then(|i| Int0Through8::new(i))
}

/// Convert a string to an Int0Through15 value
fn i_0_through_15(string: String) -> Option<Int0Through15> {
    string.parse::<i8>().ok()
        .and_then(|i| Int0Through15::new(i))
}

/// Convert a string to an Int0Through25 value
fn i_0_through_25(string: String) -> Option<Int0Through25> {
    string.parse::<i8>().ok()
        .and_then(|i| Int0Through25::new(i))
}

/// Convert a string to an Int1Through4 value
fn i_1_through_4(string: String) -> Option<Int1Through4> {
    string.parse::<i8>().ok()
        .and_then(|i| Int1Through4::new(i))
}

/// Convert a string to an Int1Through8 value
fn i_1_through_8(string: String) -> Option<Int1Through8> {
    string.parse::<i8>().ok()
        .and_then(|i| Int1Through8::new(i))
}

fn age0_2(properties: &Option<Value>) -> Int0Through2 {
    properties_lookup_string(properties, "age")
        .and_then(i_0_through_2)
        .unwrap_or_else(|| {
            Int0Through2::new(0).unwrap()
        })
}

fn age0_3(properties: &Option<Value>) -> Int0Through3 {
    i0_3(properties, "age", 0)
}

fn level0_3(properties: &Option<Value>) -> Int0Through3 {
    i0_3(properties, "level", 0)
}

fn i0_3(properties: &Option<Value>, name: &'static str, fallback: i8) -> Int0Through3 {
    properties_lookup_string(properties, name)
        .and_then(i_0_through_3)
        .unwrap_or_else(|| {
            warn!("Using fallback value \"{}\" for \"{}\" property of a block.", fallback, name);
            Int0Through3::new(fallback).unwrap()
        })
}

fn age0_5(properties: &Option<Value>) -> Int0Through5 {
    properties_lookup_string(properties, "age")
        .and_then(i_0_through_5)
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"age\" property of a block.");
            Int0Through5::new(0).unwrap()
        })
}

fn age0_7(properties: &Option<Value>) -> Int0Through7 {
    i0_7(properties, "age", 0)
}

fn moisture0_7(properties: &Option<Value>) -> Int0Through7 {
    i0_7(properties, "moisture", 7)
}

fn i0_7(properties: &Option<Value>, name: &'static str, fallback: i8) -> Int0Through7 {
    properties_lookup_string(properties, name)
        .and_then(i_0_through_7)
        .unwrap_or_else(|| {
            warn!("Using fallback value \"{}\" for \"{}\" property of a block.", fallback, name);
            Int0Through7::new(fallback).unwrap()
        })
}

fn level0_8(properties: &Option<Value>) -> Int0Through8 {
    properties_lookup_string(properties, "level")
        .and_then(i_0_through_8)
        .unwrap_or_else(|| {
            Int0Through8::new(0).unwrap()
        })
}

fn age0_15(properties: &Option<Value>) -> Int0Through15 {
    properties_lookup_string(properties, "age")
        .and_then(i_0_through_15)
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"age\" property of a block.");
            Int0Through15::new(0).unwrap()
        })
}

fn charges0_15(properties: &Option<Value>) -> Int0Through15 {
    properties_lookup_string(properties, "charges")
        .and_then(i_0_through_15)
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"charges\" property of a block.");
            Int0Through15::new(0).unwrap()
        })
}

fn age0_25(properties: &Option<Value>) -> Int0Through25 {
    properties_lookup_string(properties, "age")
        .and_then(i_0_through_25)
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"age\" property of a block.");
            Int0Through25::new(25).unwrap()
        })
}

fn pickles1_4(properties: &Option<Value>) -> Int1Through4 {
    properties_lookup_string(properties, "pickles")
        .and_then(i_1_through_4)
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"pickles\" property of a block.");
            Int1Through4::new(1).unwrap()
        })
}

fn eggs1_4(properties: &Option<Value>) -> Int1Through4 {
    properties_lookup_string(properties, "eggs")
        .and_then(i_1_through_4)
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"eggs\" property of a block.");
            Int1Through4::new(1).unwrap()
        })
}

fn delay1_4(properties: &Option<Value>) -> Int1Through4 {
    properties_lookup_string(properties, "delay")
        .and_then(i_1_through_4)
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"delay\" property of a block.");
            Int1Through4::new(1).unwrap()
        })
}

fn layers1_8(properties: &Option<Value>) -> Int1Through8 {
    properties_lookup_string(properties, "layers")
        .and_then(i_1_through_8)
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"layers\" property of a block.");
            Int1Through8::new(1).unwrap()
        })
}

fn pieces1_7(properties: &Option<Value>) -> Int1Through7 {
    properties_lookup_string(properties, "bites")
        .and_then(i_0_through_6)
        .and_then(|bites| Some(7 - i8::from(bites)))
        .and_then(|pieces| Int1Through7::new(pieces))
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"bites\" property of a block.");
            Int1Through7::new(7).unwrap()
        })
}

fn chest_variant(properties: &Option<Value>) -> Option<ChestVariant> {
    properties_lookup_string(properties, "type")
        .and_then(|variant| match variant.as_str() {
            "single" => Some(ChestVariant::Single),
            "left" => Some(ChestVariant::Left),
            "right" => Some(ChestVariant::Right),
            _ => None,
        })
}

enum Face3 {
    Ceiling,
    Floor,
    Wall,
}

impl Default for Face3 {
    fn default() -> Self {
        Self::Wall
    }
}

fn surface_rotation12(properties: &Option<Value>) -> SurfaceRotation12 {
    let face = face3(properties);
    let facing = facing_surface4(properties);

    match (face, facing) {
        (Face3::Floor, Surface4::North) => SurfaceRotation12::DownFacingNorth,
        (Face3::Floor, Surface4::South) => SurfaceRotation12::DownFacingSouth,
        (Face3::Floor, Surface4::East) => SurfaceRotation12::DownFacingEast,
        (Face3::Floor, Surface4::West) => SurfaceRotation12::DownFacingWest,
        (Face3::Wall, Surface4::North) => SurfaceRotation12::North,
        (Face3::Wall, Surface4::South) => SurfaceRotation12::South,
        (Face3::Wall, Surface4::East) => SurfaceRotation12::East,
        (Face3::Wall, Surface4::West) => SurfaceRotation12::West,
        (Face3::Ceiling, Surface4::North) => SurfaceRotation12::UpFacingNorth,
        (Face3::Ceiling, Surface4::South) => SurfaceRotation12::UpFacingSouth,
        (Face3::Ceiling, Surface4::East) => SurfaceRotation12::UpFacingEast,
        (Face3::Ceiling, Surface4::West) => SurfaceRotation12::UpFacingWest,
    }
}

fn face3(properties: &Option<Value>) -> Face3 {
    properties_lookup_string(properties, "face")
        .and_then(|face| match face.as_str() {
            "ceiling" => Some(Face3::Ceiling),
            "floor" => Some(Face3::Floor),
            "wall" => Some(Face3::Wall),
            _ => None,
        })
        .unwrap_or_else(|| {
            warn!("Using fallback value for three directional \"face\" property.");
            Face3::default()
        })
}

fn facing_surface4(properties: &Option<Value>) -> Surface4 {
    properties_lookup_string(properties, "facing")
        .and_then(|facing| match facing.as_str() {
            "north" => Some(Surface4::North),
            "south" => Some(Surface4::South),
            "east" => Some(Surface4::East),
            "west" => Some(Surface4::West),
            _ => None,
        })
        .unwrap_or_else(|| {
            warn!("Using fallback value for four directional \"facing\" property.");
            Surface4::default()
        })
}

fn facing_surface5(properties: &Option<Value>) -> Surface5 {
    properties_lookup_string(properties, "facing")
        .and_then(|facing| match facing.as_str() {
            "north" => Some(Surface5::North),
            "south" => Some(Surface5::South),
            "east" => Some(Surface5::East),
            "west" => Some(Surface5::West),
            "down" => Some(Surface5::Down),
            _ => None,
        })
        .unwrap_or_else(|| {
            warn!("Using fallback value for five directional \"facing\" property.");
            Surface5::default()
        })
}

fn facing_surface6(properties: &Option<Value>) -> Surface6 {
    properties_lookup_string(properties, "facing")
        .and_then(|string| match string.as_str() {
            "up" => Some(Surface6::Up),
            "down" => Some(Surface6::Down),
            "north" => Some(Surface6::North),
            "south" => Some(Surface6::South),
            "east" => Some(Surface6::East),
            "west" => Some(Surface6::West),
            _ => None,
        })
        .unwrap_or_else(|| {
            warn!("Using fallback value for six directional \"facing\" property.");
            Surface6::default()
        })
}

fn half_surface2(properties: &Option<Value>) -> Surface2 {
    properties_lookup_string(properties, "half")
        .and_then(|string| match string.as_str() {
            "top" => Some(Surface2::Up),
            "bottom" => Some(Surface2::Down),
            _ => None,
        })
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"half\" property.");
            Surface2::Down
        })
}

fn trapdoor_hinge(properties: &Option<Value>) -> Edge8 {
    let half = half_surface2(properties);
    let facing = facing_surface4(properties);

    match (half, facing) {
        (Surface2::Up, Surface4::North) => Edge8::UpNorth,
        (Surface2::Up, Surface4::South) => Edge8::UpSouth,
        (Surface2::Up, Surface4::East) => Edge8::UpEast,
        (Surface2::Up, Surface4::West) => Edge8::UpWest,
        (Surface2::Down, Surface4::North) => Edge8::DownNorth,
        (Surface2::Down, Surface4::South) => Edge8::DownSouth,
        (Surface2::Down, Surface4::East) => Edge8::DownEast,
        (Surface2::Down, Surface4::West) => Edge8::DownWest,
    }
}

fn door_hinge(properties: &Option<Value>) -> Hinge {
    properties_lookup_string(properties, "hinge")
        .and_then(|string| match string.as_str() {
            "left" => Some(Hinge::Left),
            "right" => Some(Hinge::Right),
            _ => None,
        })
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"hinge\" property.");
            Hinge::Left
        })
}

fn door_half(properties: &Option<Value>) -> DoorHalf {
    properties_lookup_string(properties, "half")
        .and_then(|string| match string.as_str() {
            "upper" => Some(DoorHalf::Upper),
            "lower" => Some(DoorHalf::Lower),
            _ => None,
        })
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"half\" property.");
            DoorHalf::Upper
        })
}

fn open(properties: &Option<Value>) -> bool {
    properties_lookup_bool(properties, "open", false)
}

fn waterlogged(properties: &Option<Value>) -> bool {
    properties_lookup_bool(properties, "waterlogged", false)
}

fn lit(properties: &Option<Value>) -> bool {
    properties_lookup_bool(properties, "lit", false)
}

fn powered(properties: &Option<Value>) -> bool {
    properties_lookup_bool(properties, "powered", false)
}

fn boolean(string: String) -> Option<bool> {
    string.as_str().parse().ok()
}

fn lantern_mounting(properties: &Option<Value>) -> Surface2 {
    let hanging = properties_lookup_bool(properties, "hanging", false);
    match hanging {
        true => Surface2::Up,
        false => Surface2::Down,
    }
}

fn portal_alignment(properties: &Option<Value>) -> Axis2 {
    properties_lookup_string(properties, "axis")
        .and_then(|string| match string.as_str() {
            "x" => Some(Axis2::X),
            "z" => Some(Axis2::Z),
            _ => None,
        })
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"axis\" property.");
            Axis2::X
        })
}

fn bed_part(properties: &Option<Value>) -> BedEnd {
    properties_lookup_string(properties, "part")
        .and_then(|string| match string.as_str() {
            "foot" => Some(BedEnd::Foot),
            "head" => Some(BedEnd::Head),
            _ => None,
        })
        .unwrap_or_else(|| {
            warn!("Using fallback value for \"part\" property.");
            BedEnd::Head
        })
}
