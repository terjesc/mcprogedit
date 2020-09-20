use crate::block::Block;
extern crate ndarray;

// TODO find a good name for a rectangular cuboid.
//      ("Box" is sadly a reserved word in Rust, and "canvas" feels off.)
pub struct Canvas {
    canvas: ndarray::Array3<Block>,
}

impl Canvas {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Canvas {
            canvas: ndarray::Array3::<Block>::from_elem((x, y, z), Block::Air),
        }
    }

    pub fn set_block_at(&mut self, at: (usize, usize, usize), block: Block) {
        let (x, y, z) = at;
        self.canvas[[x, y, z]] = block;
    }

    pub fn get_block_at(&self, at: (usize, usize, usize)) -> Block {
        let (x, y, z) = at;
        self.canvas[[x, y, z]].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Log;
    use crate::material::WoodMaterial;

    #[test]
    fn test_basic_functionality() {
        let mut test_canvas = Canvas::new(3, 3, 3);

        fn run_test_at(test_canvas: &mut Canvas, at: (usize, usize, usize), block: Block) {
            test_canvas.set_block_at(at, block.clone());
            let block_read_back = test_canvas.get_block_at(at);
            if block != block_read_back {
                panic!("block != block_read_back @ {:?}", at);
            }
        }

        run_test_at(&mut test_canvas, (0, 0, 0), Block::Gravel);
        run_test_at(&mut test_canvas, (2, 0, 0), Block::Sand);
        run_test_at(
            &mut test_canvas,
            (0, 2, 0),
            Block::Log(Log {
                material: WoodMaterial::Oak,
                alignment: None,
                stripped: false,
            }),
        );
        run_test_at(&mut test_canvas, (0, 0, 2), Block::WaterSource);
        run_test_at(&mut test_canvas, (2, 2, 0), Block::Cobblestone);
        run_test_at(&mut test_canvas, (2, 0, 2), Block::Clay);
        run_test_at(&mut test_canvas, (0, 2, 2), Block::GrassBlock);
        run_test_at(&mut test_canvas, (2, 2, 2), Block::Ice);
    }
}
