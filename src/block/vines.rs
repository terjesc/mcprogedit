use std::convert::TryFrom;

use crate::block::Block;
use crate::positioning::{Direction, DirectionFlags6};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vines {
    // NB should attach to all neighbouring blocks by default
    pub(crate) anchored_at: DirectionFlags6,
}

impl Vines {
    /// Returns whether or not the Vines are covering the neighbouring block in the given direction.
    pub fn is_touching_surface<T>(&self, direction: T) -> bool
    where
        T: Copy + Into<Direction>,
    {
        match Into::<Direction>::into(direction) {
            Direction::East => self.anchored_at.east,
            Direction::Down => self.anchored_at.down,
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
