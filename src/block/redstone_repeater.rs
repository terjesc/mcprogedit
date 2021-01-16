use std::convert::TryFrom;

use crate::block::Block;
use crate::bounded_ints::Int1Through4;
use crate::positioning::{Direction, DirectionError, Surface4};

/// Represents the state of a redstone repeater block.
///
/// # Example usage
/// ```
/// # use mcprogedit::positioning::{Direction, Surface4, Surface6};
/// # use mcprogedit::block::{Block, RedstoneRepeater};
/// // Instantiate a repeater:
/// let mut repeater = RedstoneRepeater::new();
///
/// // Set delay and facing:
/// repeater.set_delay(2);
/// repeater.set_facing(Surface4::East);
///
/// // Read delay and facing:
/// let delay = repeater.delay();
/// let facing = repeater.facing();
/// assert_eq!(delay, 2);
/// assert_eq!(facing, Surface4::East);
///
/// // Check delay:
/// assert!(repeater.has_delay_of(2));
///
/// // Check facing, using any type of direction:
/// assert!(repeater.has_facing_of(Surface4::East));
/// assert!(repeater.has_facing_of(Surface6::East));
/// assert!(repeater.has_facing_of(Direction::Up) == false);
///
/// // Turn the repeater into a proper block:
/// let repeater_block: Block = repeater.into();
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RedstoneRepeater {
    pub(crate) facing: Surface4,
    pub(crate) delay: Int1Through4,
}

impl RedstoneRepeater {
    /// Creates a new RedstoneRepeater.
    pub fn new() -> Self {
        Self {
            delay: Int1Through4::new_saturating(Self::DELAY_DEFAULT),
            facing: Surface4::default(),
        }
    }

    /// Sets the delay of the Redstone Repeater to the given value, clamped to the valid range.
    ///
    /// # Examples
    /// ```
    /// # use mcprogedit::block::RedstoneRepeater;
    /// let mut repeater = RedstoneRepeater::new();
    ///
    /// // Set the delay:
    /// repeater.set_delay(3);
    /// assert_eq!(repeater.delay(), 3);
    ///
    /// // The delay gets clamped if out of bounds:
    /// repeater.set_delay(i8::MIN);
    /// assert_eq!(repeater.delay(), RedstoneRepeater::DELAY_MIN);
    /// repeater.set_delay(i8::MAX);
    /// assert_eq!(repeater.delay(), RedstoneRepeater::DELAY_MAX);
    /// ```
    pub fn set_delay(&mut self, delay: i8) {
        self.delay = Int1Through4::new_saturating(delay);
    }

    /// Returns the delay setting of the RedstoneRepeater.
    ///
    /// # Example
    /// ```
    /// # use mcprogedit::block::RedstoneRepeater;
    /// let repeater = RedstoneRepeater::default();
    ///
    /// let delay = repeater.delay();
    /// assert_eq!(delay, RedstoneRepeater::DELAY_DEFAULT);
    /// ```
    pub fn delay(&self) -> i8 {
        self.delay.get()
    }

    /// Orients the RedstoneRepeater to face in the given direction.
    ///
    /// # Example
    /// ```
    /// # use mcprogedit::positioning::Surface4;
    /// # use mcprogedit::block::RedstoneRepeater;
    /// let mut repeater = RedstoneRepeater::new();
    ///
    /// // Set the facing:
    /// repeater.set_facing(Surface4::South);
    /// assert_eq!(repeater.facing(), Surface4::South);
    /// ```
    pub fn set_facing(&mut self, direction: Surface4) {
        self.facing = direction;
    }

