use std::convert::TryFrom;

mod banner;
mod bed;
mod chest;
mod dispenser;
mod door;
mod dropper;
mod furnace;
mod hopper;
mod noteblock;
mod redstone_repeater;
mod shulker_box;
mod sign;
mod stair;

pub use crate::block::banner::*;
pub use crate::block::bed::*;
pub use crate::block::chest::*;
pub use crate::block::dispenser::*;
pub use crate::block::door::*;
pub use crate::block::dropper::*;
pub use crate::block::furnace::*;
pub use crate::block::hopper::*;
pub use crate::block::noteblock::*;
pub use crate::block::redstone_repeater::*;
pub use crate::block::shulker_box::*;
pub use crate::block::sign::*;
pub use crate::block::stair::*;

use crate::bounded_ints::*;
use crate::colour::*;
use crate::inventory::Inventory;
use crate::item::Item;
use crate::material::*;
use crate::positioning::*;
use crate::status_effect::StatusEffect;

#[derive(Clone, Debug, PartialEq)]
pub struct Beacon {
    pub lock: Option<String>,
    pub levels: i32, // TODO change type to integer with valid range
    pub primary: Option<StatusEffect>,
    pub secondary: Option<StatusEffect>,
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum RailType {
    Activator,
    Detector,
    Normal,
    Powered,
}

#[derive(Clone, Debug, PartialEq)]
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
}

#[derive(Clone, Debug, PartialEq)]
pub struct BrewingStand {
    pub custom_name: Option<String>,
    pub lock: Option<String>,
    pub items: Inventory,
    pub brew_time: i16, // TODO change to integer with valid range
    pub fuel: i8,       // TODO change to integer with valid range
}

