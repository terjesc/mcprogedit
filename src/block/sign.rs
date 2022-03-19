use std::convert::TryFrom;

use crate::block::Block;
use crate::block_entity::{BlockEntity, CommonTags};
use crate::colour::Colour;
use crate::material::{Material, WoodMaterial};
use crate::positioning::{Direction, WallOrRotatedOnFloor};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
    pub fn has_colour_of(&self, colour: Colour) -> bool {
        colour == self.colour
    }

    pub fn has_material_of(&self, material: Material) -> bool {
        material == self.material.into()
    }

    pub fn has_facing_of(&self, facing: Direction) -> bool {
        facing == self.placement.into()
    }

    pub(crate) fn to_block_entity(&self, at: (i32, i32, i32)) -> BlockEntity {
        let (x, y, z) = at;
        BlockEntity::Sign {
            common: CommonTags {
                id: "minecraft:sign".into(),
                x,
                y,
                z,
                keep_packed: false,
            },
            colour: self.colour,
            text: vec![self.text1.clone(), self.text2.clone(), self.text3.clone(), self.text4.clone()],
        }
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
