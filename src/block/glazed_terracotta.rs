use std::convert::TryFrom;

use crate::block::Block;
use crate::colour::Colour;
use crate::positioning::{Direction, Surface4};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct GlazedTerracotta {
    pub(crate) colour: Colour,
    pub(crate) facing: Surface4,
}

impl GlazedTerracotta {
    pub fn has_colour_of(&self, colour: Colour) -> bool {
        self.colour == colour
    }

    pub fn has_facing_of(&self, facing: Direction) -> bool {
        facing == self.facing.into()
    }
}

impl TryFrom<Block> for GlazedTerracotta {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::GlazedTerracotta(glazed_terracotta) => Ok(glazed_terracotta),
            _ => Err(()),
        }
    }
}
