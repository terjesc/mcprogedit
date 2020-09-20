#[derive(Clone, PartialEq)]
pub enum Colour {
    Black,
    Red,
    Green,
    Brown,
    Blue,
    Purple,
    Cyan,
    LightGray,
    Gray,
    Pink,
    Lime,
    Yellow,
    LightBlue,
    Magenta,
    Orange,
    White,
}

pub type Color = Colour;
use crate::material::*;

/// Where in its 1x1x1 m space a block is positioned. E.g. a Lever
/// with Ceiling Face is mounted on the bottom side of the block
/// above it, and a Button with Floor Face is mounted on top of the
/// block below it.
#[derive(Clone, PartialEq)]
pub enum Face {
    Ceiling,
    Floor,
    Wall,
}

/// What direction a block is facing. For blocks mounted on walls,
/// the Facing is in the opposite direction of the block on which
/// they are mounted. E.g. a Button Facing East is mounted on the
/// block to its West. Using this terminology, Doors are "mounted"
/// on the block that they touch with their full width when closed.
/// Stairs are facing towards the full-block side. Trapdoors are
/// hinged on the mounted side. For beds, what end the pillow is in.
#[derive(Clone, PartialEq)]
pub enum Facing {
    East,
    North,
    South,
    West,
}

/// What direction a block is facing, 5 possible directions.
/// For Coral Fans, this is the direction they are growing out of the,
/// neighbouring block, e.g. Facing5 Up means it grows on the floor.
/// For Hopper, Up means output is down, otherwise output is towards
/// the direction indicated.
#[derive(Clone, PartialEq)]
pub enum Facing5 {
    East,
    North,
    South,
    Up,
    West,
}

/// What direction a block is facing, 6 possible directions.
#[derive(Clone, PartialEq)]
pub enum Facing6 {
    Down,
    East,
    North,
    South,
    Up,
    West,
}

/// Doors are two blocks high. Which block is this?
#[derive(Clone, PartialEq)]
pub enum DoorHalf {
    /// Bottom block of the door
    Lower,
    /// Top block of the door.
    Upper,
}

/// For doors, what way they are hinged. Left/Right relative to the direction
/// the door is Facing. (E.g. if Facing North, Left means on the West side,
/// and Right means on the East side.)
#[derive(Clone, PartialEq)]
pub enum Hinge {
    Left,
    Right,
}

bounded_integer! {
    /// For Leaves, how far they are from the trunk.
    #[repr(i8)]
    pub struct DistanceToTrunk { 0..=7 }
}

/// For Logs, pillars, etc. What axis they are aligned with.
#[derive(Clone, PartialEq)]
pub enum Axis {
    /// East-West orientation
    X,
    /// Vertical (Up-Down) orientation
    Y,
    /// North-South orientation
    Z,
}

#[allow(non_upper_case_globals)]
impl Axis {
    /// Helper alias for axis orientation
    pub const East: Axis = Axis::X;
    pub const West: Axis = Axis::X;
    pub const Up: Axis = Axis::Y;
    pub const Down: Axis = Axis::Y;
    pub const South: Axis = Axis::Z;
    pub const North: Axis = Axis::Z;
}

/// For Nether Portal blocks.
#[derive(Clone, PartialEq)]
pub enum Axis2D {
    /// East-West orientation
    X,
    /// North-South orientation
    Z,
}

#[allow(non_upper_case_globals)]
impl Axis2D {
    /// Helper alias for axis orientation
    pub const East: Axis2D = Axis2D::X;
    pub const West: Axis2D = Axis2D::X;
    pub const South: Axis2D = Axis2D::Z;
    pub const North: Axis2D = Axis2D::Z;
}

bounded_integer! {
    #[repr(i8)]
    pub struct Age2 { 0..=1 }
}

bounded_integer! {
    #[repr(i8)]
    pub struct Age3 { 0..=2 }
}

bounded_integer! {
    #[repr(i8)]
    pub struct Age4 { 0..=3 }
}

bounded_integer! {
    #[repr(i8)]
    pub struct Age6 { 0..=5 }
}

bounded_integer! {
    /// For melon and pumpkin stems.
    #[repr(i8)]
    pub struct Age8 { 0..=7 }
}

bounded_integer! {
    #[repr(i8)]
    pub struct Age16 { 0..=15 }
}

