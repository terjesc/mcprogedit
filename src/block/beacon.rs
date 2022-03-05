use std::convert::TryFrom;

use crate::block::Block;
use crate::block_entity::{BlockEntity, CommonTags};
use crate::status_effect::StatusEffect;

/// Beacon "block".
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Beacon {
    pub lock: Option<String>,
    pub levels: i32, // TODO change type to integer with valid range
    pub primary: Option<StatusEffect>,
    pub secondary: Option<StatusEffect>,
}

impl Beacon {
    pub(crate) fn to_block_entity(&self, at: (i32, i32, i32)) -> BlockEntity {
        let (x, y, z) = at;
        BlockEntity::Beacon {
            common: CommonTags {
                id: "minecraft:beacon".into(),
                x,
                y,
                z,
                keep_packed: false,
            },
            lock: self.lock.clone(),
            levels: self.levels,
            primary: self.primary,
            secondary: self.secondary,
        }
    }
}

impl TryFrom<Block> for Beacon {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::Beacon(beacon) => Ok(*beacon),
            _ => Err(()),
        }
    }
}
