use std::convert::TryFrom;

use crate::block::Block;
use crate::material::{DoorMaterial, Material};
use crate::positioning::{Direction, Edge8};

#[derive(Clone, Debug, PartialEq)]
pub struct Trapdoor {
    pub(crate) material: DoorMaterial,
    pub(crate) hinge_at: Edge8,
    pub(crate) open: bool,
    pub(crate) waterlogged: bool,
}

impl Trapdoor {
    pub fn has_hinge_at<T>(&self, hinge_position: T) -> bool
    where
        T: Copy + Into<Direction>,
    {
        Into::<Direction>::into(self.hinge_at) == Into::<Direction>::into(hinge_position)
    }

    pub fn has_material_of<T>(&self, material: T) -> bool
    where
        T: Copy + Into<Material>,
    {
        Into::<Material>::into(self.material) == Into::<Material>::into(material)
    }

    pub fn is_closed(&self) -> bool {
        !self.is_open()
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}

impl TryFrom<Block> for Trapdoor {
    type Error = ();

    fn try_from(block: Block) -> Result<Self, Self::Error> {
        match block {
            Block::Trapdoor(door) => Ok(door),
            _ => Err(()),
        }
    }
}
