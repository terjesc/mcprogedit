//! A piece of a Minecraft world.

use crate::block::Block;
use crate::block_cuboid::BlockCuboid;
use crate::chunk::{Chunk, RawChunkData};
use crate::coordinates::*;
use crate::nbt_lookup::*;
use crate::region::Region;

extern crate nbt;

/// Structure for holding blocks and entities, representing a piece of a Minecraft world.
#[derive(Debug)]
pub struct WorldExcerpt {
    blocks: BlockCuboid,
}

impl WorldExcerpt {
    /// Creates a new empty `WorldExcerpt` of the given size.
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        WorldExcerpt {
            blocks: BlockCuboid::new((x, y, z)),
        }
    }

    pub fn dim(&self) -> (usize, usize, usize) {
        self.blocks.dim()
    }

    /// Creates a new `WorldExcerpt` from part of a saved Minecraft world.
    ///
    /// Imports from the world saved at `world_directory` the blocks and entities
    /// from within the bounding box defined by `p1` and `p2`.
    pub fn from_save(p1: BlockCoord, p2: BlockCoord, world_directory: &std::path::Path) -> Self {
        // Check that the world directory exists.
        if !world_directory.is_dir() {
            panic!("Not a world save directory: {:?}", world_directory);
        }

        // Find and check that level.dat exists.
        let level_dat_file = world_directory.join("level.dat");
        if !level_dat_file.is_file() {
            panic!("Not a valid world.dat file: {:?}", level_dat_file);
        }

        // TODO candidate for refactoring: Read level.dat into LevelDat struct.
        let mut level_dat = std::fs::File::open(level_dat_file).expect("Unable to open level.dat");
        let level_dat_blob = nbt::Blob::from_gzip_reader(&mut level_dat)
            .expect("Unable to parse level.dat contents");

        let _data_version = nbt_blob_lookup_int(&level_dat_blob, "Data/DataVersion")
            .unwrap_or_else(|| panic!("level.dat Data/DataVersion not found"));

        // Create an empty (None-filled) WorldExcerpt of the correct size.
        let mut world_excerpt = Self::new(
            1 + i64::abs(p1.0 - p2.0) as usize,
            1 + i64::abs(p1.1 - p2.1) as usize,
            1 + i64::abs(p1.2 - p2.2) as usize,
        );

        // TODO candidates for refactoring: All this bounds stuff.
        // Define the bounds in a more useful way than two points.
        struct Bounds {
            x: (i64, i64),
            y: (i64, i64),
            z: (i64, i64),
        }

        // Point conversions, to chunk and region coordinates
        let column_p1: BlockColumnCoord = p1.into();
        let chunk_p1: ChunkCoord = column_p1.into();
        let region_p1: RegionCoord = chunk_p1.into();
        let column_p2: BlockColumnCoord = p2.into();
        let chunk_p2: ChunkCoord = column_p2.into();
        let region_p2: RegionCoord = chunk_p2.into();

        // Inclusive block bounds, using global coordinates
        let global_block_bounds = Bounds {
            x: (i64::min(p1.0, p2.0), i64::max(p1.0, p2.0)),
            y: (i64::min(p1.1, p2.1), i64::max(p1.1, p2.1)),
            z: (i64::min(p1.2, p2.2), i64::max(p1.2, p2.2)),
        };

        // Inclusive chunk bounds, using global coordinates
        let global_chunk_bounds = Bounds {
            x: (
                i64::min(chunk_p1.0, chunk_p2.0),
                i64::max(chunk_p1.0, chunk_p2.0),
            ),
            y: (i64::min(p1.1, p2.1), i64::max(p1.1, p2.1)),
            z: (
                i64::min(chunk_p1.1, chunk_p2.1),
                i64::max(chunk_p1.1, chunk_p2.1),
            ),
        };

        // Inclusive region bounds, using global coordinates,
        let region_bounds = Bounds {
            x: (
                i64::min(region_p1.0, region_p2.0),
                i64::max(region_p1.0, region_p2.0),
            ),
            y: (i64::min(p1.1, p2.1), i64::max(p1.1, p2.1)),
            z: (
                i64::min(region_p1.1, region_p2.1),
                i64::max(region_p1.1, region_p2.1),
            ),
        };

        // Iterate through the (existing) region files within the bound
        for region_x in region_bounds.x.0..=region_bounds.x.1 {
            for region_z in region_bounds.z.0..=region_bounds.z.1 {
                println!("Importing region {}, {}", region_x, region_z);

                // Check if there actually is a region file for the given region
                let region_file_name = format!("r.{}.{}.mca", region_x, region_z);
                let region_file = world_directory.join("region/").join(&region_file_name);

                if !region_file.is_file() {
                    println!(
                        "Region file {} does not exist, continuing.",
                        &region_file_name
                    );
                }

                let region = Region::load_from_file(&region_file);

                // Figure out what chunks overlaps with the bounding box,
                // expressed in chunk coordinates relative to the region.
                let region_coords: RegionCoord = (region_x, region_z).into();
                let chunk_offset: ChunkCoord = region_coords.into();
                let in_region_chunk_bounds = Bounds {
                    x: (
                        i64::max(global_chunk_bounds.x.0 - chunk_offset.0, 0),
                        i64::min(global_chunk_bounds.x.1 - chunk_offset.0, 31),
                    ),
                    y: (global_chunk_bounds.y.0, global_chunk_bounds.y.1),
                    z: (
                        i64::max(global_chunk_bounds.z.0 - chunk_offset.1, 0),
                        i64::min(global_chunk_bounds.z.1 - chunk_offset.1, 31),
                    ),
                };

                // Handle those chunks
                for chunk_x in in_region_chunk_bounds.x.0..=in_region_chunk_bounds.x.1 {
                    for chunk_z in in_region_chunk_bounds.z.0..=in_region_chunk_bounds.z.1 {
                        //println!("Handling (region internal) chunk {}, {}", chunk_x, chunk_z);

                        // Parse the raw chunk data into a chunk object
                        let chunk_data = region.chunk_data(&(chunk_x, chunk_z).into());
                        let chunk = Chunk::from_raw_chunk_data(&chunk_data);

                        // Paste the blocks from the chunk
                        let chunk_offset: BlockCoord = chunk.chunk_coordinates().into();
                        let chunk_offset_in_blocks = (
                            chunk_offset.0 - global_block_bounds.x.0,
                            chunk_offset.1 - global_block_bounds.y.0,
                            chunk_offset.2 - global_block_bounds.z.0,
                        );
                        world_excerpt
                            .blocks
                            .paste(chunk_offset_in_blocks, chunk.blocks());

                        // TODO Move or copy the entities from the chunk
                    }
                }
            }
        }

        // Return the constructed WorldExcerpt
        world_excerpt
    }

    /// Writes the contents of the WorldExcerpt to a Minecraft world save.
    ///
    /// Pastes the contents of the world excerpt into a world saved at `world_directory`,
    /// positioned so that the excerpt corner with the lowest integer coordinates are
    /// put at world block coordinates `p`.
    pub fn to_save(&self, p: BlockCoord, world_directory: &std::path::Path) {
        // Check that the world directory exists.
        if !world_directory.is_dir() {
            panic!("Not a world save directory: {:?}", world_directory);
        }

        // Find and check that level.dat exists.
        let level_dat_file = world_directory.join("level.dat");
        if !level_dat_file.is_file() {
            panic!("Not a valid world.dat file: {:?}", level_dat_file);
        }

        // TODO candidate for refactoring: Read level.dat into LevelDat struct.
        let mut level_dat = std::fs::File::open(level_dat_file).expect("Unable to open level.dat");
        let level_dat_blob = nbt::Blob::from_gzip_reader(&mut level_dat)
            .expect("Unable to parse level.dat contents");

        let _data_version = nbt_blob_lookup_int(&level_dat_blob, "Data/DataVersion")
            .unwrap_or_else(|| panic!("level.dat Data/DataVersion not found"));

        let (dx, dy, dz) = self.dim();
        let (dx, dy, dz) = (dx as i64, dy as i64, dz as i64);
        let p1 = p;
        let p2: BlockCoord = (p1.0 + dx - 1, p1.1 + dy - 1, p1.2 + dz - 1).into();

        // TODO copypasted from from_save, should be refactored!
        // TODO candidates for refactoring: All this bounds stuff.
        // Define the bounds in a more useful way than two points.
        struct Bounds {
            x: (i64, i64),
            y: (i64, i64),
            z: (i64, i64),
        }

        // Point conversions, to chunk and region coordinates
        let column_p1: BlockColumnCoord = p1.into();
        let chunk_p1: ChunkCoord = column_p1.into();
        let region_p1: RegionCoord = chunk_p1.into();
        let column_p2: BlockColumnCoord = p2.into();
        let chunk_p2: ChunkCoord = column_p2.into();
        let region_p2: RegionCoord = chunk_p2.into();

        // Inclusive block bounds, using global coordinates
        let global_block_bounds = Bounds {
            x: (i64::min(p1.0, p2.0), i64::max(p1.0, p2.0)),
            y: (i64::min(p1.1, p2.1), i64::max(p1.1, p2.1)),
            z: (i64::min(p1.2, p2.2), i64::max(p1.2, p2.2)),
        };

        // Inclusive chunk bounds, using global coordinates
        let global_chunk_bounds = Bounds {
            x: (
                i64::min(chunk_p1.0, chunk_p2.0),
                i64::max(chunk_p1.0, chunk_p2.0),
            ),
            y: (i64::min(p1.1, p2.1), i64::max(p1.1, p2.1)),
            z: (
                i64::min(chunk_p1.1, chunk_p2.1),
                i64::max(chunk_p1.1, chunk_p2.1),
            ),
        };

        // Inclusive region bounds, using global coordinates,
        let region_bounds = Bounds {
            x: (
                i64::min(region_p1.0, region_p2.0),
                i64::max(region_p1.0, region_p2.0),
            ),
            y: (i64::min(p1.1, p2.1), i64::max(p1.1, p2.1)),
            z: (
                i64::min(region_p1.1, region_p2.1),
                i64::max(region_p1.1, region_p2.1),
            ),
        };
        // TODO copypaste from from_save() ended here

        // TODO
        // For each region:
        // - load if exists, or create new one
        // - for each chunk with overlap:
        //      - fetch chunk
        //      - paste into chunk
        //      - put chunk back
        // - write region

        // Iterate through region files within the bound, creating new ones as needed.
        for region_x in region_bounds.x.0..=region_bounds.x.1 {
            for region_z in region_bounds.z.0..=region_bounds.z.1 {
                println!("Importing region {}, {}", region_x, region_z);

                // Check if there actually is a region file for the given region
                let region_file_name = format!("r.{}.{}.mca", region_x, region_z);
                let region_file = world_directory.join("region/").join(&region_file_name);

                let mut region = if region_file.is_file() {
                    Region::load_from_file(&region_file)
                } else {
                    Region::new()
                };

                // TODO copypaste from from_save(), consider refactoring.
                // Figure out what chunks overlaps with the bounding box,
                // expressed in chunk coordinates relative to the region.
                let region_coords: RegionCoord = (region_x, region_z).into();
                let chunk_offset: ChunkCoord = region_coords.into();
                let in_region_chunk_bounds = Bounds {
                    x: (
                        i64::max(global_chunk_bounds.x.0 - chunk_offset.0, 0),
                        i64::min(global_chunk_bounds.x.1 - chunk_offset.0, 31),
                    ),
                    y: (global_chunk_bounds.y.0, global_chunk_bounds.y.1),
                    z: (
                        i64::max(global_chunk_bounds.z.0 - chunk_offset.1, 0),
                        i64::min(global_chunk_bounds.z.1 - chunk_offset.1, 31),
                    ),
                };
                // TODO copypaste from from_save() ended here.

                // Handle those chunks
                for chunk_x in in_region_chunk_bounds.x.0..=in_region_chunk_bounds.x.1 {
                    for chunk_z in in_region_chunk_bounds.z.0..=in_region_chunk_bounds.z.1 {
                        // Get the chunk, or create a new one if empty
                        let chunk_data = region.chunk_data(&(chunk_x, chunk_z).into());
                        let mut chunk = match chunk_data {
                            RawChunkData::Empty => Chunk::new((chunk_x, chunk_z).into()),
                            _ => Chunk::from_raw_chunk_data(&chunk_data),
                        };

                        // Paste blocks into chunk
                        let chunk_coordinates: ChunkCoord = (chunk_x, chunk_z).into();
                        let chunk_block_coordinates: BlockColumnCoord = chunk_coordinates.into();
                        let offset = (
                            global_block_bounds.x.0 - chunk_block_coordinates.0,
                            global_block_bounds.y.0,
                            global_block_bounds.z.0 - chunk_block_coordinates.1,
                        );
                        chunk.blocks.paste(offset, &self.blocks);

                        // TODO Move or copy entities into the chunk

                        // Put chunk back into region
                        let chunk_data = chunk.raw_chunk_zlib();
                        region.set_chunk_data(&(chunk_x, chunk_z).into(), chunk_data);
                    }
                }

                // TODO write back region
                region.save_to_file(&region_file);
            }
        }
    }

    /// Creates a new `WorldExcerpt` from a schematic file.
    pub fn from_schematic(_schematic_file: &std::path::Path) -> Self {
        unimplemented!();
    }

    /// Creates a new `WorldExcerpt` from part of an existing `WorldExcerpt`.
    pub fn from_world_excerpt(_p1: BlockCoord, _p2: BlockCoord, _other: &WorldExcerpt) -> Self {
        unimplemented!();
    }

    /// Set the block at location `at` to the provided block.
    pub fn set_block_at(&mut self, at: BlockCoord, block: Block) {
        self.blocks
            .insert((at.0 as usize, at.1 as usize, at.2 as usize), block);
    }

    /// Get a copy of the block at location `at`.
    pub fn block_at(&self, at: BlockCoord) -> Option<&Block> {
        self.blocks
            .block_at((at.0 as usize, at.1 as usize, at.2 as usize))
    }

    /// Paste the contents of a different WorldExcerpt into this WorldExcerpt.
    ///
    /// The corner of `other` with the lowest numbered coordinates, is aligned at block
    /// coordinates `at` relative to the world excerpt. Only the parts of `other` that
    /// then overlaps with the world excerpt are pasted.
    ///
    /// Empty blocks ([`Block::None`](crate::block::Block::None)) are not copied over,
    /// allowing for pasting other selection shapes than rectangular cuboids.
    pub fn paste(&mut self, at: BlockCoord, other: &WorldExcerpt) {
        self.blocks.paste((at.0, at.1, at.2), &other.blocks);
        // TODO also handle / copy entities within the world excerpts
    }

    //TODO functions for:
    // - exporting the WorldExcerpt to a schematic file
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Log;
    use crate::material::WoodMaterial;

    #[test]
    fn test_basic_functionality() {
        let mut excerpt = WorldExcerpt::new(3, 3, 3);

        fn run_test_at(excerpt: &mut WorldExcerpt, at: BlockCoord, block: Block) {
            excerpt.set_block_at(at, block.clone());
            let block_read_back = excerpt.block_at(at).unwrap();
            if block != *block_read_back {
                panic!("block != block_read_back @ {:?}", at);
            }
        }

        run_test_at(&mut excerpt, (0, 0, 0).into(), Block::Gravel);
        run_test_at(&mut excerpt, (2, 0, 0).into(), Block::Sand);
        run_test_at(
            &mut excerpt,
            (0, 2, 0).into(),
            Block::Log(Log {
                material: WoodMaterial::Oak,
                alignment: None,
                stripped: false,
            }),
        );
        run_test_at(&mut excerpt, (0, 0, 2).into(), Block::WaterSource);
        run_test_at(&mut excerpt, (2, 2, 0).into(), Block::Cobblestone);
        run_test_at(&mut excerpt, (2, 0, 2).into(), Block::Clay);
        run_test_at(&mut excerpt, (0, 2, 2).into(), Block::GrassBlock);
        run_test_at(&mut excerpt, (2, 2, 2).into(), Block::Ice);
    }
}
