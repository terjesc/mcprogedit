use std::convert::TryFrom;

use crate::block::Block;
use crate::inventory::Inventory;
use crate::positioning::{Direction, Surface4};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Furnace {
    pub facing: Surface4,
    pub lit: bool,
    pub custom_name: Option<String>,
    pub lock: Option<String>,
    pub items: Inventory,
    pub burn_time: i16,
    pub cook_time: i16,
    pub cook_time_total: i16,
}

impl Furnace {
    pub fn has_facing_of(&self, facing: Direction) -> bool {
        facing == self.facing.into()
    }
}

impl TryFrom<Block> for Furnace {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::Furnace(furnace) => Ok(*furnace),
            _ => Err(()),
        }
    }
}
