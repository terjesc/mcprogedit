use crate::banner::*;
use crate::bounded_ints::*;
use crate::colour::*;
use crate::material::*;
use crate::positioning::*;

/// Doors are two blocks high. Which block is this?
#[derive(Clone, Debug, PartialEq)]
pub enum DoorHalf {
    /// Bottom block of the door
    Lower,
    /// Top block of the door.
    Upper,
}

/// For doors, what way they are hinged. Left/Right relative to the direction
/// the door is Facing. (E.g. if Facing North, Left means on the West side,
/// and Right means on the East side.)
#[derive(Clone, Debug, PartialEq)]
pub enum Hinge {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sign {
    material: WoodMaterial,
    placement: WallOrRotatedOnFloor,
    waterlogged: bool,
    colour: Colour,
    text1: String,
    text2: String,
    text3: String,
    text4: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SlabVariant {
    Bottom,
    Double,
    Top,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Slab {
    material: SlabMaterial,
    position: SlabVariant,
    waterlogged: bool,
}

/// Stair shape is not configurable, as it depend on neighbouring stairs.
/// Stair shape is either automatically calculated on save, or the block is
/// flagged for update so that it will be automatically corrected in-game.
#[derive(Clone, Debug, PartialEq)]
pub struct Stair {
    pub material: StairMaterial,
    pub position: Edge8,
    pub waterlogged: bool,
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

bounded_integer! {
    #[repr(i8)]
    pub struct HoneyLevel { 0..=5 }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BedEnd {
    Foot,
    Head,
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
pub enum ChestVariant {
    Left,
    Right,
    Single,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Chest {
    pub facing: Surface4,
    pub variant: ChestVariant,
    pub waterlogged: bool,
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

#[derive(Clone, Debug, PartialEq)]
pub enum Pitch {
    Fs0 = 0,
    G0,
    Gs0,
    A0,
    As0,
    B0,
    C1,
    Cs1,
    D1,
    Ds1,
    E1,
    F1,
    Fs1,
    G1,
    Gs1,
    A1,
    As1,
    B1,
    C2,
    Cs2,
    D2,
    Ds2,
    E2,
    F2,
    Fs2,
}

impl Pitch {
    pub fn from_value(value: u8) -> Self {
        match value {
            0 => Pitch::Fs0,
            1 => Pitch::G0,
            2 => Pitch::Gs0,
            3 => Pitch::A0,
            4 => Pitch::As0,
            5 => Pitch::B0,
            6 => Pitch::C1,
            7 => Pitch::Cs1,
            8 => Pitch::D1,
            9 => Pitch::Ds1,
            10 => Pitch::E1,
            11 => Pitch::F1,
            12 => Pitch::Fs1,
            13 => Pitch::G1,
            14 => Pitch::Gs1,
            15 => Pitch::A1,
            16 => Pitch::As1,
            17 => Pitch::B1,
            18 => Pitch::C2,
            19 => Pitch::Cs2,
            20 => Pitch::D2,
            21 => Pitch::Ds2,
            22 => Pitch::E2,
            23 => Pitch::F2,
            24 => Pitch::Fs2,
            _ => panic!("Pitch value out of range!"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Block {
    None,
    Air,
    AncientDebris,
    Anvil {
        facing: Surface4,
        damage: AnvilDamage,
    },
    Andesite,
    Bamboo {
        age: Int0Through1,
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
    Beacon, // TODO add block entity
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
    Bed {
        colour: Colour,
        facing: Surface4,
        end: BedEnd,
    },
    Blackstone,
    BlastFurnace {
        facing: Surface4,
    }, // TODO add block entity
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
    BrewingStand, // TODO add block entity
    BrownMushroom,
    BrownMushroomBlock {
        cap_directions: DirectionFlags6,
    },
    BubbleColumn {
        drag_direction: Surface2,
    }, // Is this even needed?
    Button(ButtonMaterial, SurfaceRotation12),
    Cactus {
        age: Int0Through15,
    },
    Cake {
        bites: Int0Through6,
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
    Chest(Chest), // TODO add block entity
    ChiseledNetherBricks,
    ChiseledPolishedBlackstone,
    ChiseledQuartzBlock,
    ChiseledRedSandstone,
    ChiseledSandstone,
    ChiseledStoneBricks,
    ChorusFlower {
        age: Int0Through5,
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
        age: Int0Through2,
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
    Dispenser {
        facing: Surface6,
    }, // TODO add block entity
    Door {
        material: DoorMaterial,
        facing: Surface4,
        half: DoorHalf,
        hinge: Hinge,
        open: bool,
    },
    DragonEgg,
    DriedKelpBlock,
    Dropper {
        facing: Surface6,
    }, // TODO add block entity
    EmeraldOre,
    EnchantingTable, // TODO add block entity
    EndGateway,      // TODO add block entity
    EndPortal,       // TODO add block entity
    EndPortalFrame {
        facing: Surface4,
        has_eye: bool,
    },
    Endrod {
        facing: Surface4,
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
        burning_faces: FireFace,
    },
    FletchingTable,
    Flower(Flower),
    FlowerPot {
        plant: Option<PottedPlant>,
    },
    FrostedIce,
    Furnace {
        facing: Surface4,
        lit: bool,
    }, // TODO add block entity
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
    Hopper {
        facing: Surface5,
    }, // TODO add block entity
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
    Jukebox, // TODO add block entity (and potentially "has record" bool)
    Kelp {
        age: Int0Through25,
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
    LavaSource, // TODO handle magic (that is, the "flowing" state)
    Leaves {
        material: LeavesMaterial,
        distance_to_trunk: Int0Through7,
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
        alignment: Axis2,
    },
    NetherQuartzOre,
    NetherSprouts,
    NetherWart {
        age: Int0Through3,
    },
    NetherWartBlock,
    Netherrack,
    Noteblock {
        pitch: Pitch,
    }, // TODO instrument depend on neighbouring block below.
    // Figure out if an "instrument" field is needed.
    Observer {
        facing: Surface6,
    }, // TODO consider if a "powered" field is useful.
    Obsidian,
    PackedIce,
    Piston {
        facing: Surface6,
    }, // TODO consider adding "extended" field and PistonHead block.
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
    Pumpkin,
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
    RedstoneRepeater {
        facing: Surface4,
        delay: Int1Through4,
    },
    RedstoneSubtractor {
        facing: Surface4,
    }, // TODO add block entity (?)
    RedstoneTorch {
        facing: Surface5,
    },
    RedstoneWire, // TODO upcoming change: * or + shape, of non-connected wire
    RespawnAnchor {
        charges: Int0Through4,
    },
    Sand,
    Sandstone,
    Sapling {
        material: SaplingMaterial,
        stage: Int0Through1,
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
    ShulkerBox {
        colour: Option<Colour>,
        facing: Surface6,
    }, // TODO add block entity
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
        facing: Surface5,
    },
    SoulSand,
    SoulSoil,
    Spawner, // TODO add block entity
    Sponge,
    Stairs(Stair),
    StickyPiston {
        facing: Surface4,
    }, // TODO consider "extended" field and StickyPistonHead.
    Stone,
    StoneBricks,
    StoneCutter {
        facing: Surface4,
    },
    StructureBlock, // TODO Add Corner, Data, Load, and Save variants. TODO add block entity
    StructureVoid,
    SugarCane {
        age: Int0Through15,
    },
    SweetBerryBush {
        age: Int0Through3,
    },
    Target,
    Terracotta {
        colour: Option<Colour>,
    },
    TNT {
        unstable: bool,
    },
    Torch {
        facing: Surface5,
    },
    Trapdoor {
        material: DoorMaterial,
        hinge_at: Edge8,
        open: bool,
        waterlogged: bool,
    },
    TrappedChest(Chest), // TODO add block entity
    Tripwire,
    TripwireHook {
        facing: Surface4,
    },
    TurtleEgg {
        count: Int1Through4,
        age: Int0Through2,
    },
    TwistingVines {
        age: Int0Through25,
    },
    TwistingVinesPlant,
    Vines, // NB should attach to all neighbouring blocks by default
    // TODO figure out if side bools are needed (e.g. hanging vines)
    Wall {
        material: WallMaterial,
        waterlogged: bool,
    },
    WarpedFungus,
    WarpedNyliym,
    WarpedRoots,
    WarpedWartBlock,
    WaterSource, // TODO handle magic (that is, the "flowing" state)
    WeepingVines {
        age: Int0Through25,
    },
    WeepingVinesPlant,
    WetSponge,
    Wheat {
        age: Int0Through7,
    },
    Wool {
        colour: Option<Colour>,
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
