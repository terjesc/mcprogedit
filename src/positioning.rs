//! For describing positioning of blocks within their voxel (placement, rotation, etc.)

use std::convert::TryFrom;
use thiserror::Error;

// TODO Consider adding door placement data structure to this file...
// Door (8) - hinged at one of 4 corners, + pointing in one of 2 directions from that corner
//
// TODO Consider if appropriate here, as not really positioning... more configuration...
// SlabVariant (3) - up/down/double
// RailShape (10) - all the different rail configuration options
// ChorusPlantdirections = DirectionFlags6
// FireFace = DirectionFlags6

#[derive(Error, Debug)]
pub enum DirectionError {
    #[error("conversion from {0} failed")]
    TryFrom(Direction),
}

/// Positioning of bells.
///
/// Bells can be rotated in four directions. On top of that they can hang
/// form the block above, be mounted to one side, hang between two blocks
/// (one on either side), or be mounted on the block below.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

/// All directions.
///
/// Convertible to and from direction, edge and surface data types.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Down,
    DownEast,
    DownNorth,
    DownSouth,
    DownWest,
    East,
    EastNorthEast,
    EastSouthEast,
    North,
    NorthEast,
    NorthNorthEast,
    NorthNorthWest,
    NorthWest,
    South,
    SouthEast,
    SouthSouthEast,
    SouthSouthWest,
    SouthWest,
    Up,
    UpEast,
    UpNorth,
    UpSouth,
    UpWest,
    West,
    WestNorthWest,
    WestSouthWest,
}

impl Direction {
    /// Returns an instance of the opposite direction.
    pub fn opposite(&self) -> Self {
        match self {
            Self::Down => Self::Up,
            Self::DownEast => Self::UpWest,
            Self::DownNorth => Self::UpSouth,
            Self::DownSouth => Self::UpNorth,
            Self::DownWest => Self::UpEast,
            Self::East => Self::West,
            Self::EastNorthEast => Self::WestSouthWest,
            Self::EastSouthEast => Self::WestNorthWest,
            Self::North => Self::South,
            Self::NorthEast => Self::SouthWest,
            Self::NorthNorthEast => Self::SouthSouthWest,
            Self::NorthNorthWest => Self::SouthSouthEast,
            Self::NorthWest => Self::SouthEast,
            Self::South => Self::North,
            Self::SouthEast => Self::NorthWest,
            Self::SouthSouthEast => Self::NorthNorthWest,
            Self::SouthSouthWest => Self::NorthNorthEast,
            Self::SouthWest => Self::NorthEast,
            Self::Up => Self::Down,
            Self::UpEast => Self::DownWest,
            Self::UpNorth => Self::DownSouth,
            Self::UpSouth => Self::DownNorth,
            Self::UpWest => Self::DownEast,
            Self::West => Self::East,
            Self::WestNorthWest => Self::EastSouthEast,
            Self::WestSouthWest => Self::EastNorthEast,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self::North
    }
}

// TODO find and use a crate for deriving Display for simple enums,
// instead of this manual implementation.
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::Down => f.write_str("Down"),
            Self::DownEast => f.write_str("DownEast"),
            Self::DownNorth => f.write_str("DownNorth"),
            Self::DownSouth => f.write_str("DownSouth"),
            Self::DownWest => f.write_str("DownWest"),
            Self::East => f.write_str("East"),
            Self::EastNorthEast => f.write_str("EastNorthEast"),
            Self::EastSouthEast => f.write_str("EastSouthEast"),
            Self::North => f.write_str("North"),
            Self::NorthEast => f.write_str("NorthEast"),
            Self::NorthNorthEast => f.write_str("NorthNorthEast"),
            Self::NorthNorthWest => f.write_str("NorthNorthWest"),
            Self::NorthWest => f.write_str("NorthWest"),
            Self::South => f.write_str("South"),
            Self::SouthEast => f.write_str("SouthEast"),
            Self::SouthSouthEast => f.write_str("SouthSouthEast"),
            Self::SouthSouthWest => f.write_str("SouthSouthWest"),
            Self::SouthWest => f.write_str("SouthWest"),
            Self::Up => f.write_str("Up"),
            Self::UpEast => f.write_str("UpEast"),
            Self::UpNorth => f.write_str("UpNorth"),
            Self::UpSouth => f.write_str("UpSouth"),
            Self::UpWest => f.write_str("UpWest"),
            Self::West => f.write_str("West"),
            Self::WestNorthWest => f.write_str("WestNorthWest"),
            Self::WestSouthWest => f.write_str("WestSouthWest"),
        }
    }
}

