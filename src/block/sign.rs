use std::convert::TryFrom;

use crate::block::Block;
use crate::colour::Colour;
use crate::material::WoodMaterial;
use crate::positioning::{Direction, WallOrRotatedOnFloor};

#[derive(Clone, Debug, PartialEq)]
pub struct Sign {
    pub material: WoodMaterial,
    pub placement: WallOrRotatedOnFloor,
    pub waterlogged: bool,
    pub colour: Colour,
    pub text1: String,
    pub text2: String,
    pub text3: String,
    pub text4: String,
}

impl Sign {
    pub fn has_facing_of(&self, facing: Direction) -> bool {
        facing == self.placement.clone().into()
    }
}

impl TryFrom<Block> for Sign {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::Sign(sign) => Ok(*sign),
            _ => Err(()),
        }
    }
}
