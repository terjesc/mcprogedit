use std::convert::TryFrom;

use crate::block::Block;
use crate::inventory::Inventory;
use crate::positioning::{Direction, Surface6};

#[derive(Clone, Debug, PartialEq)]
pub struct Dropper {
    pub facing: Surface6,
    pub custom_name: Option<String>,
    pub lock: Option<String>,
    pub items: Inventory,
}

impl Dropper {
    pub fn has_facing_of(&self, facing: Direction) -> bool {
        facing == self.facing.clone().into()
    }
}

impl TryFrom<Block> for Dropper {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::Dropper(dropper) => Ok(*dropper),
            _ => Err(()),
        }
    }
}
