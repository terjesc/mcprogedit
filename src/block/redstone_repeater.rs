use std::convert::TryFrom;

use crate::block::Block;
use crate::bounded_ints::Int1Through4;
use crate::positioning::{Direction, Surface4};

#[derive(Clone, Debug, PartialEq)]
pub struct RedstoneRepeater {
    pub(crate) facing: Surface4,
    pub(crate) delay: Int1Through4,
}

impl RedstoneRepeater {
    pub fn new() -> Self {
        Self {
            delay: Int1Through4::new_saturating(1),
            facing: Surface4::North,
        }
    }

    pub fn get_delay(&self) -> i8 {
        self.delay.get()
    }

    pub fn get_facing(&self) -> Direction {
        Direction::from(self.facing.clone())
    }

    pub fn has_delay_of(&self, delay: i8) -> bool {
        self.delay.get() == delay
    }

    pub fn has_facing_of(&self, facing: &Direction) -> bool {
        *facing == self.facing.clone().into()
    }

    pub fn set_delay(&mut self, delay: i8) {
        self.delay = Int1Through4::new_saturating(delay);
    }

    pub fn set_facing(&mut self, facing: &Direction) {
        self.facing = Surface4::try_from(facing.clone()).unwrap();
    }

    pub fn with_delay(&self, delay: i8) -> Self {
        let mut repeater = self.clone();
        repeater.set_delay(delay);
        repeater
    }

    pub fn with_facing(&self, facing: &Direction) -> Self {
        let mut repeater = self.clone();
        repeater.set_facing(facing);
        repeater
    }
}

impl Default for RedstoneRepeater {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Block> for RedstoneRepeater {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::RedstoneRepeater(repeater) => Ok(repeater),
            _ => Err(()),
        }
    }
}

impl From<RedstoneRepeater> for Block {
    fn from(repeater: RedstoneRepeater) -> Block {
        Block::RedstoneRepeater(repeater)
    }
}
