use std::convert::TryFrom;

use crate::block::Block;
use crate::block_entity::{BlockEntity, CommonTags};
use crate::bounded_ints::*;
use crate::positioning::{Direction, Surface4};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Beehive {
    pub facing: Surface4,
    pub honey_level: Int0Through6,
    //pub bees: Vec<Bee>,
    //pub flower_position
}

impl Beehive {
    pub fn has_facing_of(&self, facing: Direction) -> bool {
        facing == self.facing.into()
    }

    pub(crate) fn to_block_entity(&self, at: (i32, i32, i32)) -> BlockEntity {
        let (x, y, z) = at;
        BlockEntity::Beehive {
            common: CommonTags {
                id: "minecraft:beehive".into(),
                x,
                y,
                z,
                keep_packed: false,
            },
            //bees: None, // TODO implement bees
            //flower_position: None, // TODO implement flower position
        }
    }
}

impl TryFrom<Block> for Beehive {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::Beehive(beehive) | Block::BeeNest(beehive) => Ok(*beehive),
            _ => Err(()),
        }
    }
}
