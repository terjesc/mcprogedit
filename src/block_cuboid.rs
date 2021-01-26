use crate::block::Block;
use crate::height_map::HeightMap;

#[derive(Debug)]
pub struct BlockCuboid {
    blocks: Vec<Block>,
    x_dim: usize,
    y_dim: usize,
    z_dim: usize,
}

impl BlockCuboid {
    pub fn new((x_dim, y_dim, z_dim): (usize, usize, usize)) -> Self {
        let blocks_len = x_dim * y_dim * z_dim;
        let mut blocks = Vec::with_capacity(blocks_len);
        blocks.resize(blocks_len, Block::None);
        Self {
            blocks,
            x_dim,
            y_dim,
            z_dim,
        }
    }

    pub fn dim(&self) -> (usize, usize, usize) {
        (self.x_dim, self.y_dim, self.z_dim)
    }

    pub fn insert(&mut self, coordinates: (usize, usize, usize), block: Block) {
        let index = self.index(coordinates).unwrap();
        self.blocks[index] = block;
    }

    pub fn block_at(&self, coordinates: (usize, usize, usize)) -> Option<&Block> {
        if let Some(index) = self.index(coordinates) {
            self.blocks.get(index)
        } else {
            None
        }
    }

    pub fn _block_at_mut(&mut self, coordinates: (usize, usize, usize)) -> Option<&mut Block> {
        if let Some(index) = self.index(coordinates) {
            self.blocks.get_mut(index)
        } else {
            None
        }
    }

    /// Paste the contents of a different BlockCuboid into this BlockCuboid.
    ///
    /// Empty blocks ([`Block::None`](crate::block::Block::None)) are not copied over,
    /// allowing for pasting non-rectangular cuboid selections.
    pub fn paste(&mut self, offset: (i64, i64, i64), other: &Self) {
        // Calculate the spans relative to self, for where blocks are to be pasted in.
        let min = (
            i64::max(0, offset.0) as usize, // x
            i64::max(0, offset.1) as usize, // y
            i64::max(0, offset.2) as usize, // z
        );
        let max = (
            i64::min(self.x_dim as i64 - 1, offset.0 + other.x_dim as i64 - 1) as usize, // x
            i64::min(self.y_dim as i64 - 1, offset.1 + other.y_dim as i64 - 1) as usize, // y
            i64::min(self.z_dim as i64 - 1, offset.2 + other.z_dim as i64 - 1) as usize, // z
        );

        for to_x in min.0..=max.0 {
            let from_x = (to_x as i64 - offset.0) as usize;
            for to_y in min.1..=max.1 {
                let from_y = (to_y as i64 - offset.1) as usize;
                for to_z in min.2..=max.2 {
                    let from_z = (to_z as i64 - offset.2) as usize;
                    if let Some(block) = other.block_at((from_x, from_y, from_z)) {
                        if *block != Block::None {
                            self.insert((to_x, to_y, to_z), block.clone());
                        }
                    } else {
                        eprintln!("[warning] Tried to paste block from invalid source position");
                    }
                }
            }
        }
    }

    pub fn height_map(&self) -> HeightMap {
        let mut height_map = HeightMap::new((self.x_dim, self.z_dim));

        for x in 0..self.x_dim {
            for z in 0..self.z_dim {
                let mut height = 0;
                //for y in std::iter::range_step(self.y_dim as i32, 0, -1) {
                for y in (0..self.y_dim).rev() {
                    if let Some(Block::Air) = self.block_at((x, y as usize, z)) {
                        height = y;
                        break;
                    }
                }
                height_map.set_height((x, z), height as u32);
            }
        }

        height_map
    }

    fn index(&self, (x, y, z): (usize, usize, usize)) -> Option<usize> {
        if x >= self.x_dim || y >= self.y_dim || z >= self.z_dim {
            None
        } else {
            Some(self.y_dim * self.z_dim * x + self.y_dim * z + y)
        }
    }
}
