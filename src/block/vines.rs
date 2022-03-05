use std::convert::TryFrom;

use crate::block::Block;
use crate::positioning::{Direction, DirectionFlags5};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Vines {
    // NB Older versions attaches to all neighbouring blocks by default.
    // NB Newer versions allow arbitrary placement up and on any side surface, in any combination.
    pub(crate) anchored_at: DirectionFlags5,
}

impl Vines {
    /// Returns whether or not the Vines are covering the neighbouring block in the given direction.
    pub fn is_touching_surface<T>(&self, direction: T) -> bool
    where
        T: Copy + Into<Direction>,
    {
        match Into::<Direction>::into(direction) {
            Direction::East => self.anchored_at.east,
            Direction::North => self.anchored_at.north,
            Direction::South => self.anchored_at.south,
            Direction::Up => self.anchored_at.up,
            Direction::West => self.anchored_at.west,
            _ => false,
        }
    }
}

impl TryFrom<Block> for Vines {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::Vines(vines) => Ok(vines),
            _ => Err(()),
        }
    }
}

impl From<Vines> for Block {
    fn from(vines: Vines) -> Block {
        Block::Vines(vines)
    }
}
