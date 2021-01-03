use crate::block::Block;

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

    pub fn insert(&mut self, coordinates: (usize, usize, usize), block: Block) {
        let index = self.index(coordinates).unwrap();
        self.blocks[index] = block;
    }

    pub fn get(&self, coordinates: (usize, usize, usize)) -> Option<&Block> {
        let index = self.index(coordinates).unwrap();
        self.blocks.get(index)
    }

    pub fn get_mut(&mut self, coordinates: (usize, usize, usize)) -> Option<&mut Block> {
        let index = self.index(coordinates).unwrap();
        self.blocks.get_mut(index)
    }

    fn index(&self, (x, y, z): (usize, usize, usize)) -> Option<usize> {
        let index = self.y_dim * self.z_dim * x + self.y_dim * z + y;
        if index >= self.blocks.len() {
            None
        } else {
            Some(index)
        }
    }
}
