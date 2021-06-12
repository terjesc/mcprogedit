use crate::block::light::*;

#[derive(Clone, Debug)]
pub struct LightCuboid {
    light_levels: Vec<u8>,
    x_dim: usize,
    y_dim: usize,
    z_dim: usize,
}

impl LightCuboid {
    pub fn new((x_dim, y_dim, z_dim): (usize, usize, usize)) -> Self {
        Self::new_filled((x_dim, y_dim, z_dim), 0u8)
    }

    pub fn new_filled((x_dim, y_dim, z_dim): (usize, usize, usize), light_level: u8) -> Self {
        let light_levels_len = x_dim * y_dim * z_dim;
        let mut light_levels = Vec::with_capacity(light_levels_len);
        light_levels.resize(light_levels_len, light_level);
        Self {
            light_levels,
            x_dim,
            y_dim,
            z_dim,
        }
    }

    pub fn dim(&self) -> (usize, usize, usize) {
        (self.x_dim, self.y_dim, self.z_dim)
    }

    pub fn set_light_level_at(&mut self, coordinates: (usize, usize, usize), light_level: u8) {
        if let Some(index) = self.index(coordinates) {
            self.light_levels[index] = light_level;
        } else {
            eprintln!(
                "[warning] failed to set light level {:?} at invalid coordinates {:?}",
                light_level,
                coordinates,
            );
        }
    }

    pub fn light_level_at(&self, coordinates: (usize, usize, usize)) -> Option<u8> {
        if let Some(index) = self.index(coordinates) {
            self.light_levels.get(index).copied()
        } else {
            None
        }
    }

    /// Paste the contents of a different LightCuboid into this LightCuboid.
    ///
    /// The corner of `other` with the lowest numbered coordinates, is aligned at block
    /// coordinates `at` relative to the light cuboid. Only the parts of `other` that
    /// then overlaps with the light cuboid are pasted.
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
                    if let Some(light_level) = other.light_level_at((from_x, from_y, from_z)) {
                        self.set_light_level_at((to_x, to_y, to_z), light_level);
                    } else {
                        eprintln!("[warning] Tried to paste light level from invalid source position ({}, {}, {})", from_x, from_y, from_z);
                    }
                }
            }
        }
    }

    /// Creates a new `LightCuboid` from part of an existing `LightCuboid`.
    pub fn from_light_cuboid(
        p1: (usize, usize, usize),
        p2: (usize, usize, usize),
        other: &Self,
    ) -> Self {
        let min = (
            usize::min(p1.0, p2.0),
            usize::min(p1.1, p2.1),
            usize::min(p1.2, p2.2),
        );
        let max = (
            usize::max(p1.0, p2.0),
            usize::max(p1.1, p2.1),
            usize::max(p1.2, p2.2),
        );

        let dimensions = (max.0 - min.0, max.1 - min.1, max.2 - min.2);

        let mut cuboid = Self::new(dimensions);

        for from_x in min.0..=max.0 {
            let to_x = from_x - min.0;
            for from_y in min.1..=max.1 {
                let to_y = from_y - min.1;
                for from_z in min.2..=max.2 {
                    let to_z = from_z - min.2;
                    if let Some(light_level) = other.light_level_at((from_x, from_y, from_z)) {
                        cuboid.set_light_level_at((to_x, to_y, to_z), light_level);
                    } else {
                        eprintln!("[warning] Tried to copy light level from invalid source position ({}, {}, {})", from_x, from_y, from_z);
                    }
                }
            }
        }

        cuboid
    }

    fn index(&self, (x, y, z): (usize, usize, usize)) -> Option<usize> {
        if x >= self.x_dim || y >= self.y_dim || z >= self.z_dim {
            None
        } else {
            Some(self.y_dim * self.z_dim * x + self.y_dim * z + y)
        }
    }
}
