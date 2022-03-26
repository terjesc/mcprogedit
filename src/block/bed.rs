use std::convert::TryFrom;
use std::fmt;

use crate::block::Block;
use crate::colour::Colour;
use crate::positioning::{Direction, Surface4};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
        facing == self.facing.into()
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum BedEnd {
    Foot,
    Head,
}

impl fmt::Display for BedEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            BedEnd::Foot => "foot",
            BedEnd::Head => "head",
        })
    }
}
