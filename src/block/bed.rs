use std::convert::TryFrom;

use crate::block::Block;
use crate::colour::Colour;
use crate::positioning::{Direction, Surface4};

#[derive(Clone, Debug, PartialEq)]
pub struct Bed {
    pub colour: Colour,
    pub facing: Surface4,
    pub end: BedEnd,
}

impl Bed {
    pub fn has_colour_of(&self, colour: Colour) -> bool {
        colour == self.colour
    }

    pub fn has_facing_of(&self, facing: Direction) -> bool {
        facing == self.facing.clone().into()
    }
}

impl TryFrom<Block> for Bed {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::Bed(bed) => Ok(bed),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BedEnd {
    Foot,
    Head,
}
