//! A three-dimensional grid of voxels, representing a piece of a Minecraft world.

use crate::block::Block;
use crate::coordinates::BlockCoord;
extern crate ndarray;

/// Structure for holding blocks and entities, representing part of a Minecraft world.
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
    pub fn from_save(
        world_directory: &std::path::Path,
        p1: BlockCoord,
        p2: BlockCoord,
    ) -> WorldExcerpt {
        unimplemented!();
    }

    /// Creates a new `WorldExcerpt` from a schematics file.
    pub fn from_schematic(schematic_file: &std::path::Path) {
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
