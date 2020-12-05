//! A piece of a Minecraft world.

use crate::block::Block;
use crate::chunk::Chunk;
use crate::coordinates::*;
use crate::nbt_lookup::*;
use crate::region::Region;

extern crate nbt;
extern crate ndarray;

/// Structure for holding blocks and entities, representing a piece of a Minecraft world.
pub struct WorldExcerpt {
    blocks: ndarray::Array3<Block>,
}

impl WorldExcerpt {
    /// Creates a new empty `WorldExcerpt` of the given size.
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        WorldExcerpt {
            blocks: ndarray::Array3::<Block>::from_elem((x, y, z), Block::None),
        }
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

        let data_version = nbt_blob_lookup_int(&level_dat_blob, "Data/DataVersion")
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
            x_min: i64,
            x_max: i64,
            y_min: i64,
            y_max: i64,
            z_min: i64,
            z_max: i64,
        };

        // Point conversions, to chunk and region coordinates
        let column_p1: BlockColumnCoord = p1.into();
        let chunk_p1: ChunkCoord = column_p1.into();
        let region_p1: RegionCoord = chunk_p1.into();
        let column_p2: BlockColumnCoord = p2.into();
        let chunk_p2: ChunkCoord = column_p2.into();
        let region_p2: RegionCoord = chunk_p2.into();

        // Inclusive block bounds, using global coordinates
        let global_block_bounds = Bounds {
            x_min: i64::min(p1.0, p2.0),
            x_max: i64::max(p1.0, p2.0),
            y_min: i64::min(p1.1, p2.1),
            y_max: i64::max(p1.1, p2.1),
            z_min: i64::min(p1.2, p2.2),
            z_max: i64::max(p1.2, p2.2),
        };

        // Inclusive chunk bounds, using global coordinates
        let global_chunk_bounds = Bounds {
            x_min: i64::min(chunk_p1.0, chunk_p2.0),
            x_max: i64::max(chunk_p1.0, chunk_p2.0),
            y_min: i64::min(p1.1, p2.1),
            y_max: i64::max(p1.1, p2.1),
            z_min: i64::min(chunk_p1.1, chunk_p2.1),
            z_max: i64::max(chunk_p1.1, chunk_p2.1),
        };

        // Inclusive region bounds, using global coordinates,
        let region_bounds = Bounds {
            x_min: i64::min(region_p1.0, region_p2.0),
            x_max: i64::max(region_p1.0, region_p2.0),
            y_min: i64::min(p1.1, p2.1),
            y_max: i64::max(p1.1, p2.1),
            z_min: i64::min(region_p1.1, region_p2.1),
            z_max: i64::max(region_p1.1, region_p2.1),
        };

        // Iterate through the (existing) region files within the bound
        for region_x in region_bounds.x_min..=region_bounds.x_max {
            for region_z in region_bounds.z_min..=region_bounds.z_max {
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
                    x_min: i64::max(global_chunk_bounds.x_min - chunk_offset.0, 0),
                    x_max: i64::min(global_chunk_bounds.x_max - chunk_offset.0, 31),
                    y_min: global_chunk_bounds.y_min,
                    y_max: global_chunk_bounds.y_max,
                    z_min: i64::max(global_chunk_bounds.z_min - chunk_offset.1, 0),
                    z_max: i64::min(global_chunk_bounds.z_max - chunk_offset.1, 31),
                };

                // Handle those chunks
                for chunk_x in in_region_chunk_bounds.x_min..=in_region_chunk_bounds.x_max {
                    for chunk_z in in_region_chunk_bounds.z_min..=in_region_chunk_bounds.z_max {
                        println!("Handling (region internal) chunk {}, {}", chunk_x, chunk_z);

                        // Parse the raw chunk data into a chunk object
                        let chunk_data = region.get_chunk_data(&(chunk_x, chunk_z).into());
                        let chunk = Chunk::from_raw_chunk_data(&chunk_data);

                        //TODO Read out the blocks, and put them in the WorldExcerpt
                        //TODO Read out entities, and put them in the WorldExcerpt
                    }
                }
            }
        }

        // Return the constructed WorldExcerpt
        world_excerpt
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
        self.blocks[[at.0 as usize, at.1 as usize, at.2 as usize]] = block;
    }

    /// Get a copy of the block at location `at`.
    pub fn get_block_at(&self, at: BlockCoord) -> Block {
        self.blocks[[at.0 as usize, at.1 as usize, at.2 as usize]].clone()
    }

    /// Paste the contents of a different WorldExcerpt into this WorldExcerpt.
    ///
    /// Empty blocks ([`Block::None`](crate::block::Block::None)) are not copied over,
    /// allowing for pasting non-rectangular cuboid selections.
    pub fn paste(&mut self, at: BlockCoord, other: &WorldExcerpt) {
        for x in 0..other.blocks.len_of(ndarray::Axis(0)) as i64 {
            let self_x = x + at.0;
            if self_x < 0 || self_x >= self.blocks.len_of(ndarray::Axis(0)) as i64 {
                // Do not paste outside of self bounding box.
                continue;
            }
            for y in 0..other.blocks.len_of(ndarray::Axis(1)) as i64 {
                let self_y = y + at.1;
                if self_y < 0 || self_y >= self.blocks.len_of(ndarray::Axis(1)) as i64 {
                    // Do not paste outside of self bounding box.
                    continue;
                }
                for z in 0..other.blocks.len_of(ndarray::Axis(2)) as i64 {
                    let self_z = z + at.2;
                    if self_z < 0 || self_z >= self.blocks.len_of(ndarray::Axis(2)) as i64 {
                        // Do not paste outside of self bounding box.
                        continue;
                    }

                    // The actual pasting of blocks
                    let block = other.get_block_at((x, y, z).into());
                    if Block::None != block {
                        self.set_block_at((self_x, self_y, self_z).into(), block);
                    }
                }
            }
        }
    }

    //TODO functions for:
    // - pasting the WorldExcerpt into an existing world save
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
            let block_read_back = excerpt.get_block_at(at);
            if block != block_read_back {
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
