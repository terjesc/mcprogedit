use std::convert::TryFrom;
use std::fmt;

use crate::block::Block;
use crate::material::{DoorMaterial, Material};
use crate::positioning::{Direction, Surface4};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Door {
    pub material: DoorMaterial,
    pub facing: Surface4,
    pub half: DoorHalf,
    pub hinged_at: Hinge,
    pub open: bool,
}

impl Door {
    pub fn close(&mut self) {
        self.open = false;
    }

    pub fn has_facing_of<T>(&self, facing: T) -> bool
    where
        T: Copy + Into<Direction>,
    {
        Into::<Direction>::into(self.facing) == Into::<Direction>::into(facing)
    }

    pub fn has_material_of(&self, material: &Material) -> bool {
        *material == self.material.into()
    }

    pub fn is_bottom_half(&self) -> bool {
        self.half == DoorHalf::Lower
    }

    pub fn is_closed(&self) -> bool {
        !self.is_open()
    }

    pub fn is_hinged_at(&self, hinge_side: &Hinge) -> bool {
        *hinge_side == self.hinged_at
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn is_top_half(&self) -> bool {
        self.half == DoorHalf::Upper
    }

    pub fn open(&mut self) {
        self.open = true;
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
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum DoorHalf {
    /// Bottom block of the door
    Lower,
    /// Top block of the door.
    Upper,
}

impl fmt::Display for DoorHalf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            DoorHalf::Lower => "lower",
            DoorHalf::Upper => "upper",
        })
    }
}

/// For doors, what way they are hinged. Left/Right relative to the direction
/// the door is Facing. (E.g. if Facing North, Left means on the West side,
/// and Right means on the East side.)
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Hinge {
    Left,
    Right,
}

impl fmt::Display for Hinge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Hinge::Left => "left",
            Hinge::Right => "right",
        })
    }
}
