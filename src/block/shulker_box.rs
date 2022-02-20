use std::convert::TryFrom;

use crate::block::Block;
use crate::block_entity::{BlockEntity, ChestTags, CommonTags};
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
    pub fn has_colour_of(&self, colour: Colour) -> bool {
        Some(colour) == self.colour
    }

    pub fn has_facing_of(&self, facing: Direction) -> bool {
        facing == self.facing.into()
    }

    pub(crate) fn to_block_entity(&self, at: (i32, i32, i32)) -> BlockEntity {
        let (x, y, z) = at;
        BlockEntity::ShulkerBox {
            tags: ChestTags {
                common: CommonTags {
                    id: "minecraft:shulker_box".into(),
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

impl TryFrom<Block> for ShulkerBox {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::ShulkerBox(shulker_box) => Ok(*shulker_box),
            _ => Err(()),
        }
    }
}
