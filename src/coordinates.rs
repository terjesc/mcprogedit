#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockCoord(pub i64, pub i64, pub i64);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockColumnCoord(pub i64, pub i64);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChunkCoord(pub i64, pub i64);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegionCoord(pub i64, pub i64);

impl From<BlockCoord> for BlockColumnCoord {
    fn from(block: BlockCoord) -> Self {
        BlockColumnCoord(block.0, block.2)
    }
}

impl From<BlockColumnCoord> for ChunkCoord {
    fn from(block: BlockColumnCoord) -> Self {
        ChunkCoord(block.0 >> 4, block.1 >> 4)
    }
}

impl From<ChunkCoord> for RegionCoord {
    fn from(chunk: ChunkCoord) -> Self {
        RegionCoord(chunk.0 >> 5, chunk.1 >> 5)
    }
}

impl From<(i64, i64, i64)> for BlockCoord {
    fn from(coords: (i64, i64, i64)) -> Self {
        BlockCoord(coords.0, coords.1, coords.2)
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
