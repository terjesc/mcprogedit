//! A three-dimensional grid of voxels, representing a piece of a Minecraft world.

use crate::block::Block;
use crate::coordinates::BlockCoord;
extern crate ndarray;

// TODO find a good name for a rectangular cuboid.
//      ("Box" is sadly a reserved word in Rust, and "canvas" feels off.)
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

    /// Creates a new `WorldExcerpt` from part of a saved world.
    ///
    /// `p1` and `p2` defines a bounding box for what portion of the world
    /// saved at `world_directory` to import into the new `WorldExcerpt`.
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

    pub fn set_block_at(&mut self, at: (usize, usize, usize), block: Block) {
        let (x, y, z) = at;
        self.blocks[[x, y, z]] = block;
    }

    pub fn get_block_at(&self, at: (usize, usize, usize)) -> Block {
        let (x, y, z) = at;
        self.blocks[[x, y, z]].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Log;
    use crate::material::WoodMaterial;

    #[test]
    fn test_basic_functionality() {
        let mut excerpt = WorldExcerpt::new(3, 3, 3);

        fn run_test_at(excerpt: &mut WorldExcerpt, at: (usize, usize, usize), block: Block) {
            excerpt.set_block_at(at, block.clone());
            let block_read_back = excerpt.get_block_at(at);
            if block != block_read_back {
                panic!("block != block_read_back @ {:?}", at);
            }
        }

        run_test_at(&mut excerpt, (0, 0, 0), Block::Gravel);
        run_test_at(&mut excerpt, (2, 0, 0), Block::Sand);
        run_test_at(
            &mut excerpt,
            (0, 2, 0),
            Block::Log(Log {
                material: WoodMaterial::Oak,
                alignment: None,
                stripped: false,
            }),
        );
        run_test_at(&mut excerpt, (0, 0, 2), Block::WaterSource);
        run_test_at(&mut excerpt, (2, 2, 0), Block::Cobblestone);
        run_test_at(&mut excerpt, (2, 0, 2), Block::Clay);
        run_test_at(&mut excerpt, (0, 2, 2), Block::GrassBlock);
        run_test_at(&mut excerpt, (2, 2, 2), Block::Ice);
    }
}
