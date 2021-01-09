use std::convert::TryFrom;

use crate::block::Block;
use crate::material::DoorMaterial;
use crate::positioning::{Direction, Surface4};

#[derive(Clone, Debug, PartialEq)]
pub struct Door {
    pub material: DoorMaterial,
    pub facing: Surface4,
    pub half: DoorHalf,
    pub hinge: Hinge,
    pub open: bool,
}

impl Door {
    pub fn has_facing_of(&self, facing: Direction) -> bool {
        facing == self.facing.clone().into()
    }
}

impl TryFrom<Block> for Door {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::Door(door) => Ok(door),
            _ => Err(()),
        }
    }
}

/// Doors are two blocks high. Which block is this?
#[derive(Clone, Debug, PartialEq)]
pub enum DoorHalf {
    /// Bottom block of the door
    Lower,
    /// Top block of the door.
    Upper,
}

/// For doors, what way they are hinged. Left/Right relative to the direction
/// the door is Facing. (E.g. if Facing North, Left means on the West side,
/// and Right means on the East side.)
#[derive(Clone, Debug, PartialEq)]
pub enum Hinge {
    Left,
    Right,
}
