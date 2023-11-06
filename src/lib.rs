pub mod mesh;

use bevy::prelude::*;
pub use mesh::{generate_naive_mesh, VoxelContainer};

/// Only here for testing purpose, use your own struct
pub struct Chunk {
    pub size: usize,
    size_squared: usize,
    pub voxels: Vec<Voxel>,
}

impl Chunk {
    pub fn new(size: usize) -> Self {
        let voxels = (0..size.pow(3))
            .map(|_p| Voxel {
                kind: BlockKind::Rock,
            })
            .collect();
        Chunk {
            voxels,
            size,
            size_squared: size * size,
        }
    }

    fn pos_from_index(&self, i: usize) -> UVec3 {
        let z = i / (self.size_squared);

        let ti = i - (z * self.size_squared);

        let x = ti % self.size;
        let y = ti / self.size;

        UVec3::new(x as u32, y as u32, z as u32)
    }
}

#[allow(dead_code)]
pub struct Voxel {
    kind: BlockKind,
}

pub enum BlockKind {
    Rock,
    Air,
}

impl VoxelContainer for Chunk {
    fn index_to_position(&self, i: usize) -> UVec3 {
        self.pos_from_index(i)
    }
}
