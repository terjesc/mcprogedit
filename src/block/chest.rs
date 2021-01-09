use std::convert::TryFrom;

use crate::block::Block;
use crate::inventory::Inventory;
use crate::positioning::{Direction, Surface4};

#[derive(Clone, Debug, PartialEq)]
pub struct Chest {
    pub facing: Surface4,
    pub variant: Option<ChestVariant>,
    pub waterlogged: bool,
    pub custom_name: Option<String>,
    pub lock: Option<String>,
    pub items: Inventory,
}

impl Chest {
    pub fn has_facing_of(&self, facing: Direction) -> bool {
        facing == self.facing.clone().into()
    }
}

impl TryFrom<Block> for Chest {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::Chest(chest) => Ok(*chest),
            Block::TrappedChest(chest) => Ok(*chest),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ChestVariant {
    Left,
    Right,
    Single,
}