impl From<Direction16> for Direction {
    fn from(item: Direction16) -> Self {
        match item {
            Direction16::South => Self::South,
            Direction16::SouthSouthWest => Self::SouthSouthWest,
            Direction16::SouthWest => Self::SouthWest,
            Direction16::WestSouthWest => Self::WestSouthWest,
            Direction16::West => Self::West,
            Direction16::WestNorthWest => Self::WestNorthWest,
            Direction16::NorthWest => Self::NorthWest,
            Direction16::NorthNorthWest => Self::NorthNorthWest,
            Direction16::North => Self::North,
            Direction16::NorthNorthEast => Self::NorthNorthEast,
            Direction16::NorthEast => Self::NorthEast,
            Direction16::EastNorthEast => Self::EastNorthEast,
            Direction16::East => Self::East,
            Direction16::EastSouthEast => Self::EastSouthEast,
            Direction16::SouthEast => Self::SouthEast,
            Direction16::SouthSouthEast => Self::SouthSouthEast,
        }
    }
}

impl From<Edge8> for Direction {
    fn from(item: Edge8) -> Self {
        match item {
            Edge8::DownEast => Self::DownEast,
            Edge8::DownWest => Self::DownWest,
            Edge8::DownSouth => Self::DownSouth,
            Edge8::DownNorth => Self::DownNorth,
            Edge8::UpEast => Self::UpEast,
            Edge8::UpWest => Self::UpWest,
            Edge8::UpSouth => Self::UpSouth,
            Edge8::UpNorth => Self::UpNorth,
        }
    }
}

impl From<Surface2> for Direction {
    fn from(item: Surface2) -> Self {
        match item {
            Surface2::Down => Self::Down,
            Surface2::Up => Self::Up,
        }
    }
}

impl From<Surface4> for Direction {
    fn from(item: Surface4) -> Self {
        match item {
            Surface4::East => Self::East,
            Surface4::North => Self::North,
            Surface4::South => Self::South,
            Surface4::West => Self::West,
        }
    }
}

impl From<Surface5> for Direction {
    fn from(item: Surface5) -> Self {
        match item {
            Surface5::Down => Self::Down,
            Surface5::East => Self::East,
            Surface5::North => Self::North,
            Surface5::South => Self::South,
            Surface5::West => Self::West,
        }
    }
}

impl From<Surface6> for Direction {
    fn from(item: Surface6) -> Self {
        match item {
            Surface6::Down => Self::Down,
            Surface6::East => Self::East,
            Surface6::North => Self::North,
            Surface6::South => Self::South,
            Surface6::Up => Self::Up,
            Surface6::West => Self::West,
        }
    }
}

impl From<WallOrRotatedOnFloor> for Direction {
    fn from(item: WallOrRotatedOnFloor) -> Self {
        match item {
            WallOrRotatedOnFloor::Floor(direction) => Self::from(direction),
            WallOrRotatedOnFloor::Wall(surface) => Self::from(surface),
        }
    }
}

