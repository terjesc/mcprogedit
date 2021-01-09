use std::convert::TryFrom;

use crate::block::Block;
use crate::colour::Colour;
use crate::inventory::Inventory;
use crate::positioning::{Direction, Surface6};

#[derive(Clone, Debug, PartialEq)]
pub struct ShulkerBox {
    pub colour: Option<Colour>,
    pub facing: Surface6,
    pub custom_name: Option<String>,
    pub lock: Option<String>,
    pub items: Inventory,
}

impl ShulkerBox {
    pub fn has_facing_of(&self, facing: Direction) -> bool {
        facing == self.facing.clone().into()
    }
}

impl TryFrom<Block> for ShulkerBox {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::ShulkerBox(shulker_box) => Ok(*shulker_box),
            _ => Err(()),
        }
    }
}
