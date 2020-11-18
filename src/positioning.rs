//! For describing positioning of blocks within their voxel (placement, rotation, etc.)

// TODO Consider adding door placement data structure to this file...
// Door (8) - hinged at one of 4 corners, + pointing in one of 2 directions from that corner
//
// TODO Consider if appropriate here, as not really positioning... more configuration...
// SlabVariant (3) - up/down/double
// RailShape (10) - all the different rail configuration options
// DirectionFlags6 (2^6 = 64) - each cube surface can be in one of two states
// ChorusPlantdirections = DirectionFlags6
// FireFace = DirectionFlags6

/// Positioning of bells.
///
/// Bells can be rotated in four directions. On top of that they can hang
/// form the block above, be mounted to one side, hang between two blocks
/// (one on either side), or be mounted on the block below.
#[derive(Clone, PartialEq)]
pub enum BellPosition {
    UpEast,
    UpNorth,
    UpSouth,
    UpWest,
    SideEast,
    SideNorth,
    SideSouth,
    SideWest,
    DoubleSideEast,
    DoubleSideNorth,
    DoubleSideSouth,
    DoubleSideWest,
    DownEast,
    DownNorth,
    DownSouth,
    DownWest,
}

/// Describes the rotation of blocks or entities that can be positioned in
/// 16 different directions, by what direction they are facing.
#[derive(Clone, PartialEq)]
pub enum Direction16 {
    South = 0,
    SouthSouthWest = 1,
    SouthWest = 2,
    WestSouthWest = 3,
    West = 4,
    WestNorthWest = 5,
    NorthWest = 6,
    NorthNorhtWest = 7,
    North = 8,
    NorthNorthEast = 9,
    NorthEast = 10,
    EastNorthEast = 11,
    East = 12,
    EastSouthEast = 13,
    SouthEast = 14,
    SouthSouthEast = 15,
}

/// Position and rotation for blocks that can either be put on top of the block below,
/// or attached to the side of a horizontally adjacent block.
///
/// Used for e.g. signs and banners.
///
/// # Examples
/// ```
/// use mcprogedit::positioning::{WallOrRotatedOnFloor, Surface4, Direction16};
///
/// // WallOrRotatedOnFloor for a block lying on the floor,
/// // facing in the west-northwest direction.
/// let placement = WallOrRotatedOnFloor::Floor(Direction16::WestNorthWest);
///
/// // A block attached to its neighbouring block to the south (i.e. facing north).
/// let placement = WallOrRotatedOnFloor::Wall(Surface4::South);
/// ```
#[derive(Clone, PartialEq)]
pub enum WallOrRotatedOnFloor {
    /// Block rests on top of the block below it, and may face 16 different directions.
    Floor(Direction16),
    /// The block is mounted on a side surface of the voxel containing it.
    Wall(Surface4),
}

/// Alignment along one of the 2 horizontal axes.
#[derive(Clone, PartialEq)]
pub enum Axis2 {
    /// East-West orientation
    X,
    /// North-South orientation
    Z,
}

/// Helper aliases for axis orientation
#[allow(non_upper_case_globals)]
impl Axis2 {
    pub const East: Axis2 = Axis2::X;
    pub const West: Axis2 = Axis2::X;
    pub const South: Axis2 = Axis2::Z;
    pub const North: Axis2 = Axis2::Z;
}

/// Alignment along an axis.
#[derive(Clone, PartialEq)]
pub enum Axis3 {
    /// East-West orientation
    X,
    /// Vertical (Up-Down) orientation
    Y,
    /// North-South orientation
    Z,
}

/// Helper aliases for axis orientation
#[allow(non_upper_case_globals)]
impl Axis3 {
    pub const East: Axis3 = Axis3::X;
    pub const West: Axis3 = Axis3::X;
    pub const Up: Axis3 = Axis3::Y;
    pub const Down: Axis3 = Axis3::Y;
    pub const South: Axis3 = Axis3::Z;
    pub const North: Axis3 = Axis3::Z;
}

/// The top and bottom surfaces of the voxel volume populated by the block.
#[derive(Clone, PartialEq)]
pub enum Surface2 {
    Down,
    Up,
}

/// The four side surfaces of the voxel volume populated by the block.
#[derive(Clone, PartialEq)]
pub enum Surface4 {
    East,
    North,
    South,
    West,
}

/// The bottom and four side surfaces of the voxel volume populated by the block..
#[derive(Clone, PartialEq)]
pub enum Surface5 {
    Down,
    East,
    North,
    South,
    West,
}

/// All six surfaces of the voxel volume populated by the block.
#[derive(Clone, PartialEq)]
pub enum Surface6 {
    Down,
    East,
    North,
    South,
    Up,
    West,
}

/// The four top-most and four bottom-most edges of the voxel volume populated by the block.
#[derive(Clone, PartialEq)]
pub enum Edge8 {
    DownEast,
    DownNorth,
    DownSouth,
    DownWest,
    UpEast,
    UpNorth,
    UpSouth,
    UpWest,
}

/// All six surfaces of the voxel volume populated by the block,
/// with rotation towards a cardinal direction for the Up and Down surfaces.
#[derive(Clone, PartialEq)]
pub enum SurfaceRotation12 {
    DownFacingEast,
    DownFacingNorth,
    DownFacingSouth,
    DownFacingWest,
    East,
    North,
    South,
    West,
    UpFacingEast,
    UpFacingNorth,
    UpFacingSouth,
    UpFacingWest,
}

/// Valid orientations for a Jigsaw Block in Java Edition.
///
/// Please don't ask. The terminology is taken directly from the Minecraft save format.
/// I have no idea what it means.
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