impl From<SurfaceRotation12> for Direction {
    fn from(item: SurfaceRotation12) -> Self {
        match item {
            SurfaceRotation12::DownFacingEast => Self::DownEast,
            SurfaceRotation12::DownFacingNorth => Self::DownNorth,
            SurfaceRotation12::DownFacingSouth => Self::DownSouth,
            SurfaceRotation12::DownFacingWest => Self::DownWest,
            SurfaceRotation12::East => Self::East,
            SurfaceRotation12::North => Self::North,
            SurfaceRotation12::South => Self::South,
            SurfaceRotation12::West => Self::West,
            SurfaceRotation12::UpFacingEast => Self::UpEast,
            SurfaceRotation12::UpFacingNorth => Self::UpNorth,
            SurfaceRotation12::UpFacingSouth => Self::UpSouth,
            SurfaceRotation12::UpFacingWest => Self::UpWest,
        }
    }
}

/// Describes the rotation of blocks or entities that can be positioned in
/// 16 different directions, by what direction they are facing.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction16 {
    South = 0,
    SouthSouthWest = 1,
    SouthWest = 2,
    WestSouthWest = 3,
    West = 4,
    WestNorthWest = 5,
    NorthWest = 6,
    NorthNorthWest = 7,
    North = 8,
    NorthNorthEast = 9,
    NorthEast = 10,
    EastNorthEast = 11,
    East = 12,
    EastSouthEast = 13,
    SouthEast = 14,
    SouthSouthEast = 15,
}

impl Direction16 {
    /// Returns an instance of the opposite direction.
    pub fn opposite(&self) -> Self {
        match self {
            Self::East => Self::West,
            Self::EastNorthEast => Self::WestSouthWest,
            Self::EastSouthEast => Self::WestNorthWest,
            Self::North => Self::South,
            Self::NorthEast => Self::SouthWest,
            Self::NorthNorthEast => Self::SouthSouthWest,
            Self::NorthNorthWest => Self::SouthSouthEast,
            Self::NorthWest => Self::SouthEast,
            Self::South => Self::North,
            Self::SouthEast => Self::NorthWest,
            Self::SouthSouthEast => Self::NorthNorthWest,
            Self::SouthSouthWest => Self::NorthNorthEast,
            Self::SouthWest => Self::NorthEast,
            Self::West => Self::East,
            Self::WestNorthWest => Self::EastSouthEast,
            Self::WestSouthWest => Self::EastNorthEast,
        }
    }
}

impl Default for Direction16 {
    fn default() -> Self {
        Self::North
    }
}

impl From<i8> for Direction16 {
    fn from(direction_number: i8) -> Self {
        match direction_number {
            0 => Direction16::South,
            1 => Direction16::SouthSouthWest,
            2 => Direction16::SouthWest,
            3 => Direction16::WestSouthWest,
            4 => Direction16::West,
            5 => Direction16::WestNorthWest,
            6 => Direction16::NorthWest,
            7 => Direction16::NorthNorthWest,
            8 => Direction16::North,
            9 => Direction16::NorthNorthEast,
            10 => Direction16::NorthEast,
            11 => Direction16::EastNorthEast,
            12 => Direction16::East,
            13 => Direction16::EastSouthEast,
            14 => Direction16::SouthEast,
            15 => Direction16::SouthSouthEast,
            _ => panic!("Invalid direction number: {}", direction_number),
        }
    }
}

impl From<Direction16> for u8 {
    fn from(direction: Direction16) -> u8 {
        match direction {
            Direction16::South => 0,
            Direction16::SouthSouthWest => 1,
            Direction16::SouthWest => 2,
            Direction16::WestSouthWest => 3,
            Direction16::West => 4,
            Direction16::WestNorthWest => 5,
            Direction16::NorthWest => 6,
            Direction16::NorthNorthWest => 7,
            Direction16::North => 8,
            Direction16::NorthNorthEast => 9,
            Direction16::NorthEast => 10,
            Direction16::EastNorthEast => 11,
            Direction16::East => 12,
            Direction16::EastSouthEast => 13,
            Direction16::SouthEast => 14,
            Direction16::SouthSouthEast => 15,
        }
    }
}

impl TryFrom<Direction> for Direction16 {
    type Error = DirectionError;

