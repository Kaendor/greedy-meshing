use crate::{mesh::create_cube_vertices_at, CHUNK_SIZE, CHUNK_SIZE_CUBED, CHUNK_SIZE_SQUARED};
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

pub struct Chunk {
    pub voxels: Vec<Voxel>,
}

impl Chunk {
    pub fn new() -> Self {
        let voxels = (0..CHUNK_SIZE_CUBED)
            .map(|_p| Voxel {
                kind: BlockKind::Rock,
            })
            .collect();
        Chunk { voxels }
    }

    pub fn generate_mesh(&self) -> Mesh {
        let mut cube_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();

        for (i, v) in self.voxels.iter().enumerate() {
            let z = i / (CHUNK_SIZE_SQUARED);

            let ti = i - (z * CHUNK_SIZE_SQUARED);

            let x = ti % CHUNK_SIZE;
            let y = ti / CHUNK_SIZE;
            let index = UVec3::new(x as u32, y as u32, z as u32);

            let (pos, n, id) = create_cube_vertices_at(&index);

            positions.extend(pos);
            normals.extend(n);
            indices.extend(id.into_iter().map(|n| n as u32 + positions.len() as u32));
        }

        cube_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

        // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
        // the surface.
        // Normals are required for correct lighting calculations.
        // Each array represents a normalized vector, which length should be equal to 1.0.
        cube_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

        cube_mesh.set_indices(Some(Indices::U32(indices)));

        cube_mesh
    }
}

pub struct Voxel {
    kind: BlockKind,
}

pub enum BlockKind {
    Rock,
    Air,
}
