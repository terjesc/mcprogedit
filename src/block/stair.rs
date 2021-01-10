use std::convert::TryFrom;

use crate::block::Block;
use crate::material::{Material, StairMaterial};
use crate::positioning::{Direction, Edge8};

/// Stair shape is not configurable, as it depend on neighbouring stairs.
/// Stair shape is either automatically calculated on save, or the block is
/// flagged for update so that it will be automatically corrected in-game.
#[derive(Clone, Debug, PartialEq)]
pub struct Stair {
    pub material: StairMaterial,
    pub position: Edge8,
    pub waterlogged: bool,
}

impl Stair {
    pub fn has_facing_of(&self, facing: Direction) -> bool {
        facing == self.position.clone().into()
    }

    pub fn has_material_of(&self, material: Material) -> bool {
        material == self.material.clone().into()
    }
}

impl TryFrom<Block> for Stair {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::Stairs(stair) => Ok(stair),
            _ => Err(()),
        }
    }
}
