use std::convert::TryFrom;

use crate::block::Block;
use crate::block_entity::{BlockEntity, CommonTags};
use crate::positioning::{Direction, WallOrRotatedOnFloor};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Head {
    pub(crate) variant: HeadVariant,
    pub(crate) placement: WallOrRotatedOnFloor,
    pub(crate) waterlogged: bool,
}

impl Head {
    pub fn has_facing_of(&self, facing: Direction) -> bool {
        facing == self.placement.into()
    }

    pub fn has_variant_of(&self, variant: HeadVariant) -> bool {
        self.variant == variant
    }

    pub fn is_on_floor(&self) -> bool {
        self.placement.is_on_floor()
    }

    pub fn is_on_wall(&self) -> bool {
        self.placement.is_on_wall()
    }

    pub(crate) fn to_block_entity(&self, at: (i32, i32, i32)) -> BlockEntity {
        let (x, y, z) = at;
        BlockEntity::Skull {
            common: CommonTags {
                id: "minecraft:skull".into(),
                x,
                y,
                z,
                keep_packed: false,
            },
            skull_type: None, // TODO fill for pre flattening
            facing: None,     // TODO fill for pre flattening
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HeadVariant {
    CreeperHead,
    DragonHead,
    PlayerHead,
    SkeletonSkull,
    WitherSkeletonSkull,
    ZombieHead,
}

impl Default for HeadVariant {
    fn default() -> Self {
        HeadVariant::SkeletonSkull
    }
}

impl TryFrom<Block> for Head {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::Head(head) => Ok(head),
            _ => Err(()),
        }
    }
}

impl From<Head> for Block {
    fn from(head: Head) -> Block {
        Block::Head(head)
    }
}
