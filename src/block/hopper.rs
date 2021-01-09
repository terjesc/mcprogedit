use std::convert::TryFrom;

use crate::block::Block;
use crate::inventory::Inventory;
use crate::positioning::{Direction, Surface5};

#[derive(Clone, Debug, PartialEq)]
pub struct Hopper {
    pub facing: Surface5,
    pub waterlogged: bool,
    pub custom_name: Option<String>,
    pub lock: Option<String>,
    pub items: Inventory,
}

impl Hopper {
    pub fn has_facing_of(&self, facing: Direction) -> bool {
        facing == self.facing.clone().into()
    }
}

impl TryFrom<Block> for Hopper {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::Hopper(hopper) => Ok(*hopper),
            _ => Err(()),
        }
    }
}