bounded_integer! {
    #[repr(i8)]
    pub struct Age26 { 0..=25 }
}

bounded_integer! {
    /// For composter.
    #[repr(i8)]
    pub struct Level9 { 0..=8 }
}

bounded_integer! {
    /// For framland. Fully hydrated (wet texture) at 7.
    #[repr(i8)]
    pub struct Wetness { 0..=7 }
}

bounded_integer! {
    /// Number of bites taken from a cake.
    #[repr(i8)]
    pub struct CakeBites { 0..=6 }
}

bounded_integer! {
    /// Facing starts at 0 for South, then goes clockwise by 22.5 degrees,
    /// e.g. 2 is southwest, 4 is West, 15 is south-southeast.
    #[repr(i8)]
    pub struct Facing16 { 0..=15 }
}

/// Despite the name, actually is used for describing both Sign, Banner, and Head.
#[derive(Clone, PartialEq)]
pub enum WallFloorFacing {
    Floor(Facing16),
    Wall(Facing),
}

#[derive(Clone, PartialEq)]
pub struct Sign {
    material: WoodMaterial,
    facing: WallFloorFacing,
    waterlogged: bool,
    colour: Colour,
    text1: String,
    text2: String,
    text3: String,
    text4: String,
}

#[derive(Clone, PartialEq)]
pub enum SlabPosition {
    Bottom,
    Double,
    Top,
}

#[derive(Clone, PartialEq)]
pub struct Slab {
    material: SlabMaterial,
    position: SlabPosition,
    waterlogged: bool,
}

#[derive(Clone, PartialEq)]
pub enum VerticalPosition {
    /// For stairs, full block surface at the bottom.
    /// For trapdoors, trapdoor sits at the bottom when closed.
    Bottom,
    /// For stairs, full block surface at the top.
    /// For trapdoors, trapdoor sits at the top when closed.
    Top,
}

/// Stair shape is not configurable, as it depend on neighbouring stairs.
/// Stair shape is either automatically calculated on save, or the block is
/// flagged for update so that it will be automatically corrected in-game.
#[derive(Clone, PartialEq)]
pub struct Stair {
    pub material: StairMaterial,
    pub position: VerticalPosition,
    pub facing: Facing,
    pub waterlogged: bool,
}

#[derive(Clone, PartialEq)]
pub enum RailType {
    Activator,
    Detector,
    Normal,
    Powered,
}

#[derive(Clone, PartialEq)]
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

#[derive(Clone, PartialEq)]
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

#[derive(Clone, PartialEq)]
pub enum Grass {
    Fern,
    Grass,
    LargeFernBottom,
    LargeFernTop,
    TallGrassBottom,
    TallGrassTop,
}

#[derive(Clone, PartialEq)]
pub enum Seagrass {
    Seagrass,
    TallSeagrassBottom,
    TallSeagrassTop,
}

#[derive(Clone, PartialEq)]
pub enum AnvilDamage {
    Intact,
    SlightlyDamaged,
    VeryDamaged,
}

#[derive(Clone, PartialEq)]
pub enum StemState {
    Growing(Age8),
    Attached(Facing),
}

#[derive(Clone, PartialEq)]
pub enum StemMaterial {
    Melon,
    Pumpkin,
}

#[derive(Clone, PartialEq)]
pub enum BambooLeaves {
    None,
    Small,
    Large,
}

#[derive(Clone, PartialEq)]
pub struct Log {
    pub material: WoodMaterial,
    /// Logs with no alignment have bark (or stripped pattern) on all sides.
    pub alignment: Option<Axis>,
    pub stripped: bool,
}

bounded_integer! {
    #[repr(i8)]
    pub struct HoneyLevel { 0..=5 }
}

#[derive(Clone, PartialEq)]
pub enum BellMounting {
    Ceiling,
    DoubleWall,
    Floor,
    SingleWall,
}

#[derive(Clone, PartialEq)]
pub enum BedEnd {
    Foot,
    Head,
}

// TODO consider using BitSet here
#[derive(Clone, PartialEq)]
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

#[derive(Clone, PartialEq)]
pub enum BubbleDirection {
    Upward,
    Downward,
}

bounded_integer! {
    #[repr(i8)]
    pub struct WaterLevel { 0..=3 }
}

