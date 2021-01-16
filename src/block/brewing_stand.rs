use std::convert::TryFrom;

use crate::block::Block;
use crate::inventory::Inventory;

/// Represents the state of a brewing stand block.
#[derive(Clone, Debug, PartialEq)]
pub struct BrewingStand {
    pub custom_name: Option<String>,
    pub lock: Option<String>,
    pub items: Inventory,
    pub brew_time: i16, // TODO change to integer with valid range
    pub fuel: i8,       // TODO change to integer with valid range
}

impl TryFrom<Block> for BrewingStand {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::BrewingStand(repeater) => Ok(*repeater),
            _ => Err(()),
        }
    }
}

impl From<BrewingStand> for Block {
    fn from(brewing_stand: BrewingStand) -> Block {
        Block::BrewingStand(Box::new(brewing_stand))
    }
}
