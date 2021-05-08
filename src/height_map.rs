use std::fmt;

use crate::utils::vec_i32_into_vec_u32;

//#[derive(Debug)]
#[derive(Clone)]
pub struct HeightMap {
    heights: Vec<u32>,
    x_dim: usize,
    z_dim: usize,
}

impl HeightMap {
    pub fn new((x_dim, z_dim): (usize, usize)) -> Self {
        let heights_len = x_dim * z_dim;
        let heights = vec![0; heights_len];
        Self {
            heights,
            x_dim,
            z_dim,
        }
    }

    /// Create a new heightmap from a HeightMap NBT tag
    /// * NB Pre-flattening heightmap NBT format!
    pub fn from_nbt_value(nbt: nbt::Value) -> Self {
        let (x_dim, z_dim) = (16, 16);
        if let nbt::Value::IntArray(heights) = nbt {
            let heights = vec_i32_into_vec_u32(heights);

            Self {
                heights,
                x_dim,
                z_dim,
            }
        } else {
            panic!("Unexpected nbt value type for height map.");
        }
    }

    pub fn dim(&self) -> (usize, usize) {
        (self.x_dim, self.z_dim)
    }

    pub fn set_height(&mut self, coordinates: (usize, usize), height: u32) {
        if let Some(index) = self.index(coordinates) {
            self.heights[index] = height;
        }
    }

    pub fn height_at(&self, coordinates: (usize, usize)) -> Option<u32> {
        if let Some(index) = self.index(coordinates) {
            Some(*self.heights.get(index).unwrap())
        } else {
            None
        }
    }

    pub fn to_vec_u32(&self) -> Vec<u32> {
        self.heights.clone()
    }

    fn index(&self, (x, z): (usize, usize)) -> Option<usize> {
        if x >= self.x_dim || z >= self.z_dim {
            None
        } else {
            Some(x + self.x_dim * z)
        }
    }
}

impl From<HeightMap> for Vec<u32> {
    fn from(height_map: HeightMap) -> Vec<u32> {
        height_map.heights
    }
}

impl From<HeightMap> for Vec<i32> {
    fn from(height_map: HeightMap) -> Vec<i32> {
        let mut heights = height_map.heights;

        // Hack to convert biomes from Vec<u32> to Vec<i32>, as that is what hematite-nbt needs...
        let p = heights.as_mut_ptr();
        let len = heights.len();
        let cap = heights.capacity();
        std::mem::forget(heights);
        unsafe { Vec::from_raw_parts(p as *mut i32, len, cap) }
    }
}

// Attempt to get a bit more reasonable printing of Height Maps
// (They are 2D-vectors stored in a 1D-vector.)
// TODO Ideally they should get printed similar to a spreadsheet
impl fmt::Debug for HeightMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.heights
            .chunks(self.x_dim)
            .fold(&mut f.debug_list(), |b, e| b.entry(&e))
            .finish()
    }
}
