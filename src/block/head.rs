use std::convert::TryFrom;

use crate::block::Block;
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
