use crate::chunk::Chunk;
use crate::coordinates::BlockCoord;

impl Chunk {
    /// Calculates the global block coordinates of the block at index `index`
    /// of the "Blocks" and similar NBT tags, within section `section_y_index`
    /// of the chunk whose local (0, 0, 0) coordinates are at global block
    /// coordinates `chunk_offset`.
    pub(in crate::chunk) fn coordinates(
        section_y_index: i64,
        chunk_offset: BlockCoord,
        index: usize,
    ) -> BlockCoord {
        // index = (y * X_LENGTH * Z_LENGTH) + (z * X_LENGTH) + x
        const X_LENGTH: i64 = 16;
        const Y_HEIGHT: i64 = 16;
        const Z_LENGTH: i64 = 16;
        let y_offset = section_y_index * Y_HEIGHT;
        let y = y_offset + (index as i64) / (X_LENGTH * Z_LENGTH);
        let z = ((index as i64) % (X_LENGTH * Z_LENGTH)) / X_LENGTH;
        let x = (index as i64) % X_LENGTH;
        //println!("Looking for block entity at ({}, {}, {})", x, y, z);
        let local_coordinates: BlockCoord = (x, y, z).into();
        local_coordinates + chunk_offset
    }

    /// Calculates the index into the "Blocks" and similar NBT tags, for a block
    /// within section `section_y_index`, located at chunk local coordinates
    /// `local_block_coords`.
    pub(in crate::chunk) fn local_index(
        section_y_index: i64,
        local_block_coords: BlockCoord,
    ) -> usize {
        const X_LENGTH: i64 = 16;
        const Y_HEIGHT: i64 = 16;
        const Z_LENGTH: i64 = 16;
        let y_offset = section_y_index * Y_HEIGHT;
        let (x, y, z) = (
            local_block_coords.0,
            local_block_coords.1,
            local_block_coords.2,
        );
        ((y - y_offset) * X_LENGTH * Z_LENGTH + z * X_LENGTH + x) as usize
    }

    /// Calculates the index into the "Blocks" and similar NBT tags, for a block
    /// within section `section_y_index` of the chunk whose local (0, 0, 0)
    /// coordinates are at global block coordinates `chunk_offset`.
    pub(in crate::chunk) fn _global_index(
        section_y_index: i64,
        chunk_offset: BlockCoord,
        global_block_coordinates: BlockCoord,
    ) -> usize {
        let local_coords = global_block_coordinates - chunk_offset;
        Self::local_index(section_y_index, local_coords)
    }
}
