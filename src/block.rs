use std::convert::TryFrom;

mod banner;
mod beacon;
mod bed;
mod brewing_stand;
mod chest;
mod dispenser;
mod door;
mod dropper;
mod flower_pot;
mod furnace;
mod glazed_terracotta;
mod head;
mod hopper;
mod noteblock;
mod redstone_repeater;
mod shulker_box;
mod sign;
mod stair;
mod trapdoor;
mod vines;

mod foilage;
mod light;

pub use self::banner::*;
pub use self::beacon::*;
pub use self::bed::*;
pub use self::brewing_stand::*;
pub use self::chest::*;
pub use self::dispenser::*;
pub use self::door::*;
pub use self::dropper::*;
pub use self::flower_pot::*;
pub use self::furnace::*;
pub use self::glazed_terracotta::*;
pub use self::head::*;
pub use self::hopper::*;
pub use self::noteblock::*;
pub use self::redstone_repeater::*;
pub use self::shulker_box::*;
pub use self::sign::*;
pub use self::stair::*;
pub use self::trapdoor::*;
pub use self::vines::*;

use crate::bounded_ints::*;
use crate::colour::*;
use crate::item::Item;
use crate::material::*;
use crate::positioning::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SlabVariant {
    Bottom,
    Double,
    Top,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Slab {
    pub material: SlabMaterial,
    pub position: SlabVariant,
    pub waterlogged: bool,
}

impl Slab {
    pub fn has_material_of(&self, material: Material) -> bool {
        material == self.material.clone().into()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RailType {
    Activator,
    Detector,
    Normal,
    Powered,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RailShape {
    EastWest,
    NorthEast,
    NorthSouth,
    NorthWest,
    SouthEast,
    SouthWest,
    AscendingEast,
    AscendingNorth,
    AscendingSouth,
    AscendingWest,
}

impl RailShape {
    pub fn from_value(value: i8) -> Self {
        match value {
            0 => Self::NorthSouth,
            1 => Self::EastWest,
            2 => Self::AscendingEast,
            3 => Self::AscendingWest,
            4 => Self::AscendingNorth,
            5 => Self::AscendingSouth,
            6 => Self::SouthEast,
            7 => Self::SouthWest,
            8 => Self::NorthWest,
            9 => Self::NorthEast,
            n => panic!("Invalid rail shape value: {}", n),
        }
    }

    pub fn to_value(&self) -> u8 {
        match self {
            Self::NorthSouth => 0,
            Self::EastWest => 1,
            Self::AscendingEast => 2,
            Self::AscendingWest => 3,
            Self::AscendingNorth => 4,
            Self::AscendingSouth => 5,
            Self::SouthEast => 6,
            Self::SouthWest => 7,
            Self::NorthWest => 8,
            Self::NorthEast => 9,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Flower {
    Allium,
    AzureBluet,
    BlueOrchid,
    Cornflower,
    Dandelion,
    LilacBottom,
    LilacTop,
    LilyOfTheValley,
    OxeyeDaisy,
    PeonyBottom,
    PeonyTop,
    Poppy,
    RoseBushBottom,
    RoseBushTop,
    SunflowerBottom,
    SunflowerTop,
    TulipWhite,
    TulipOrange,
    TulipPink,
    TulipRed,
    WitherRose,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Grass {
    Fern,
    Grass,
    LargeFernBottom,
    LargeFernTop,
    TallGrassBottom,
    TallGrassTop,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Seagrass {
    Seagrass,
    TallSeagrassBottom,
    TallSeagrassTop,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AnvilDamage {
    Intact,
    SlightlyDamaged,
    VeryDamaged,
}

impl From<i16> for AnvilDamage {
    fn from(damage: i16) -> Self {
        match damage {
            0 => Self::Intact,
            1 => Self::SlightlyDamaged,
            2 => Self::VeryDamaged,
            _ => panic!("Invalid anvil damage value: {}", damage),
        }
    }
}

/// Growth and attachment state for Melon and Pumpkin stems.
#[derive(Clone, Debug, PartialEq)]
pub enum StemState {
    /// Stem has not yet produced any fruit, or the fruit has been removed.
    Growing(Int0Through7),
    /// Stem has produced a fruit, and faces in the direction of that fruit.
    Attached(Surface4),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BambooLeaves {
    None,
    Small,
    Large,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Log {
    pub material: WoodMaterial,
    /// Logs with no alignment have bark (or stripped pattern) on all sides.
    pub alignment: Option<Axis3>,
    pub stripped: bool,
}

impl Log {
    pub fn has_material_of(&self, material: Material) -> bool {
        material == self.material.clone().into()
    }
}

pub type HoneyLevel = Int0Through5;
pub type FireFace = DirectionFlags6;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CommandBlockVariant {
    ChainedCommandBlock,
    CommandBlock,
    RepeatingCommandBlock,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommandBlock {
    pub variant: CommandBlockVariant,
    pub conditional: bool,
    pub facing: Surface6,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Jukebox {
    pub record: Option<Item>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OnOffState {
    On,
    Off,
}

/// Blocks, with the attributes required for full representation in the world.
///
/// Plain blocks can be created directly.
/// Blocks with attributes can be created directly, or through the use of helper functions.
/// Some of the more complex blocks have their own data structures, that are put inside
/// the corresponding enum variant (often boxed.)
#[derive(Clone, Debug, PartialEq)]
pub enum Block {
    None,
    Unknown(Option<u16>),
    Air,
    AncientDebris,
    Anvil {
        facing: Surface4,
        damage: AnvilDamage,
    },
    Andesite,
    Bamboo {
        growth_stage: Int0Through1,
        leaves: BambooLeaves,
        stage: Int0Through1,
    },
    Banner(Box<Banner>),
    Barrel {
        facing: Surface6,
    }, // TODO add block entity
    Barrier,
    Basalt {
        alignment: Axis3,
    },
    Beacon(Box<Beacon>),
    Bedrock,
    Beetroots {
        growth_stage: Int0Through3,
    },
    Beehive {
        facing: Surface4,
        honey_level: HoneyLevel,
    }, // TODO add block entity
    BeeNest {
        facing: Surface4,
        honey_level: HoneyLevel,
    }, // TODO add block entity
    Bell {
        position: BellPosition,
    }, // TODO add block entity
    Bed(Bed),
    Blackstone,
    BlastFurnace(Box<Furnace>),
    BlockOfCoal,
    BlockOfDiamond,
    BlockOfEmerald,
    BlockOfGold,
    BlockOfIron,
    BlockOfNetherite,
    BlockOfQuartz,
    BlockOfRedstone,
    BlockOfSlime,
    BlueIce,
    BoneBlock {
        alignment: Axis3,
    },
    Bookshelf,
    BrewingStand(Box<BrewingStand>),
    BrickBlock,
    BrownMushroom,
    BrownMushroomBlock {
        cap_directions: DirectionFlags6,
    },
    BrownMushroomStem {
        stem_directions: DirectionFlags6,
    },
    BubbleColumn {
        drag_direction: Surface2,
    }, // Is this even needed?
    Button(ButtonMaterial, Surface6),
    Cactus {
        growth_stage: Int0Through15,
    },
    Cake {
        pieces: Int1Through7,
    },
    Campfire {
        facing: Surface4,
        lit: bool,
        waterlogged: bool,
    },
    Carpet {
        colour: Colour,
    },
    Carrots {
        growth_stage: Int0Through7,
    },
    CartographyTable,
    CarvedPumpkin {
        facing: Surface4,
    },
    Cauldron {
        water_level: Int0Through3,
    },
    CaveAir,
    Chest(Box<Chest>),
    ChiseledNetherBricks,
    ChiseledPolishedBlackstone,
    ChiseledQuartzBlock,
    ChiseledRedSandstone,
    ChiseledSandstone,
    ChiseledStoneBricks,
    ChorusFlower {
        growth_stage: Int0Through5,
    },
    ChorusPlant,
    Clay,
    CoalOre,
    CoarseDirt,
    Cobblestone,
    Cobweb,
    CocoaBeans {
        growth_stage: Int0Through2,
        facing: Surface4,
    },
    CommandBlock(CommandBlock), // TODO add block entity
    Composter {
        fullness: Int0Through8,
    },
    Concrete {
        colour: Colour,
    },
    ConcretePowder {
        colour: Colour,
    },
    Conduit {
        waterlogged: bool,
    }, // TODO optionally add optional block entity
    Coral {
        material: CoralMaterial,
        dead: bool,
        waterlogged: bool,
    },
    CoralBlock {
        material: CoralMaterial,
        dead: bool,
    },
    CoralFan {
        material: CoralMaterial,
        facing: Surface5,
        dead: bool,
        waterlogged: bool,
    },
    CrackedNetherBricks,
    CrackedPolishedBlackstoneBricks,
    CrackedStoneBricks,
    CraftingTable,
    CrimsonFungus,
    CrimsonNylium,
    CrimsonRoots,
    CryingObsidian,
    CutRedSandstone,
    CutSandstone,
    DarkPrismarine,
    DaylightDetector,
    DeadBush,
    DiamondOre,
    Diorite,
    Dirt,
    Dispenser(Box<Dispenser>),
    Door(Door),
    DragonEgg,
    DriedKelpBlock,
    Dropper(Box<Dropper>),
    EmeraldOre,
    EnchantingTable {
        custom_name: Box<Option<String>>,
    },
    EndGateway, // TODO add block entity
    EndPortal,  // TODO add block entity
    EndPortalFrame {
        facing: Surface4,
        has_eye: bool,
    },
    EndRod {
        facing: Surface6,
    },
    EndStone,
    EndStoneBricks,
    EnderChest {
        facing: Surface4,
        waterlogged: bool,
    }, // TODO add block entity (?)
    Farmland {
        wetness: Int0Through7,
    },
    Fence {
        material: FenceMaterial,
        waterlogged: bool,
    },
    FenceGate {
        material: WoodMaterial,
        facing: Surface4,
        open: bool,
    },
    Fire {
        age: Int0Through15,
        //burning_faces: FireFace, // It seems burning faces are not a thing?
    },
    FletchingTable,
    Flower(Flower),
    FlowerPot(FlowerPot),
    FrostedIce,
    Furnace(Box<Furnace>),
    GildedBlackstone,
    Glass {
        colour: Option<Colour>,
    },
    GlassPane {
        colour: Option<Colour>,
        waterlogged: bool,
    },
    GlazedTerracotta(GlazedTerracotta),
    Glowstone,
    GoldOre,
    Granite,
    /// Grass, Fern, and two block high variants.
    Grass(Grass),
    GrassBlock,
    GrassPath,
    Gravel,
    GrindStone(SurfaceRotation12),
    HayBale {
        alignment: Axis3,
    },
    Head(Head), // TODO add block entity (used only for PlayerHead variant)
    HoneyBlock,
    HoneycombBlock,
    Hopper(Box<Hopper>),
    Ice,
    InfestedChiseledStoneBricks,
    InfestedCobblestone,
    InfestedCrackedStoneBricks,
    InfestedMossyStoneBricks,
    InfestedStone,
    InfestedStoneBricks,
    InvertedDaylightDetector,
    IronBars {
        waterlogged: bool,
    },
    IronOre,
    JackOLantern {
        facing: Surface4,
    },
    JigsawBlock {
        orientation: JigsawBlockOrientation,
    }, // TODO add block entity
    Jukebox(Box<Jukebox>),
    Kelp {
        growth_stage: Int0Through25,
    },
    Ladder {
        facing: Surface4,
        waterlogged: bool,
    },
    Lantern {
        mounted_at: Surface2,
    },
    LapisLazuliBlock,
    LapisLazuliOre,
    Lava {
        falling: bool,
        level: Int1Through7,
    },
    LavaSource,
    Leaves {
        material: LeavesMaterial,
        distance_to_trunk: Option<Int0Through7>,
        persistent: bool,
    },
    Lectern {
        facing: Surface4,
    }, // TODO add block entity (and possibly "has book" bool)
    Lever(SurfaceRotation12, OnOffState),
    LilyPad,
    LodeStone,
    Log(Log),
    Loom {
        facing: Surface4,
    },
    MagmaBlock,
    Melon,
    MelonStem {
        state: StemState,
    },
    MossyCobblestone,
    MossyStoneBricks,
    // TODO consider adding the MovingPiston technical block and block entity
    Mycelium,
    NetherBricks,
    NetherGoldOre,
    NetherPortal {
        alignment: Option<Axis2>,
    },
    NetherSprouts,
    NetherWart {
        growth_stage: Int0Through3,
    },
    NetherWartBlock,
    Netherrack,
    Noteblock(Noteblock),
    Observer {
        facing: Surface6,
    },
    Obsidian,
    PackedIce,
    Piston {
        facing: Surface6,
        extended: bool,
    },
    PistonHead {
        facing: Surface6,
    },
    Planks {
        material: WoodMaterial,
    },
    Podzol,
    PolishedAndesite,
    PolishedBasalt {
        alignment: Axis3,
    },
    PolishedBlackstone,
    PolishedBlackstoneBricks,
    PolishedDiorite,
    PolishedGranite,
    Potatoes {
        growth_stage: Int0Through7,
    },
    PressurePlate {
        material: PressurePlateMaterial,
    },
    Prismarine,
    PrismarineBricks,
    Pumpkin {
        facing: Surface4,
    },
    PumpkinStem {
        state: StemState,
    },
    PurpurBlock,
    PurpurPillar {
        alignment: Axis3,
    },
    QuartzBricks,
    QuartzOre,
    QuartzPillar {
        alignment: Axis3,
    },
    Rail {
        variant: RailType,
        shape: RailShape,
    },
    RedMushroom,
    RedMushroomBlock {
        cap_directions: DirectionFlags6,
    },
    RedMushroomStem {
        stem_directions: DirectionFlags6,
    },
    RedNetherBricks,
    RedSand,
    RedSandstone,
    RedstoneComparator {
        facing: Surface4,
    }, // TODO add block entity (?)
    RedstoneLamp,
    RedstoneOre,
    RedstoneRepeater(RedstoneRepeater),
    RedstoneSubtractor {
        facing: Surface4,
    }, // TODO add block entity (?)
    RedstoneTorch {
        attached: Surface5,
    },
    RedstoneWire, // TODO upcoming change: * or + shape, of non-connected wire
    RespawnAnchor {
        charges: Int0Through4,
    },
    Sand,
    Sandstone,
    Sapling {
        material: SaplingMaterial,
        growth_stage: Int0Through1,
    },
    Scaffolding {
        waterlogged: bool,
    },
    SeaLantern,
    SeaPickle {
        count: Int1Through4,
        waterlogged: bool,
    },
    Seagrass {
        variant: Seagrass,
    },
    Shroomlight,
    ShulkerBox(Box<ShulkerBox>),
    Sign(Box<Sign>),
    Slab(Slab),
    SmithingTable,
    Smoker(Box<Furnace>),
    SmoothQuartz,
    SmoothRedSandstone,
    SmoothSandstone,
    SmoothStone,
    Snow {
        thickness: Int1Through8,
    },
    SnowBlock,
    SoulCampfire {
        facing: Surface4,
        lit: bool,
        waterlogged: bool,
    },
    SoulFire {
        age: Int0Through15,
        burning_faces: FireFace,
    },
    SoulLantern {
        mounted_at: Surface2,
    },
    SoulTorch {
        attached: Surface5,
    },
    SoulSand,
    SoulSoil,
    Spawner, // TODO add block entity
    Sponge,
    Stairs(Stair),
    StickyPiston {
        facing: Surface6,
        extended: bool,
    },
    StickyPistonHead {
        facing: Surface6,
    },
    Stone,
    StoneBricks,
    StoneCutter {
        facing: Surface4,
    },
    StructureBlock, // TODO Add Corner, Data, Load, and Save variants. TODO add block entity
    StructureVoid,
    SugarCane {
        growth_stage: Int0Through15,
    },
    SweetBerryBush {
        growth_stage: Int0Through3,
    },
    Target,
    Terracotta {
        colour: Option<Colour>,
    },
    TNT,
    Torch {
        attached: Surface5,
    },
    Trapdoor(Trapdoor),
    TrappedChest(Box<Chest>),
    Tripwire,
    TripwireHook {
        facing: Surface4,
    },
    TurtleEgg {
        count: Int1Through4,
        age: Int0Through2,
    },
    TwistingVines {
        growth_stage: Int0Through25,
    },
    TwistingVinesPlant,
    Vines(Vines),
    Wall {
        material: WallMaterial,
        waterlogged: bool,
    },
    WarpedFungus,
    WarpedNyliym,
    WarpedRoots,
    WarpedWartBlock,
    Water {
        falling: bool,
        level: Int1Through7,
    },
    WaterSource,
    WeepingVines {
        growth_stage: Int0Through25,
    },
    WeepingVinesPlant,
    WetSponge,
    Wheat {
        growth_stage: Int0Through7,
    },
    Wool {
        colour: Colour,
    },
}

// An enum with Box values, on a 64 bit system with pointer alignment,
// has a minimum size of 16. A voxel cube containing 256^3 voxels would
// then use 256 MiB for the main "bitmap" (16 bytes per voxel), in addition
// to the contribution from Boxed structures. Thus 16 sounds like a reasonable
// size for the time being.

// Ideally the assert should check for "size of Block <= 16", but I cannot
// be bothered with finding the right hacks. If you are not on a 64 bit system
// with pointer alignment, the assert will likely not work as intended. In that
// case it can safely be commented out.
assert_eq_size!(Block, i128);

impl Block {
    /// Returns an acacia fence.
    pub const fn acacia_fence() -> Self {
        Self::Fence {
            material: FenceMaterial::Acacia,
            waterlogged: false,
        }
    }

    /// Returns a closed acacia fence gate with the doors facing in the given direction.
    pub fn acacia_fence_gate(facing: Direction) -> Self {
        Self::FenceGate {
            material: WoodMaterial::Acacia,
            facing: Surface4::try_from(facing).unwrap(),
            open: false,
        }
    }

    /// Returns an opened acacia fence gate with the doors facing in the given direction.
    pub fn acacia_fence_gate_opened(facing: Direction) -> Self {
        Self::FenceGate {
            material: WoodMaterial::Acacia,
            facing: Surface4::try_from(facing).unwrap(),
            open: true,
        }
    }

    /// Returns a Leaves block of the Acacia variant.
    pub fn acacia_leaves(persistent: bool) -> Self {
        Self::Leaves {
            material: LeavesMaterial::Acacia,
            distance_to_trunk: None,
            persistent,
        }
    }

    /// Returns a Log block of the Acacia variant, aligned with the given axis.
    pub fn acacia_log(axis: Axis3) -> Self {
        Self::Log(Log {
            material: WoodMaterial::Acacia,
            alignment: Some(axis),
            stripped: false,
        })
    }

    /// Returns a Plank block of the Acacia variant.
    pub const fn acacia_planks() -> Self {
        Self::Planks {
            material: WoodMaterial::Acacia,
        }
    }

    /// Returns a Sapling block of the Acacia variant.
    pub const fn acacia_sapling() -> Self {
        Self::Sapling {
            material: SaplingMaterial::Acacia,
            growth_stage: Int0Through1::MIN,
        }
    }

    /// Returns an activator rail of the specified shape.
    pub fn activator_rail(shape: RailShape) -> Self {
        Self::Rail {
            variant: RailType::Activator,
            shape,
        }
    }

    /// Returns a birch fence.
    pub const fn birch_fence() -> Self {
        Self::Fence {
            material: FenceMaterial::Birch,
            waterlogged: false,
        }
    }

    /// Returns a closed birch fence gate with the doors facing in the given direction.
    pub fn birch_fence_gate(facing: Direction) -> Self {
        Self::FenceGate {
            material: WoodMaterial::Birch,
            facing: Surface4::try_from(facing).unwrap(),
            open: false,
        }
    }

    /// Returns an opened birch fence gate with the doors facing in the given direction.
    pub fn birch_fence_gate_opened(facing: Direction) -> Self {
        Self::FenceGate {
            material: WoodMaterial::Birch,
            facing: Surface4::try_from(facing).unwrap(),
            open: true,
        }
    }

    /// Returns a Leaves block of the Birch variant.
    pub fn birch_leaves(persistent: bool) -> Self {
        Self::Leaves {
            material: LeavesMaterial::Birch,
            distance_to_trunk: None,
            persistent,
        }
    }

    /// Returns a Log block of the Birch variant, aligned with the given axis.
    pub fn birch_log(axis: Axis3) -> Self {
        Self::Log(Log {
            material: WoodMaterial::Birch,
            alignment: Some(axis),
            stripped: false,
        })
    }

    /// Returns a Plank block of the Birch variant.
    pub const fn birch_planks() -> Self {
        Self::Planks {
            material: WoodMaterial::Birch,
        }
    }

    /// Returns a Sapling block of the Birch variant.
    pub const fn birch_sapling() -> Self {
        Self::Sapling {
            material: SaplingMaterial::Birch,
            growth_stage: Int0Through1::MIN,
        }
    }

    /// Returns a bottom slab of the specified material.
    pub fn bottom_slab(material: Material) -> Self {
        Self::Slab(Slab {
            material: SlabMaterial::try_from(material).unwrap(),
            position: SlabVariant::Bottom,
            waterlogged: false,
        })
    }

    /// Returns a (full) cake.
    pub const fn cake() -> Self {
        Self::Cake {
            pieces: Int1Through7::MAX,
        }
    }

    /// Returns a cake with the given number of pieces remaining.
    pub fn cake_with_remaining_pieces(pieces: i8) -> Self {
        Self::Cake {
            pieces: Int1Through7::new_saturating(pieces),
        }
    }

    /// Returns a cactus block.
    pub const fn cactus() -> Self {
        Self::Cactus {
            growth_stage: Int0Through15::MIN,
        }
    }

    /// Returns a carpet of the given colour.
    pub fn carpet_with_colour(colour: Colour) -> Self {
        Self::Carpet { colour }
    }

    /// Returns a concrete block of the given colour.
    pub fn concrete_with_colour(colour: Colour) -> Self {
        Self::Concrete { colour }
    }

    /// Returns a concrete powder block of the given colour.
    pub fn concrete_powder_with_colour(colour: Colour) -> Self {
        Self::ConcretePowder { colour }
    }

    /// Returns a dark oak fence.
    pub const fn dark_oak_fence() -> Self {
        Self::Fence {
            material: FenceMaterial::DarkOak,
            waterlogged: false,
        }
    }

    /// Returns a closed dark oak fence gate with the doors facing in the given direction.
    pub fn dark_oak_fence_gate(facing: Direction) -> Self {
        Self::FenceGate {
            material: WoodMaterial::DarkOak,
            facing: Surface4::try_from(facing).unwrap(),
            open: false,
        }
    }

    /// Returns an opened dark oak fence gate with the doors facing in the given direction.
    pub fn dark_oak_fence_gate_opened(facing: Direction) -> Self {
        Self::FenceGate {
            material: WoodMaterial::DarkOak,
            facing: Surface4::try_from(facing).unwrap(),
            open: true,
        }
    }

    /// Returns a Leaves block of the Dark Oak variant.
    pub fn dark_oak_leaves(persistent: bool) -> Self {
        Self::Leaves {
            material: LeavesMaterial::DarkOak,
            distance_to_trunk: None,
            persistent,
        }
    }

    /// Returns a Log block of the Dark Oak variant, aligned with the given axis.
    pub fn dark_oak_log(axis: Axis3) -> Self {
        Self::Log(Log {
            material: WoodMaterial::DarkOak,
            alignment: Some(axis),
            stripped: false,
        })
    }

    /// Returns a Plank block of the Dark Oak variant.
    pub const fn dark_oak_planks() -> Self {
        Self::Planks {
            material: WoodMaterial::DarkOak,
        }
    }

    /// Returns a Sapling block of the Dark Oak variant.
    pub const fn dark_oak_sapling() -> Self {
        Self::Sapling {
            material: SaplingMaterial::DarkOak,
            growth_stage: Int0Through1::MIN,
        }
    }

    /// Returns a detector rail of the specified shape.
    pub fn detector_rail(shape: RailShape) -> Self {
        Self::Rail {
            variant: RailType::Detector,
            shape,
        }
    }

    /// Returns a double slab of the specified material.
    pub fn double_slab(material: Material) -> Self {
        Self::Slab(Slab {
            material: SlabMaterial::try_from(material).unwrap(),
            position: SlabVariant::Double,
            waterlogged: false,
        })
    }

    /// Returns a fire block of minimum age.
    pub const fn fire() -> Self {
        Self::Fire {
            age: Int0Through15::MIN,
        }
    }

    /// Returns an uncoloured glass block.
    pub const fn glass() -> Self {
        Self::Glass { colour: None }
    }

    /// Returns an uncoloured glass pane.
    pub const fn glass_pane() -> Self {
        Self::GlassPane {
            colour: None,
            waterlogged: false,
        }
    }

    /// Returns a glass pane of the given colour.
    pub fn glass_pane_with_colour(colour: Colour) -> Self {
        Self::GlassPane {
            colour: Some(colour),
            waterlogged: false,
        }
    }

    /// Returns a glass block of the given colour.
    pub fn glass_with_colour(colour: Colour) -> Self {
        Self::Glass {
            colour: Some(colour),
        }
    }

    /// Returns true if the block has the given colour, false otherwise.
    pub fn has_colour_of(&self, colour: Colour) -> bool {
        match self {
            Self::Banner(banner) => banner.has_colour_of(colour),
            Self::Bed(bed) => bed.has_colour_of(colour),
            Self::Carpet { colour: c } => *c == colour,
            Self::Concrete { colour: c } => *c == colour,
            Self::ConcretePowder { colour: c } => *c == colour,
            Self::Glass { colour: Some(c) } => *c == colour,
            Self::GlassPane {
                colour: Some(c), ..
            } => *c == colour,
            Self::GlazedTerracotta(gt) => gt.has_colour_of(colour),
            Self::ShulkerBox(shulker_box) => shulker_box.has_colour_of(colour),
            Self::Sign(sign) => sign.has_colour_of(colour),
            Self::Terracotta { colour: Some(c) } => *c == colour,
            Self::Wool { colour: c } => *c == colour,
            _ => false,
        }
    }

    /// Returns true if the block faces in the given direction, false otherwise.
    pub fn has_facing_of(&self, direction: Direction) -> bool {
        match self {
            Self::Anvil { facing, .. } => Direction::from(*facing) == direction,
            Self::Banner(banner) => banner.has_facing_of(direction),
            Self::Barrel { facing, .. } => Direction::from(*facing) == direction,
            Self::Beehive { facing, .. } => Direction::from(*facing) == direction,
            Self::BeeNest { facing, .. } => Direction::from(*facing) == direction,
            Self::Bed(bed) => bed.has_facing_of(direction),
            Self::BlastFurnace(furnace) => furnace.has_facing_of(direction),
            Self::Button(_, rotation) => Direction::from(*rotation) == direction,
            Self::Campfire { facing, .. } => Direction::from(*facing) == direction,
            Self::CarvedPumpkin { facing, .. } => Direction::from(*facing) == direction,
            Self::Chest(chest) => chest.has_facing_of(direction),
            Self::CocoaBeans { facing, .. } => Direction::from(*facing) == direction,
            Self::CoralFan { facing, .. } => Direction::from(*facing) == direction,
            Self::Dispenser(dispenser) => dispenser.has_facing_of(direction),
            Self::Door(door) => door.has_facing_of(direction),
            Self::Dropper(dropper) => dropper.has_facing_of(direction),
            Self::EndPortalFrame { facing, .. } => Direction::from(*facing) == direction,
            Self::EndRod { facing, .. } => Direction::from(*facing) == direction,
            Self::EnderChest { facing, .. } => Direction::from(*facing) == direction,
            Self::FenceGate { facing, .. } => Direction::from(*facing) == direction,
            Self::Furnace(furnace) => furnace.has_facing_of(direction),
            Self::GlazedTerracotta(gt) => gt.has_facing_of(direction),
            Self::GrindStone(rotation) => Direction::from(*rotation) == direction,
            Self::Head(head) => head.has_facing_of(direction),
            Self::Hopper(hopper) => hopper.has_facing_of(direction),
            Self::JackOLantern { facing, .. } => Direction::from(*facing) == direction,
            Self::Ladder { facing, .. } => Direction::from(*facing) == direction,
            Self::Lantern { mounted_at } => Direction::from(*mounted_at) == direction,
            Self::Lever(rotation, _) => Direction::from(*rotation) == direction,
            Self::Loom { facing, .. } => Direction::from(*facing) == direction,
            Self::Observer { facing, .. } => Direction::from(*facing) == direction,
            Self::Piston { facing, .. } => Direction::from(*facing) == direction,
            Self::PistonHead { facing, .. } => Direction::from(*facing) == direction,
            Self::Pumpkin { facing, .. } => Direction::from(*facing) == direction,
            Self::RedstoneComparator { facing, .. } => Direction::from(*facing) == direction,
            Self::RedstoneRepeater(repeater) => repeater.has_facing_of(direction),
            Self::RedstoneSubtractor { facing, .. } => Direction::from(*facing) == direction,
            Self::RedstoneTorch { attached, .. } => {
                Direction::from(*attached).opposite() == direction
            }
            Self::ShulkerBox(shulker_box) => shulker_box.has_facing_of(direction),
            Self::Sign(sign) => sign.has_facing_of(direction),
            Self::Smoker(furnace) => furnace.has_facing_of(direction),
            Self::SoulCampfire { facing, .. } => Direction::from(*facing) == direction,
            Self::SoulLantern { mounted_at } => Direction::from(*mounted_at) == direction,
            Self::SoulTorch { attached, .. } => Direction::from(*attached).opposite() == direction,
            Self::Stairs(stair) => stair.has_facing_of(direction),
            Self::StickyPiston { facing, .. } => Direction::from(*facing) == direction,
            Self::StoneCutter { facing, .. } => Direction::from(*facing) == direction,
            Self::Torch { attached, .. } => Direction::from(*attached).opposite() == direction,
            Self::TrappedChest(chest) => chest.has_facing_of(direction),
            Self::TripwireHook { facing, .. } => Direction::from(*facing) == direction,
            _ => false,
        }
    }

    /// Returns true if the block is made of the given material, false otherwise.
    pub fn has_material_of(&self, material: Material) -> bool {
        match self {
            Self::Button(mat, _) => Material::from(*mat) == material,
            Self::Coral { material: mat, .. } => Material::from(*mat) == material,
            Self::CoralBlock { material: mat, .. } => Material::from(*mat) == material,
            Self::CoralFan { material: mat, .. } => Material::from(*mat) == material,
            Self::Door(door) => door.has_material_of(&material),
            Self::Fence { material: mat, .. } => Material::from(*mat) == material,
            Self::FenceGate { material: mat, .. } => Material::from(*mat) == material,
            Self::Leaves { material: mat, .. } => Material::from(*mat) == material,
            Self::Log(log) => log.has_material_of(material),
            Self::Planks { material: mat, .. } => Material::from(*mat) == material,
            Self::PressurePlate { material: mat, .. } => Material::from(*mat) == material,
            Self::Sapling { material: mat, .. } => Material::from(*mat) == material,
            Self::Sign(sign) => sign.has_material_of(material),
            Self::Slab(slab) => slab.has_material_of(material),
            Self::Stairs(stair) => stair.has_material_of(material),
            Self::Trapdoor(trapdoor) => trapdoor.has_material_of(material),
            Self::Wall { material: mat, .. } => Material::from(*mat) == material,
            _ => false,
        }
    }

    /// Returns an iron bars block..
    pub const fn iron_bars() -> Self {
        Self::IronBars { waterlogged: false }
    }

    /// Returns true if the block is a beacon.
    pub fn is_beacon(&self) -> bool {
        matches!(self, Self::Beacon { .. })
    }

    /// Returns true if the block is a brewing stand.
    pub fn is_brewing_stand(&self) -> bool {
        matches!(self, Self::BrewingStand(_))
    }

    /// Returns true if the block is a chest.
    pub fn is_chest(&self) -> bool {
        matches!(self, Self::Chest(_))
    }

    /// Returns true if the block is a dispenser.
    pub fn is_dispenser(&self) -> bool {
        matches!(self, Self::Dispenser(_))
    }

    /// Returns true if the block is a dropper.
    pub fn is_dropper(&self) -> bool {
        matches!(self, Self::Dropper(_))
    }

    /// Returns true if the block is an enchanting table.
    pub fn is_enchanting_table(&self) -> bool {
        matches!(self, Self::EnchantingTable { .. })
    }

    /// Returns true if the block is an ender chest.
    pub fn is_ender_chest(&self) -> bool {
        matches!(self, Self::EnderChest { .. })
    }

    /// Returns true if the block is a furnace.
    pub fn is_furnace(&self) -> bool {
        matches!(self, Self::Furnace(_))
    }

    /// Returns true if the block is a hopper.
    pub fn is_hopper(&self) -> bool {
        matches!(self, Self::Hopper(_))
    }

    /// Returns true if the block is an observer.
    pub fn is_observer(&self) -> bool {
        matches!(self, Self::Observer { .. })
    }

    /// Returns true if the block is a piston (base).
    pub fn is_piston(&self) -> bool {
        matches!(self, Self::Piston { .. })
    }

    /// Returns true if the block is a piston head.
    pub fn is_piston_head(&self) -> bool {
        matches!(self, Self::PistonHead { .. })
    }

    /// Returns true if the block is a redstone torch.
    pub fn is_redstone_torch(&self) -> bool {
        matches!(self, Self::RedstoneTorch { .. })
    }

    /// Returns true if the block is a sign.
    pub fn is_sign(&self) -> bool {
        matches!(self, Self::Sign(_))
    }

    /// Returns true if the block cannot be moved through and fills the full block space.
    pub fn is_solid(&self) -> bool {
        matches!(
            self,
            Self::AncientDebris
                | Self::Andesite
                | Self::Barrel { .. }
                | Self::Basalt { .. }
                | Self::Bedrock
                | Self::Beehive { .. }
                | Self::BeeNest { .. }
                | Self::Blackstone
                | Self::BlastFurnace(_)
                | Self::BlockOfCoal
                | Self::BlockOfDiamond
                | Self::BlockOfEmerald
                | Self::BlockOfGold
                | Self::BlockOfIron
                | Self::BlockOfNetherite
                | Self::BlockOfQuartz
                | Self::BlockOfRedstone
                | Self::BlockOfSlime
                | Self::BlueIce
                | Self::BoneBlock { .. }
                | Self::Bookshelf
                | Self::BrickBlock
                | Self::BrownMushroomBlock { .. }
                | Self::BrownMushroomStem { .. }
                | Self::Cactus { .. }
                | Self::CarvedPumpkin { .. }
                | Self::ChiseledNetherBricks
                | Self::ChiseledPolishedBlackstone
                | Self::ChiseledQuartzBlock
                | Self::ChiseledRedSandstone
                | Self::ChiseledSandstone
                | Self::ChiseledStoneBricks
                | Self::Clay
                | Self::CoalOre
                | Self::CoarseDirt
                | Self::Cobblestone
                | Self::CommandBlock(_)
                | Self::Concrete { .. }
                | Self::ConcretePowder { .. }
                | Self::CoralBlock { .. }
                | Self::CrackedNetherBricks
                | Self::CrackedPolishedBlackstoneBricks
                | Self::CrackedStoneBricks
                | Self::CraftingTable
                | Self::CryingObsidian
                | Self::CutRedSandstone
                | Self::CutSandstone
                | Self::DarkPrismarine
                | Self::DiamondOre
                | Self::Diorite
                | Self::Dirt
                | Self::Dispenser(_)
                | Self::DriedKelpBlock
                | Self::Dropper(_)
                | Self::EmeraldOre
                | Self::EndStone
                | Self::EndStoneBricks
                | Self::FletchingTable
                | Self::FrostedIce
                | Self::Furnace(_)
                | Self::GildedBlackstone
                | Self::Glass { .. }
                | Self::GlazedTerracotta(_)
                | Self::Glowstone
                | Self::GoldOre
                | Self::Granite
                | Self::Grass(_)
                | Self::GrassBlock
                | Self::Gravel
                | Self::HayBale { .. }
                | Self::HoneyBlock
                | Self::HoneycombBlock
                | Self::Ice
                | Self::InfestedChiseledStoneBricks
                | Self::InfestedCobblestone
                | Self::InfestedCrackedStoneBricks
                | Self::InfestedMossyStoneBricks
                | Self::InfestedStone
                | Self::InfestedStoneBricks
                | Self::IronOre
                | Self::JackOLantern { .. }
                | Self::Jukebox(_)
                | Self::LapisLazuliBlock
                | Self::LapisLazuliOre
                | Self::Leaves { .. }
                | Self::Log(_)
                | Self::Loom { .. }
                | Self::MagmaBlock
                | Self::Melon
                | Self::MossyCobblestone
                | Self::MossyStoneBricks
                | Self::Mycelium
                | Self::NetherBricks
                | Self::NetherGoldOre
                | Self::NetherWartBlock
                | Self::Netherrack
                | Self::Noteblock(_)
                | Self::Observer { .. }
                | Self::Obsidian
                | Self::PackedIce
                | Self::Piston { .. }
                | Self::Planks { .. }
                | Self::Podzol
                | Self::PolishedAndesite
                | Self::PolishedBasalt { .. }
                | Self::PolishedBlackstone
                | Self::PolishedBlackstoneBricks
                | Self::PolishedDiorite
                | Self::PolishedGranite
                | Self::Prismarine
                | Self::PrismarineBricks
                | Self::Pumpkin { .. }
                | Self::PurpurBlock
                | Self::PurpurPillar { .. }
                | Self::QuartzBricks
                | Self::QuartzOre
                | Self::QuartzPillar { .. }
                | Self::RedMushroomBlock { .. }
                | Self::RedMushroomStem { .. }
                | Self::RedNetherBricks
                | Self::RedSand
                | Self::RedSandstone
                | Self::RedstoneLamp
                | Self::RedstoneOre
                | Self::RespawnAnchor { .. }
                | Self::Sand
                | Self::Sandstone
                | Self::SeaLantern
                | Self::Shroomlight
                | Self::ShulkerBox(_)
                | Self::Slab(Slab {
                    position: SlabVariant::Double,
                    ..
                })
                | Self::SmithingTable
                | Self::Smoker { .. }
                | Self::SmoothQuartz
                | Self::SmoothRedSandstone
                | Self::SmoothSandstone
                | Self::SmoothStone
                | Self::SnowBlock
                | Self::SoulSand
                | Self::SoulSoil
                | Self::Spawner
                | Self::Sponge
                | Self::StickyPiston { .. }
                | Self::Stone
                | Self::StoneBricks
                | Self::Target
                | Self::Terracotta { .. }
                | Self::TNT
                | Self::WarpedWartBlock
                | Self::WetSponge
                | Self::Wool { .. }
        )
    }

    /// Returns true if the block is a stair.
    pub fn is_stairs(&self) -> bool {
        matches!(self, Self::Stairs(_))
    }

    /// Returns true if the block is a sticky piston (base).
    pub fn is_sticky_piston(&self) -> bool {
        matches!(self, Self::StickyPiston { .. })
    }

    /// Returns true if the block is a torch.
    pub fn is_torch(&self) -> bool {
        matches!(self, Self::Torch { .. })
    }

    /// Returns true if the block is a trapped chest.
    pub fn is_trapped_chest(&self) -> bool {
        matches!(self, Self::TrappedChest(_))
    }

    /// Returns true if the block is a wall.
    ///
    /// Please note that walls are not full blocks; Wall is the type of block
    /// that represents e.g. stone walls, often used for fencing. Their
    /// collision box is narrower than a block, but extends higher upwards.
    pub fn is_wall(&self) -> bool {
        matches!(self, Self::Wall { .. })
    }

    /// Returns a jack o'lantern facing in the given direction.
    pub fn jack_o_lantern(facing: Direction) -> Self {
        Self::JackOLantern {
            facing: Surface4::try_from(facing).unwrap(),
        }
    }

    /// Returns an empty jukebox.
    pub fn jukebox() -> Self {
        Self::Jukebox(Box::new(Jukebox { record: None }))
    }

    /// Returns a jukebox with a disk of the given recording contained.
    pub fn jukebox_with_recording(recording: crate::item::Recording) -> Self {
        Self::Jukebox(Box::new(Jukebox {
            record: Some(crate::item::Item::new_record(recording)),
        }))
    }

    /// Returns an jungle fence.
    pub const fn jungle_fence() -> Self {
        Self::Fence {
            material: FenceMaterial::Jungle,
            waterlogged: false,
        }
    }

    /// Returns a closed jungle fence gate with the doors facing in the given direction.
    pub fn jungle_fence_gate(facing: Direction) -> Self {
        Self::FenceGate {
            material: WoodMaterial::Jungle,
            facing: Surface4::try_from(facing).unwrap(),
            open: false,
        }
    }

    /// Returns an opened jungle fence gate with the doors facing in the given direction.
    pub fn jungle_fence_gate_opened(facing: Direction) -> Self {
        Self::FenceGate {
            material: WoodMaterial::Jungle,
            facing: Surface4::try_from(facing).unwrap(),
            open: true,
        }
    }

    /// Returns a Leaves block of the Jungle variant.
    pub fn jungle_leaves(persistent: bool) -> Self {
        Self::Leaves {
            material: LeavesMaterial::Jungle,
            distance_to_trunk: None,
            persistent,
        }
    }

    /// Returns a Log block of the Jungle variant, aligned with the given axis.
    pub fn jungle_log(axis: Axis3) -> Self {
        Self::Log(Log {
            material: WoodMaterial::Jungle,
            alignment: Some(axis),
            stripped: false,
        })
    }

    /// Returns a Plank block of the Jungle variant.
    pub const fn jungle_planks() -> Self {
        Self::Planks {
            material: WoodMaterial::Jungle,
        }
    }

    /// Returns a Sapling block of the Jungle variant.
    pub const fn jungle_sapling() -> Self {
        Self::Sapling {
            material: SaplingMaterial::Jungle,
            growth_stage: Int0Through1::MIN,
        }
    }

    /// Returns a ladder block of the given facing.
    pub fn ladder(facing: Direction) -> Self {
        Self::Ladder {
            facing: Surface4::try_from(facing).unwrap(),
            waterlogged: false,
        }
    }

    /// Returns a Lava or LavaSource block with the given lava level.
    /// Note that lava, if not in the nether, goes down by two levels per block.
    pub fn lava(level: i8) -> Self {
        if level == 8 {
            Block::LavaSource
        } else {
            Block::Lava {
                falling: false,
                level: Int1Through7::new_saturating(level),
            }
        }
    }

    /// Returns a lever in the given orientation.
    pub fn lever(facing: Direction) -> Self {
        Self::lever_off(facing)
    }

    /// Returns a switched off lever in the given orientation.
    pub fn lever_off(facing: Direction) -> Self {
        Self::Lever(
            SurfaceRotation12::try_from(facing).unwrap(),
            OnOffState::Off,
        )
    }

    /// Returns a switched on lever in the given orientation.
    pub fn lever_on(facing: Direction) -> Self {
        Self::Lever(SurfaceRotation12::try_from(facing).unwrap(), OnOffState::On)
    }

    /// Returns a nether brick fence.
    pub const fn nether_brick_fence() -> Self {
        Self::Fence {
            material: FenceMaterial::NetherBrick,
            waterlogged: false,
        }
    }

    /// Returns a oak button of the given placemnet.
    pub fn oak_button(direction: Direction) -> Self {
        Self::Button(ButtonMaterial::Oak, Surface6::try_from(direction).unwrap())
    }

    /// Returns an oak fence.
    pub const fn oak_fence() -> Self {
        Self::Fence {
            material: FenceMaterial::Oak,
            waterlogged: false,
        }
    }

    /// Returns a closed oak fence gate with the doors facing in the given direction.
    pub fn oak_fence_gate(facing: Direction) -> Self {
        Self::FenceGate {
            material: WoodMaterial::Oak,
            facing: Surface4::try_from(facing).unwrap(),
            open: false,
        }
    }

    /// Returns an opened oak fence gate with the doors facing in the given direction.
    pub fn oak_fence_gate_opened(facing: Direction) -> Self {
        Self::FenceGate {
            material: WoodMaterial::Oak,
            facing: Surface4::try_from(facing).unwrap(),
            open: true,
        }
    }

    /// Returns a Leaves block of the Oak variant.
    pub fn oak_leaves(persistent: bool) -> Self {
        Self::Leaves {
            material: LeavesMaterial::Oak,
            distance_to_trunk: None,
            persistent,
        }
    }

    /// Returns a Log block of the Oak variant, aligned with the given axis.
    pub fn oak_log(axis: Axis3) -> Self {
        Self::Log(Log {
            material: WoodMaterial::Oak,
            alignment: Some(axis),
            stripped: false,
        })
    }

    /// Returns a Plank block of the Oak variant.
    pub const fn oak_planks() -> Self {
        Self::Planks {
            material: WoodMaterial::Oak,
        }
    }

    /// Returns a Sapling block of the Oak variant.
    pub const fn oak_sapling() -> Self {
        Self::Sapling {
            material: SaplingMaterial::Oak,
            growth_stage: Int0Through1::MIN,
        }
    }

    /// Returns a powered rail of the specified shape.
    pub fn powered_rail(shape: RailShape) -> Self {
        Self::Rail {
            variant: RailType::Powered,
            shape,
        }
    }

    /// Returns a pressure plate with the specified material.
    pub fn pressure_plate(material: Material) -> Self {
        Self::PressurePlate {
            material: PressurePlateMaterial::try_from(material).unwrap(),
        }
    }

    /// Returns a pumpkin facing in the given direction.
    pub fn pumpkin(facing: Direction) -> Self {
        Self::Pumpkin {
            facing: Surface4::try_from(facing).unwrap(),
        }
    }

    /// Returns a rail of the specified shape.
    pub fn rail(shape: RailShape) -> Self {
        Self::Rail {
            variant: RailType::Normal,
            shape,
        }
    }

    /// Sets the age or growth_stage field of the block to the given value,
    /// clamped to the valid range for the field of the particular block.
    pub fn set_age_to(&mut self, new_age: i8) {
        match self {
            Self::Bamboo {
                ref mut growth_stage,
                ..
            } => {
                *growth_stage = Int0Through1::new_saturating(new_age);
            }
            Self::Beetroots {
                ref mut growth_stage,
                ..
            } => {
                *growth_stage = Int0Through3::new_saturating(new_age);
            }
            Self::Cactus {
                ref mut growth_stage,
                ..
            } => {
                *growth_stage = Int0Through15::new_saturating(new_age);
            }
            Self::Carrots {
                ref mut growth_stage,
                ..
            } => {
                *growth_stage = Int0Through7::new_saturating(new_age);
            }
            Self::ChorusFlower {
                ref mut growth_stage,
                ..
            } => {
                *growth_stage = Int0Through5::new_saturating(new_age);
            }
            Self::CocoaBeans {
                ref mut growth_stage,
                ..
            } => {
                *growth_stage = Int0Through2::new_saturating(new_age);
            }
            Self::Fire { ref mut age, .. } => {
                *age = Int0Through15::new_saturating(new_age);
            }
            // TODO FrostedIce
            Self::Kelp {
                ref mut growth_stage,
                ..
            } => {
                *growth_stage = Int0Through25::new_saturating(new_age);
            }
            // TODO MelonStem
            Self::NetherWart {
                ref mut growth_stage,
                ..
            } => {
                *growth_stage = Int0Through3::new_saturating(new_age);
            }
            Self::Potatoes {
                ref mut growth_stage,
                ..
            } => {
                *growth_stage = Int0Through7::new_saturating(new_age);
            }
            // TODO PumpkinStem
            Self::Sapling {
                ref mut growth_stage,
                ..
            } => {
                *growth_stage = Int0Through1::new_saturating(new_age);
            }
            Self::SoulFire { ref mut age, .. } => {
                *age = Int0Through15::new_saturating(new_age);
            }
            Self::SugarCane {
                ref mut growth_stage,
                ..
            } => {
                *growth_stage = Int0Through15::new_saturating(new_age);
            }
            Self::SweetBerryBush {
                ref mut growth_stage,
                ..
            } => {
                *growth_stage = Int0Through3::new_saturating(new_age);
            }
            Self::TurtleEgg { ref mut age, .. } => {
                *age = Int0Through2::new_saturating(new_age);
            }
            Self::TwistingVines {
                ref mut growth_stage,
                ..
            } => {
                *growth_stage = Int0Through25::new_saturating(new_age);
            }
            Self::WeepingVines {
                ref mut growth_stage,
                ..
            } => {
                *growth_stage = Int0Through25::new_saturating(new_age);
            }
            Self::Wheat {
                ref mut growth_stage,
                ..
            } => {
                *growth_stage = Int0Through7::new_saturating(new_age);
            }
            _ => (),
        }
    }

    /// Returns a one layer thick snow block.
    pub const fn snow_layer() -> Self {
        Self::Snow {
            thickness: Int1Through8::MIN,
        }
    }

    /// Returns a snow block of the given thickness.
    pub fn snow_layers(thickness: i8) -> Self {
        Self::Snow {
            thickness: Int1Through8::new_saturating(thickness),
        }
    }

    /// Returns a spruce fence.
    pub const fn spruce_fence() -> Self {
        Self::Fence {
            material: FenceMaterial::Spruce,
            waterlogged: false,
        }
    }

    /// Returns a closed spruce fence gate with the doors facing in the given direction.
    pub fn spruce_fence_gate(facing: Direction) -> Self {
        Self::FenceGate {
            material: WoodMaterial::Spruce,
            facing: Surface4::try_from(facing).unwrap(),
            open: false,
        }
    }

    /// Returns an opened spruce fence gate with the doors facing in the given direction.
    pub fn spruce_fence_gate_opened(facing: Direction) -> Self {
        Self::FenceGate {
            material: WoodMaterial::Spruce,
            facing: Surface4::try_from(facing).unwrap(),
            open: true,
        }
    }

    /// Returns a Leaves block of the Spruce variant.
    pub fn spruce_leaves(persistent: bool) -> Self {
        Self::Leaves {
            material: LeavesMaterial::Spruce,
            distance_to_trunk: None,
            persistent,
        }
    }

    /// Returns a Log block of the Spruce variant, aligned with the given axis.
    pub fn spruce_log(axis: Axis3) -> Self {
        Self::Log(Log {
            material: WoodMaterial::Spruce,
            alignment: Some(axis),
            stripped: false,
        })
    }

    /// Returns a Plank block of the Spruce variant.
    pub const fn spruce_planks() -> Self {
        Self::Planks {
            material: WoodMaterial::Spruce,
        }
    }

    /// Returns a Sapling block of the Spruce variant.
    pub const fn spruce_sapling() -> Self {
        Self::Sapling {
            material: SaplingMaterial::Spruce,
            growth_stage: Int0Through1::MIN,
        }
    }

    /// Returns a stone button of the given placemnet.
    pub fn stone_button(direction: Direction) -> Self {
        Self::Button(
            ButtonMaterial::Stone,
            Surface6::try_from(direction).unwrap(),
        )
    }

    /// Returns a sugar cane block.
    pub const fn sugar_cane() -> Self {
        Self::SugarCane {
            growth_stage: Int0Through15::MIN,
        }
    }

    /// Returns an uncoloured terracotta block.
    pub const fn terracotta() -> Self {
        Self::Terracotta { colour: None }
    }

    /// Returns a terracotta block of the given colour.
    pub fn terracotta_with_colour(colour: Colour) -> Self {
        Self::Terracotta {
            colour: Some(colour),
        }
    }

    /// Returns a top slab of the specified material.
    pub fn top_slab(material: Material) -> Self {
        Self::Slab(Slab {
            material: SlabMaterial::try_from(material).unwrap(),
            position: SlabVariant::Top,
            waterlogged: false,
        })
    }

    /// Returns a torch facing up.
    pub const fn torch() -> Self {
        Self::Torch {
            attached: Surface5::Down,
        }
    }

    /// Returns a Water or WaterSource block with the given water level.
    pub fn water(level: i8) -> Self {
        if level == 8 {
            Block::WaterSource
        } else {
            Block::Water {
                falling: false,
                level: Int1Through7::new_saturating(level),
            }
        }
    }

    /// Returns a wooden button of the given placemnet.
    pub fn wooden_button(direction: Direction) -> Self {
        Self::Button(ButtonMaterial::Oak, Surface6::try_from(direction).unwrap())
    }

    /// Returns a wool block of the given colour.
    pub fn wool_with_colour(colour: Colour) -> Self {
        Self::Wool { colour }
    }

    /// Returns a wheat block of minimum age.
    pub const fn wheat() -> Self {
        Self::Wheat {
            growth_stage: Int0Through7::MIN,
        }
    }
}