    /// Tries to orient the RedstoneRepeater to face in the given direction.
    ///
    /// # Examples
    /// ```
    /// # use mcprogedit::positioning::{Direction, Surface4, Surface6};
    /// # use mcprogedit::block::RedstoneRepeater;
    /// let mut repeater = RedstoneRepeater::new();
    ///
    /// // Returns the new setting upon success:
    /// let result = repeater.try_set_facing(Surface4::South);
    /// assert_eq!(result.unwrap(), Surface4::South);
    /// assert_eq!(repeater.facing(), Surface4::South);
    ///
    /// // Can be called with any direction type:
    /// let result = repeater.try_set_facing(Surface6::East);
    /// assert_eq!(result.unwrap(), Surface4::East);
    /// assert_eq!(repeater.facing(), Surface4::East);
    ///
    /// // Repeaters cannot point upwards, so this call returns an error:
    /// let result = repeater.try_set_facing(Direction::Up);
    /// assert!(result.is_err());
    ///
    /// // Facing is still the same after the failing try_set:
    /// assert_eq!(repeater.facing(), Surface4::East);
    /// ```
    pub fn try_set_facing<T>(&mut self, direction: T) -> Result<Surface4, DirectionError>
    where
        T: Copy + Into<Direction>,
    {
        let direction = Into::<Direction>::into(direction);
        match Surface4::try_from(direction) {
            Ok(dir) => {
                self.facing = dir;
                Ok(dir)
            }
            Err(e) => Err(e),
        }
    }

    /// Returns what direction the RedstoneRepeater is facing.
    ///
    /// # Example
    /// ```
    /// # use mcprogedit::positioning::Surface4;
    /// # use mcprogedit::block::RedstoneRepeater;
    /// let mut repeater = RedstoneRepeater::new();
    /// repeater.set_facing(Surface4::West);
    ///
    /// let facing = repeater.facing();
    /// assert_eq!(facing, Surface4::West);
    /// ```
    pub fn facing(&self) -> Surface4 {
        self.facing
    }

    /// Returns whether or not the RedstoneRepeater is set to the given delay.
    ///
    /// # Examples
    /// ```
    /// # use mcprogedit::block::RedstoneRepeater;
    /// let mut repeater = RedstoneRepeater::new();
    /// assert!(repeater.has_delay_of(RedstoneRepeater::DELAY_DEFAULT));
    ///
    /// repeater.set_delay(3);
    /// assert!(repeater.has_delay_of(3));
    /// assert!(repeater.has_delay_of(2) == false);
    /// ```
    pub fn has_delay_of(&self, delay: i8) -> bool {
        self.delay.get() == delay
    }

    /// Returns whether or not the RedstoneRepeater is facing in the given direction.
    ///
    /// # Examples
    /// ```
    /// # use mcprogedit::positioning::{Surface4, Surface6, Direction};
    /// # use mcprogedit::block::RedstoneRepeater;
    /// let mut repeater = RedstoneRepeater::new();
    /// repeater.set_facing(Surface4::West);
    ///
    /// // Check the facing using the "proper" type.
    /// assert!(repeater.has_facing_of(Surface4::West));
    /// assert!(repeater.has_facing_of(Surface4::South) == false);
    ///
    /// // Check the facing against other direction types.
    /// assert!(repeater.has_facing_of(Surface6::West));
    /// assert!(repeater.has_facing_of(Direction::Up) == false);
    /// ```
    pub fn has_facing_of<T>(&self, facing: T) -> bool
    where
        T: Copy + Into<Direction>,
    {
        Into::<Direction>::into(self.facing) == Into::<Direction>::into(facing)
    }

    /// The default delay value.
    pub const DELAY_DEFAULT: i8 = Self::DELAY_MIN;

    /// The lowest possible delay value.
    pub const DELAY_MIN: i8 = 1;

    /// The highest possible delay value.
    pub const DELAY_MAX: i8 = 4;
}

impl Default for RedstoneRepeater {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Block> for RedstoneRepeater {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::RedstoneRepeater(repeater) => Ok(repeater),
            _ => Err(()),
        }
    }
}

impl From<RedstoneRepeater> for Block {
    fn from(repeater: RedstoneRepeater) -> Block {
        Block::RedstoneRepeater(repeater)
    }
}
