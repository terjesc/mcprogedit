use crate::coordinates::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoundingBox {
    pub x_min: i64,
    pub x_max: i64,
    pub y_min: i64,
    pub y_max: i64,
    pub z_min: i64,
    pub z_max: i64,
}

impl BoundingBox {
    pub fn from_block_coords(p1: BlockCoord, p2: BlockCoord) -> Self {
        Self {
            x_min: i64::min(p1.0, p2.0),
            x_max: i64::max(p1.0, p2.0),
            y_min: i64::min(p1.1, p2.1),
            y_max: i64::max(p1.1, p2.1),
            z_min: i64::min(p1.2, p2.2),
            z_max: i64::max(p1.2, p2.2),
        }
    }

    pub fn from_coords(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> Self {
        Self::from_block_coords(p1.into(), p2.into())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChunkBounds {
    pub x_min: i64,
    pub x_max: i64,
    pub z_min: i64,
    pub z_max: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegionBounds {
    pub x_min: i64,
    pub x_max: i64,
    pub z_min: i64,
    pub z_max: i64,
}