    fn try_from(item: Direction) -> Result<Self, Self::Error> {
        match item {
            Direction::South => Ok(Self::South),
            Direction::SouthSouthWest => Ok(Self::SouthSouthWest),
            Direction::SouthWest => Ok(Self::SouthWest),
            Direction::WestSouthWest => Ok(Self::WestSouthWest),
            Direction::West => Ok(Self::West),
            Direction::WestNorthWest => Ok(Self::WestNorthWest),
            Direction::NorthWest => Ok(Self::NorthWest),
            Direction::NorthNorthWest => Ok(Self::NorthNorthWest),
            Direction::North => Ok(Self::North),
            Direction::NorthNorthEast => Ok(Self::NorthNorthEast),
            Direction::NorthEast => Ok(Self::NorthEast),
            Direction::EastNorthEast => Ok(Self::EastNorthEast),
            Direction::East => Ok(Self::East),
            Direction::EastSouthEast => Ok(Self::EastSouthEast),
            Direction::SouthEast => Ok(Self::SouthEast),
            Direction::SouthSouthEast => Ok(Self::SouthSouthEast),
            _ => Err(DirectionError::TryFrom(item)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DirectionFlags5 {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DirectionFlags6 {
    pub east: bool,
    pub down: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
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
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WallOrRotatedOnFloor {
    /// Block rests on top of the block below it, and may face 16 different directions.
    Floor(Direction16),
    /// The block is mounted on a side surface of the voxel containing it.
    Wall(Surface4),
}

impl WallOrRotatedOnFloor {
    pub fn is_on_floor(&self) -> bool {
        matches!(self, Self::Floor(_))
    }

    pub fn is_on_wall(&self) -> bool {
        !self.is_on_floor()
    }
}

impl Default for WallOrRotatedOnFloor {
    fn default() -> Self {
        Self::Floor(Direction16::default())
    }
}

/// Alignment along one of the 2 horizontal axes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

impl Default for Axis2 {
    fn default() -> Self {
        Self::Z
    }
}

/// Alignment along an axis.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

impl Default for Axis3 {
    fn default() -> Self {
        Self::Y
    }
}

/// The top and bottom surfaces of the voxel volume populated by the block.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Surface2 {
    Down,
    Up,
}

impl Default for Surface2 {
    fn default() -> Self {
        Self::Up
    }
}

impl TryFrom<Direction> for Surface2 {
    type Error = DirectionError;

    fn try_from(item: Direction) -> Result<Self, Self::Error> {
        match item {
            Direction::Down => Ok(Self::Down),
            Direction::Up => Ok(Self::Up),
            _ => Err(DirectionError::TryFrom(item)),
        }
    }
}

/// The four side surfaces of the voxel volume populated by the block.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Surface4 {
    East,
    North,
    South,
    West,
}

impl Default for Surface4 {
    fn default() -> Self {
        Self::North
    }
}

impl TryFrom<Direction> for Surface4 {
    type Error = DirectionError;

    fn try_from(item: Direction) -> Result<Self, Self::Error> {
        match item {
            Direction::East => Ok(Self::East),
            Direction::North => Ok(Self::North),
            Direction::South => Ok(Self::South),
            Direction::West => Ok(Self::West),
            _ => Err(DirectionError::TryFrom(item)),
        }
    }
}

/// The bottom and four side surfaces of the voxel volume populated by the block..
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Surface5 {
    Down,
    East,
    North,
    South,
    West,
}

impl Default for Surface5 {
    fn default() -> Self {
        Self::Down
    }
}

impl TryFrom<Direction> for Surface5 {
    type Error = DirectionError;

    fn try_from(item: Direction) -> Result<Self, Self::Error> {
        match item {
            Direction::Down => Ok(Self::Down),
            Direction::East => Ok(Self::East),
            Direction::North => Ok(Self::North),
            Direction::South => Ok(Self::South),
            Direction::West => Ok(Self::West),
            _ => Err(DirectionError::TryFrom(item)),
        }
    }
}

/// All six surfaces of the voxel volume populated by the block.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Surface6 {
    Down,
    East,
    North,
    South,
    Up,
    West,
}

impl Default for Surface6 {
    fn default() -> Self {
        Self::North
    }
}

impl TryFrom<Direction> for Surface6 {
    type Error = DirectionError;

    fn try_from(item: Direction) -> Result<Self, Self::Error> {
        match item {
            Direction::Down => Ok(Self::Down),
            Direction::East => Ok(Self::East),
            Direction::North => Ok(Self::North),
            Direction::South => Ok(Self::South),
            Direction::Up => Ok(Self::Up),
            Direction::West => Ok(Self::West),
            _ => Err(DirectionError::TryFrom(item)),
        }
    }
}

/// The four top-most and four bottom-most edges of the voxel volume populated by the block.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

impl Default for Edge8 {
    fn default() -> Self {
        Self::DownNorth
    }
}

impl From<i8> for Edge8 {
    fn from(edge_number: i8) -> Self {
        match edge_number {
            0 => Edge8::DownEast,
            1 => Edge8::DownWest,
            2 => Edge8::DownSouth,
            3 => Edge8::DownNorth,
            4 => Edge8::UpEast,
            5 => Edge8::UpWest,
            6 => Edge8::UpSouth,
            7 => Edge8::UpNorth,
            _ => panic!("Invalid edge number: {}", edge_number),
        }
    }
}

impl From<Edge8> for u8 {
    fn from(edge: Edge8) -> u8 {
        match edge {
            Edge8::DownEast => 0,
            Edge8::DownWest => 1,
            Edge8::DownSouth => 2,
            Edge8::DownNorth => 3,
            Edge8::UpEast => 4,
            Edge8::UpWest => 5,
            Edge8::UpSouth => 6,
            Edge8::UpNorth => 7,
        }
    }
}

impl TryFrom<Direction> for Edge8 {
    type Error = DirectionError;

    fn try_from(item: Direction) -> Result<Self, Self::Error> {
        match item {
            Direction::DownEast => Ok(Self::DownEast),
            Direction::DownWest => Ok(Self::DownWest),
            Direction::DownSouth => Ok(Self::DownSouth),
            Direction::DownNorth => Ok(Self::DownNorth),
            Direction::UpEast => Ok(Self::UpEast),
            Direction::UpWest => Ok(Self::UpWest),
            Direction::UpSouth => Ok(Self::UpSouth),
            Direction::UpNorth => Ok(Self::UpNorth),
            _ => Err(DirectionError::TryFrom(item)),
        }
    }
}

/// All six surfaces of the voxel volume populated by the block,
/// with rotation towards a cardinal direction for the Up and Down surfaces.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

impl Default for SurfaceRotation12 {
    fn default() -> Self {
        Self::North
    }
}

impl TryFrom<Direction> for SurfaceRotation12 {
    type Error = DirectionError;

    fn try_from(item: Direction) -> Result<Self, Self::Error> {
        match item {
            Direction::Down => Ok(Self::DownFacingNorth),
            Direction::DownEast => Ok(Self::DownFacingEast),
            Direction::DownWest => Ok(Self::DownFacingWest),
            Direction::DownSouth => Ok(Self::DownFacingSouth),
            Direction::DownNorth => Ok(Self::DownFacingNorth),
            Direction::East => Ok(Self::East),
            Direction::West => Ok(Self::West),
            Direction::South => Ok(Self::South),
            Direction::North => Ok(Self::North),
            Direction::Up => Ok(Self::UpFacingNorth),
            Direction::UpEast => Ok(Self::UpFacingEast),
            Direction::UpWest => Ok(Self::UpFacingWest),
            Direction::UpSouth => Ok(Self::UpFacingSouth),
            Direction::UpNorth => Ok(Self::UpFacingNorth),
            _ => Err(DirectionError::TryFrom(item)),
        }
    }
}

/// Valid orientations for a Jigsaw Block in Java Edition.
///
/// Please don't ask. The terminology is taken directly from the Minecraft save format.
/// I have no idea what it means.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
