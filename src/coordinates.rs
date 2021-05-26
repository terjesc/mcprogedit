#[derive(Clone, Copy, Debug, Hash, Ord, PartialEq, PartialOrd, Eq)]
pub struct BlockCoord(pub i64, pub i64, pub i64);
#[derive(Clone, Copy, Debug, Hash, Ord, PartialEq, PartialOrd, Eq)]
pub struct BlockColumnCoord(pub i64, pub i64);
#[derive(Clone, Copy, Debug, Hash, Ord, PartialEq, PartialOrd, Eq)]
pub struct ChunkCoord(pub i64, pub i64);
#[derive(Clone, Copy, Debug, Hash, Ord, PartialEq, PartialOrd, Eq)]
pub struct RegionCoord(pub i64, pub i64);

impl From<(i64, i64, i64)> for BlockCoord {
    fn from(coords: (i64, i64, i64)) -> Self {
        Self(coords.0, coords.1, coords.2)
    }
}

impl std::ops::Add for BlockCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::Sub for BlockCoord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl std::ops::Mul<i64> for BlockCoord {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        BlockCoord(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::Mul<BlockCoord> for i64 {
    type Output = BlockCoord;

    fn mul(self, rhs: BlockCoord) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<i64> for BlockCoord {
    type Output = Self;

    fn div(self, rhs: i64) -> Self::Output {
        BlockCoord(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl From<&ChunkCoord> for BlockCoord {
    fn from(chunk: &ChunkCoord) -> Self {
        Self(chunk.0 << 4, 0, chunk.1 << 4)
    }
}

impl From<(i64, i64)> for BlockColumnCoord {
    fn from(coords: (i64, i64)) -> Self {
        Self(coords.0, coords.1)
    }
}

impl std::ops::Add for BlockColumnCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub for BlockColumnCoord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl std::ops::Mul<i64> for BlockColumnCoord {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        BlockColumnCoord(self.0 * rhs, self.1 * rhs)
    }
}

impl std::ops::Mul<BlockColumnCoord> for i64 {
    type Output = BlockColumnCoord;

    fn mul(self, rhs: BlockColumnCoord) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<i64> for BlockColumnCoord {
    type Output = Self;

    fn div(self, rhs: i64) -> Self::Output {
        BlockColumnCoord(self.0 / rhs, self.1 / rhs)
    }
}

impl From<BlockCoord> for BlockColumnCoord {
    fn from(block: BlockCoord) -> Self {
        Self(block.0, block.2)
    }
}

impl From<ChunkCoord> for BlockColumnCoord {
    fn from(chunk: ChunkCoord) -> Self {
        Self(chunk.0 << 4, chunk.1 << 4)
    }
}

impl From<RegionCoord> for BlockColumnCoord {
    fn from(region: RegionCoord) -> Self {
        Self(region.0 << (4 + 5), region.1 << (4 + 5))
    }
}

impl From<(i64, i64)> for ChunkCoord {
    fn from(coords: (i64, i64)) -> Self {
        Self(coords.0, coords.1)
    }
}

impl std::ops::Add for ChunkCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub for ChunkCoord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl ChunkCoord {
    pub fn relative_to_region(&self, region: &RegionCoord) -> Self {
        let relative_chunk_0_0: Self = Self::from(*region);
        *self - relative_chunk_0_0
    }
}

impl From<BlockColumnCoord> for ChunkCoord {
    fn from(block: BlockColumnCoord) -> Self {
        Self(block.0 >> 4, block.1 >> 4)
    }
}

impl From<RegionCoord> for ChunkCoord {
    fn from(region: RegionCoord) -> Self {
        Self(region.0 << 5, region.1 << 5)
    }
}

impl From<(i64, i64)> for RegionCoord {
    fn from(coords: (i64, i64)) -> Self {
        Self(coords.0, coords.1)
    }
}

impl std::ops::Add for RegionCoord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub for RegionCoord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl From<ChunkCoord> for RegionCoord {
    fn from(chunk: ChunkCoord) -> Self {
        RegionCoord(chunk.0 >> 5, chunk.1 >> 5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_to_region_coord_conversion() {
        assert_eq!(RegionCoord(0, -1), RegionCoord::from(ChunkCoord(30, -3)));
        assert_eq!(RegionCoord(2, -1), RegionCoord::from(ChunkCoord(70, -30)));
    }

    #[test]
    fn test_block_column_to_chunk_coord_conversion() {
        assert_eq!(
            ChunkCoord(1, -1),
            ChunkCoord::from(BlockColumnCoord(27, -15))
        );
        assert_eq!(
            ChunkCoord(-8, -2),
            ChunkCoord::from(BlockColumnCoord(-115, -30))
        );
    }
}
