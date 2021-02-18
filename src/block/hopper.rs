use std::convert::TryFrom;

use crate::block::Block;
use crate::block_entity::{BlockEntity, ChestTags, CommonTags};
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

    pub(crate) fn to_block_entity(&self, at: (i32, i32, i32)) -> BlockEntity {
        let (x, y, z) = at;
        BlockEntity::Hopper {
            tags: ChestTags {
                common: CommonTags {
                    id: "minecraft:hopper".into(),
                    x,
                    y,
                    z,
                    keep_packed: false,
                },
                custom_name: self.custom_name.clone(),
                lock: self.custom_name.clone(),
                items: self.items.clone(),
                loot_table: None,      // TODO
                loot_table_seed: None, // TODO
            }
        }
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
