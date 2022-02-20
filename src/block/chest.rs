use std::convert::TryFrom;

use crate::block::Block;
use crate::block_entity::{BlockEntity, ChestTags, CommonTags};
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
        facing == self.facing.into()
    }

    pub(crate) fn to_block_entity(&self, at: (i32, i32, i32)) -> BlockEntity {
        let (x, y, z) = at;
        BlockEntity::Chest {
            tags: ChestTags {
                common: CommonTags {
                    id: "minecraft:chest".into(),
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