#[derive(Clone, PartialEq)]
pub enum CommandBlockVariant {
    ChainedCommandBlock,
    CommandBlock,
    RepeatingCommandBlock,
}

#[derive(Clone, PartialEq)]
pub struct CommandBlock {
    pub variant: CommandBlockVariant,
    pub conditional: bool,
    pub facing: Facing6,
}

#[derive(Clone, PartialEq)]
pub enum ChestVariant {
    Left,
    Right,
    Single,
}

#[derive(Clone, PartialEq)]
pub struct Chest {
    pub facing: Facing,
    pub variant: ChestVariant,
    pub waterlogged: bool,
}

#[derive(Clone, PartialEq)]
pub enum HeadVariant {
    CreeperHead,
    DragonHead,
    PlayerHead,
    SkeletonSkull,
    WitherSkeletonSkull,
    ZombieHead,
}

#[derive(Clone, PartialEq)]
pub struct Head {
    pub variant: HeadVariant,
    pub facing: WallFloorFacing,
    pub waterlogged: bool,
}

#[derive(Clone, PartialEq)]
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

/// I have no idea what all of this means... But supposedly it is the valid
/// orientations for a Jigsaw Block in Java Edition.
#[derive(Clone, PartialEq)]
pub enum JigsawBlockOrientation {
    DownEast,
    DownNorth,
    DownSouth,
    DownWest,
    EastUp,
    NorthUp,
    SouthUp,
    UpEast,
    UpNorth,
    UpSouth,
    UpWest,
    WestUp,
}

#[derive(Clone, PartialEq)]
pub enum OnOffState {
    On,
    Off,
}

#[derive(Clone, PartialEq)]
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

bounded_integer! {
    #[repr(i8)]
    pub struct PitchValue { 0..=24 }
}

impl Pitch {
    pub fn from_value(value: PitchValue) -> Self {
        match value {
            PitchValue(0) => Pitch::Fs0,
            PitchValue(1) => Pitch::G0,
            PitchValue(2) => Pitch::Gs0,
            PitchValue(3) => Pitch::A0,
            PitchValue(4) => Pitch::As0,
            PitchValue(5) => Pitch::B0,
            PitchValue(6) => Pitch::C1,
            PitchValue(7) => Pitch::Cs1,
            PitchValue(8) => Pitch::D1,
            PitchValue(9) => Pitch::Ds1,
            PitchValue(10) => Pitch::E1,
            PitchValue(11) => Pitch::F1,
            PitchValue(12) => Pitch::Fs1,
            PitchValue(13) => Pitch::G1,
            PitchValue(14) => Pitch::Gs1,
            PitchValue(15) => Pitch::A1,
            PitchValue(16) => Pitch::As1,
            PitchValue(17) => Pitch::B1,
            PitchValue(18) => Pitch::C2,
            PitchValue(19) => Pitch::Cs2,
            PitchValue(20) => Pitch::D2,
            PitchValue(21) => Pitch::Ds2,
            PitchValue(22) => Pitch::E2,
            PitchValue(23) => Pitch::F2,
            PitchValue(24) => Pitch::Fs2,
            _ => panic!("PitchValue out of range!"),
        }
    }
}

bounded_integer! {
    /// RedstoneRepeater delay in number of "redstone" ticks.
    #[repr(i8)]
    pub struct DelaySetting { 1..=4 }
}

bounded_integer! {
    /// Number of charges for a RespawnAnchor.
    #[repr(i8)]
    pub struct RespawnAnchorCharges { 0..=4 }
}

bounded_integer! {
    /// Number of SeaPickles in the block.
    #[repr(i8)]
    pub struct PickleCount { 1..=4 }
}

bounded_integer! {
    /// Number of TurtleEggs in the block.
    #[repr(i8)]
    pub struct TurtleEggCount { 1..=4 }
}

bounded_integer! {
    /// Number of snow layers (snow depth)
    #[repr(i8)]
    pub struct SnowLayerCount { 1..=8 }
}

