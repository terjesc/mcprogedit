use std::convert::TryFrom;

use crate::block::Block;
use crate::positioning::{Direction, DirectionFlags6};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChorusPlant {
    // NB should attach to all neighbouring blocks by default
    pub(crate) connections: DirectionFlags6,
}

impl ChorusPlant {
    /// Returns whether or not the ChorusPlant is connected in the given direction.
    pub fn is_touching_surface<T>(&self, direction: T) -> bool
    where
        T: Copy + Into<Direction>,
    {
        match Into::<Direction>::into(direction) {
            Direction::East => self.connections.east,
            Direction::Down => self.connections.down,
            Direction::North => self.connections.north,
            Direction::South => self.connections.south,
            Direction::Up => self.connections.up,
            Direction::West => self.connections.west,
            _ => false,
        }
    }
}

impl TryFrom<Block> for ChorusPlant {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::ChorusPlant(chorus_plant) => Ok(chorus_plant),
            _ => Err(()),
        }
    }
}

impl From<ChorusPlant> for Block {
    fn from(chorus_plant: ChorusPlant) -> Block {
        Block::ChorusPlant(chorus_plant)
    }
}