#[derive(Clone, Debug, PartialEq)]
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
    TulipLightGray,
    TulipOrange,
    TulipPink,
    TulipRed,
    WitherRose,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Grass {
    Fern,
    Grass,
    LargeFernBottom,
    LargeFernTop,
    TallGrassBottom,
    TallGrassTop,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Seagrass {
    Seagrass,
    TallSeagrassBottom,
    TallSeagrassTop,
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
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

bounded_integer! {
    #[repr(i8)]
    pub struct HoneyLevel { 0..=5 }
}

// TODO consider using BitSet here
#[derive(Clone, Debug, PartialEq)]
pub struct DirectionFlags6 {
    pub east: bool,
    pub down: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}

pub type ChorusPlantConnections = DirectionFlags6;
pub type FireFace = DirectionFlags6;

#[derive(Clone, Debug, PartialEq)]
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
pub enum HeadVariant {
    CreeperHead,
    DragonHead,
    PlayerHead,
    SkeletonSkull,
    WitherSkeletonSkull,
    ZombieHead,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Head {
    pub variant: HeadVariant,
    pub placement: WallOrRotatedOnFloor,
    pub waterlogged: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Jukebox {
    pub record: Option<Item>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PottedPlant {
    AcaciaSapling,
    Allium,
    AzureBluet,
    Bamboo,
    BirchSapling,
    BlueOrchid,
    BrownMushroom,
    Cactus,
    Cornflower,
    CrimsonFungus,
    CrimsonRoots,
    Dandelion,
    DarkOakSapling,
    DeadBush,
    Fern,
    JungleSapling,
    LilyOfTheValley,
    OakSapling,
    OxeyeDaisy,
    Poppy,
    RedMushroom,
    SpruceSapling,
    TulipOrange,
    TulipPink,
    TulipRed,
    TulipWhite,
    WarpedFungus,
    Warpedroots,
    WitherRose,
}

#[derive(Clone, Debug, PartialEq)]
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
    ChorusPlant {
        connections: ChorusPlantConnections,
    },
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
    FlowerPot {
        plant: Option<PottedPlant>,
    },
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
    GlazedTerracotta {
        colour: Colour,
        facing: Surface4,
    },
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
    MushroomStem {
        stem_directions: DirectionFlags6,
    },
    Mycelium,
    NetherBricks,
    NetherGoldOre,
    NetherPortal {
        alignment: Option<Axis2>,
    },
    NetherQuartzOre,
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
    QuartsPillar {
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
    SlimeBlock,
    SmithingTable,
    Smoker {
        facing: Surface4,
        lit: bool,
    }, // TODO add block entity
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
    Trapdoor {
        material: DoorMaterial,
        hinge_at: Edge8,
        open: bool,
        waterlogged: bool,
    },
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
    Vines {
        // NB should attach to all neighbouring blocks by default
        anchored_at: DirectionFlags6,
    },
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
    pub fn acacia_fence() -> Self {
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
    pub fn acacia_planks() -> Self {
        Self::Planks {
            material: WoodMaterial::Acacia,
        }
    }

    /// Returns a Sapling block of the Acacia variant.
    pub fn acacia_sapling() -> Self {
        Self::Sapling {
            material: SaplingMaterial::Acacia,
            growth_stage: Int0Through1::MIN,
        }
    }

    /// Returns a birch fence.
    pub fn birch_fence() -> Self {
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
    pub fn birch_planks() -> Self {
        Self::Planks {
            material: WoodMaterial::Birch,
        }
    }

    /// Returns a Sapling block of the Birch variant.
    pub fn birch_sapling() -> Self {
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
    pub fn cake() -> Self {
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
    pub fn cactus() -> Self {
        Self::Cactus {
            growth_stage: Int0Through15::MIN,
        }
    }

    /// Returns a dark oak fence.
    pub fn dark_oak_fence() -> Self {
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
    pub fn dark_oak_planks() -> Self {
        Self::Planks {
            material: WoodMaterial::DarkOak,
        }
    }

    /// Returns a Sapling block of the Dark Oak variant.
    pub fn dark_oak_sapling() -> Self {
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
    pub fn fire() -> Self {
        Self::Fire {
            age: Int0Through15::MIN,
        }
    }

    /// Returns an uncoloured glass block.
    pub fn glass() -> Self {
        Self::Glass { colour: None }
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
            Self::GlazedTerracotta { colour: c, .. } => *c == colour,
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
            Self::Door(door) => door.has_facing_of(&direction),
            Self::Dropper(dropper) => dropper.has_facing_of(direction),
            Self::EndPortalFrame { facing, .. } => Direction::from(*facing) == direction,
            Self::EndRod { facing, .. } => Direction::from(*facing) == direction,
            Self::EnderChest { facing, .. } => Direction::from(*facing) == direction,
            Self::FenceGate { facing, .. } => Direction::from(*facing) == direction,
            Self::Furnace(furnace) => furnace.has_facing_of(direction),
            Self::GlazedTerracotta { facing, .. } => Direction::from(*facing) == direction,
            Self::GrindStone(rotation) => Direction::from(*rotation) == direction,
            // TODO Head
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
            Self::Smoker { facing, .. } => Direction::from(*facing) == direction,
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
            Self::Trapdoor { material: mat, .. } => Material::from(*mat) == material,
            Self::Wall { material: mat, .. } => Material::from(*mat) == material,
            _ => false,
        }
    }

    /// Returns true if the block is a chest.
    pub fn is_chest(&self) -> bool {
        match self {
            Self::Chest(_) => true,
            _ => false,
        }
    }

    /// Returns true if the block is a dispenser.
    pub fn is_dispenser(&self) -> bool {
        match self {
            Self::Dispenser(_) => true,
            _ => false,
        }
    }

    /// Returns true if the block is a furnace.
    pub fn is_furnace(&self) -> bool {
        match self {
            Self::Furnace(_) => true,
            _ => false,
        }
    }

    /// Returns true if the block is a piston (base).
    pub fn is_piston(&self) -> bool {
        match self {
            Self::Piston { .. } => true,
            _ => false,
        }
    }

    /// Returns true if the block is a piston head.
    pub fn is_piston_head(&self) -> bool {
        match self {
            Self::PistonHead { .. } => true,
            _ => false,
        }
    }

    /// Returns true if the block is a redstone torch.
    pub fn is_redstone_torch(&self) -> bool {
        match self {
            Self::RedstoneTorch { .. } => true,
            _ => false,
        }
    }

    /// Returns true if the block is a sign.
    pub fn is_sign(&self) -> bool {
        match self {
            Self::Sign(_) => true,
            _ => false,
        }
    }

    /// Returns true if the block is a stair.
    pub fn is_stairs(&self) -> bool {
        match self {
            Self::Stairs(_) => true,
            _ => false,
        }
    }

    /// Returns true if the block is a sticky piston (base).
    pub fn is_sticky_piston(&self) -> bool {
        match self {
            Self::StickyPiston { .. } => true,
            _ => false,
        }
    }

    /// Returns a sugar cane block.
    pub fn sugar_cane() -> Self {
        Self::SugarCane {
            growth_stage: Int0Through15::MIN,
        }
    }

    /// Returns true if the block is a torch.
    pub fn is_torch(&self) -> bool {
        match self {
            Self::Torch { .. } => true,
            _ => false,
        }
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
    pub fn jungle_fence() -> Self {
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
    pub fn jungle_planks() -> Self {
        Self::Planks {
            material: WoodMaterial::Jungle,
        }
    }

    /// Returns a Sapling block of the Jungle variant.
    pub fn jungle_sapling() -> Self {
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
    pub fn nether_brick_fence() -> Self {
        Self::Fence {
            material: FenceMaterial::NetherBrick,
            waterlogged: false,
        }
    }

    /// Returns an oak fence.
    pub fn oak_fence() -> Self {
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
    pub fn oak_planks() -> Self {
        Self::Planks {
            material: WoodMaterial::Oak,
        }
    }

    /// Returns a Sapling block of the Oak variant.
    pub fn oak_sapling() -> Self {
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
    pub fn snow_layer() -> Self {
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
    pub fn spruce_fence() -> Self {
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
    pub fn spruce_planks() -> Self {
        Self::Planks {
            material: WoodMaterial::Spruce,
        }
    }

    /// Returns a Sapling block of the Spruce variant.
    pub fn spruce_sapling() -> Self {
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

    /// Returns a top slab of the specified material.
    pub fn top_slab(material: Material) -> Self {
        Self::Slab(Slab {
            material: SlabMaterial::try_from(material).unwrap(),
            position: SlabVariant::Top,
            waterlogged: false,
        })
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
    pub fn wheat() -> Self {
        Self::Wheat {
            growth_stage: Int0Through7::MIN,
        }
    }
}