#[derive(Clone, PartialEq)]
pub enum Block {
    Air,
    AncientDebris,
    Anvil {
        facing: Facing,
        damage: AnvilDamage,
    },
    Andesite,
    Bamboo {
        age: Age2,
        leaves: BambooLeaves,
        stage: Age2,
    },
    Banner {
        colour: Colour,
        placement: WallFloorFacing,
    }, // TODO add block entity
    Barrel {
        facing: Facing6,
    }, // TODO add block entity
    Barrier,
    Basalt {
        alignment: Axis,
    },
    Beacon, // TODO add block entity
    Bedrock,
    Beetroots {
        growth_stage: Age4,
    },
    Beehive {
        facing: Facing,
        honey_level: HoneyLevel,
    }, // TODO add block entity
    BeeNest {
        facing: Facing,
        honey_level: HoneyLevel,
    }, // TODO add block entity
    Bell {
        mounting: BellMounting,
        facing: Facing,
    }, // TODO add block entity
    Bed {
        colour: Colour,
        facing: Facing,
        end: BedEnd,
    },
    Blackstone,
    BlastFurnace {
        facing: Facing,
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
        alignment: Axis,
    },
    Bookshelf,
    BrewingStand, // TODO add block entity
    BrownMushroom,
    BrownMushroomBlock {
        cap_directions: DirectionFlags6,
    },
    BubbleColumn {
        drag_direction: BubbleDirection,
    }, // Is this even needed?
    Button(ButtonMaterial, Face, Facing),
    Cactus {
        age: Age16,
    },
    Cake {
        bites: CakeBites,
    },
    Campfire {
        facing: Facing,
        lit: bool,
        waterlogged: bool,
    },
    Carpet {
        colour: Colour,
    },
    Carrots {
        growth_stage: Age8,
    },
    CartographyTable,
    CarvedPumpkin {
        facing: Facing,
    },
    Cauldron {
        water_level: WaterLevel,
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
        age: Age6,
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
        age: Age3,
        facing: Facing,
    },
    CommandBlock(CommandBlock), // TODO add block entity
    Composter {
        fullness: Level9,
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
        facing: Facing5,
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
        facing: Facing6,
    }, // TODO add block entity
    Door {
        material: DoorMaterial,
        facing: Facing,
        half: DoorHalf,
        hinge: Hinge,
        open: bool,
    },
    DragonEgg,
    DriedKelpBlock,
    Dropper {
        facing: Facing6,
    }, // TODO add block entity
    EmeraldOre,
    EnchantingTable, // TODO add block entity
    EndGateway,      // TODO add block entity
    EndPortal,       // TODO add block entity
    EndPortalFrame {
        facing: Facing,
        has_eye: bool,
    },
    Endrod {
        facing: Facing,
    },
    EndStone,
    EndStoneBricks,
    EnderChest {
        facing: Facing,
        waterlogged: bool,
    }, // TODO add block entity (?)
    Farmland {
        wetness: Wetness,
    },
    Fence {
        material: FenceMaterial,
        waterlogged: bool,
    },
    FenceGate {
        material: WoodMaterial,
        facing: Facing,
        open: bool,
    },
    Fire {
        age: Age16,
        burning_faces: FireFace,
    },
    FletchingTable,
    Flower(Flower),
    FlowerPot {
        plant: Option<PottedPlant>,
    },
    FrostedIce,
    Furnace {
        facing: Facing,
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
        facing: Facing,
    },
    Glowstone,
    GoldOre,
    Granite,
    /// Grass, Fern, and two block high variants.
    Grass(Grass),
    GrassBlock,
    GrassPath,
    Gravel,
    GrindStone(Face, Facing),
    HayBale {
        alignment: Axis,
    },
    Head(Head), // TODO add block entity (used only for PlayerHead variant)
    HoneyBlock,
    HoneycombBlock,
    Hopper {
        facing: Facing5,
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
        facing: Facing,
    },
    JigsawBlock {
        orientation: JigsawBlockOrientation,
    }, // TODO add block entity
    Jukebox, // TODO add block entity (and potentially "has record" bool)
    Kelp {
        age: Age26,
    },
    Ladder {
        facing: Facing,
        waterlogged: bool,
    },
    Lantern {
        hanging: bool,
    },
    LapisLazuliBlock,
    LapisLazuliOre,
    LavaSource, // TODO handle magic (that is, the "flowing" state)
    Leaves {
        material: LeavesMaterial,
        distance_to_trunk: DistanceToTrunk,
        persistent: bool,
    },
    Lectern {
        facing: Facing,
    }, // TODO add block entity (and possibly "has book" bool)
    Lever(Face, Facing, OnOffState),
    LilyPad,
    LodeStone,
    Log(Log),
    Loom {
        facing: Facing,
    },
    MagmaBlock,
    Melon,
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
        alignment: Axis2D,
    },
    NetherQuartzOre,
    NetherSprouts,
    NetherWart {
        age: Age4,
    },
    NetherWartBlock,
    Netherrack,
    Noteblock {
        pitch: Pitch,
    }, // TODO instrument depend on neighbouring block below.
    // Figure out if an "instrument" field is needed.
    Observer {
        facing: Facing,
    }, // TODO consider if a "powered" field is useful.
    Obsidian,
    PackedIce,
    Piston {
        facing: Facing,
    }, // TODO consider adding "extended" field and PistonHead block.
    Planks {
        material: WoodMaterial,
    },
    Podzol,
    PolishedAndesite,
    PolishedBasalt {
        alignment: Axis,
    },
    PolishedBlackstone,
    PolishedBlackstoneBricks,
    PolishedDiorite,
    PolishedGranite,
    Potatoes {
        growth_stage: Age8,
    },
    PressurePlate {
        material: PressurePlateMaterial,
    },
    Prismarine,
    PrismarineBricks,
    Pumpkin,
    PurpurBlock,
    PurpurPillar {
        alignment: Axis,
    },
    QuartzBricks,
    QuartsPillar {
        alignment: Axis,
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
        facing: Facing,
    }, // TODO add block entity (?)
    RedstoneSubtractor {
        facing: Facing,
    }, // TODO add block entity (?)
    RedstoneLamp,
    RedstoneOre,
    RedstoneRepeater {
        facing: Facing,
        delay: DelaySetting,
    },
    RedstoneTorch {
        facing: Facing5,
    },
    RedstoneWire, // TODO upcoming change: * or + shape, of non-connected wire
    RespawnAnchor {
        charges: RespawnAnchorCharges,
    },
    Sand,
    Sandstone,
    Sapling {
        material: SaplingMaterial,
        stage: Age2,
    },
    Scaffolding {
        waterlogged: bool,
    },
    SeaLantern,
    SeaPickle {
        count: PickleCount,
        waterlogged: bool,
    },
    Seagrass {
        variant: Seagrass,
    },
    Shroomlight,
    ShulkerBox {
        colour: Option<Colour>,
        facing: Facing6,
    }, // TODO add block entity
    Sign(Box<Sign>),
    Slab(Slab),
    SlimeBlock,
    SmithingTable,
    Smoker {
        facing: Facing,
        lit: bool,
    }, // TODO add block entity
    SmoothQuartz,
    SmoothRedSandstone,
    SmoothSandstone,
    SmoothStone,
    Snow {
        thickness: SnowLayerCount,
    },
    SnowBlock,
    SoulCampfire {
        facing: Facing,
        lit: bool,
        waterlogged: bool,
    },
    SoulFire {
        age: Age16,
        burning_faces: FireFace,
    },
    SoulLantern {
        hanging: bool,
    },
    SoulTorch {
        facing: Facing5,
    },
    SoulSand,
    SoulSoil,
    Spawner, // TODO add block entity
    Sponge,
    Stair(Stair),
    Stem {
        material: StemMaterial,
        state: StemState,
    },
    StickyPiston {
        facing: Facing,
    }, // TODO consider "extended" field and StickyPistonHead.
    Stone,
    StoneBricks,
    StoneCutter {
        facing: Facing,
    },
    StructureBlock, // TODO Add Corner, Data, Load, and Save variants. TODO add block entity
    StructureVoid,
    SugarCane {
        age: Age16,
    },
    SweetBerryBush {
        age: Age4,
    },
    Target,
    Terracotta {
        colour: Option<Colour>,
    },
    TNT {
        unstable: bool,
    },
    Torch {
        facing: Facing5,
    },
    Trapdoor {
        material: DoorMaterial,
        position: VerticalPosition,
        facing: Facing,
        open: bool,
        waterlogged: bool,
    },
    TrappedChest(Chest), // TODO add block entity
    Tripwire,
    TripwireHook {
        facing: Facing,
    },
    TurtleEgg {
        count: TurtleEggCount,
        age: Age3,
    },
    TwistingVines {
        age: Age26,
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
        age: Age26,
    },
    WeepingVinesPlant,
    WetSponge,
    Wheat {
        age: Age8,
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
