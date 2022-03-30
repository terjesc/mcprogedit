use std::convert::TryFrom;

use crate::block::Block;
use crate::block_entity::{BlockEntity, FurnaceTags, CommonTags};
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

    pub(crate) fn to_block_entity(&self, id: &'static str, at: (i32, i32, i32)) -> BlockEntity {
        let (x, y, z) = at;
        BlockEntity::Furnace {
            tags: FurnaceTags {
                common: CommonTags {
                    id: id.into(),
                    x,
                    y,
                    z,
                    keep_packed: false,
                },
                custom_name: self.custom_name.clone(),
                lock: self.custom_name.clone(),
                items: self.items.clone(),
                burn_time: self.burn_time,
                cook_time: self.cook_time,
                cook_time_total: self.cook_time_total,
            }
        }
    }
}

impl TryFrom<Block> for Furnace {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::Furnace(furnace)
            | Block::BlastFurnace(furnace)
            | Block::Smoker(furnace) => Ok(*furnace),
            _ => Err(()),
        }
    }
}
