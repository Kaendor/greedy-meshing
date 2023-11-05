use crate::mesh::create_cube_vertices_at;
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

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

    pub fn generate_naive_mesh(&self) -> Mesh {
        let mut cube_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();

        for (i, _v) in self.voxels.iter().enumerate() {
            let index = self.pos_from_index(i);

            let (pos, n, id) = create_cube_vertices_at(&index);

            indices.extend(id.into_iter().map(|n| n as u32 + positions.len() as u32));
            positions.extend(pos);
            normals.extend(n);
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

#[cfg(test)]
mod tests {
    use bevy::{
        prelude::Mesh,
        render::mesh::{Indices, VertexAttributeValues},
    };

    use super::Chunk;

    #[test]
    fn naive_mesh_simple() {
        let size = 1;
        let chunk = Chunk::new(size);
        let chunk_mesh = chunk.generate_naive_mesh();

        let vertices_count = chunk_mesh.count_vertices();
        let Indices::U32(indices) = chunk_mesh.indices().expect("indices") else {
            panic!("no indices of u32")
        };

        assert_eq!(vertices_count, 24);
        assert_eq!(indices.len(), 36);

        let expected_vertices = vec![
            // top (facing towards +y)
            [-0.5, 0.5, -0.5], // vertex with index 0
            [0.5, 0.5, -0.5],  // vertex with index 1
            [0.5, 0.5, 0.5],   // etc. until 23
            [-0.5, 0.5, 0.5],
            // bottom   (-y)
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, 0.5],
            // right    (+x)
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
            [0.5, 0.5, -0.5],
            // left     (-x)
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
            // back     (+z)
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
            // forward  (-z)
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
        ];

        let VertexAttributeValues::Float32x3(vertices) = chunk_mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("vertices")
        else {
            panic!("vertice vec")
        };

        let expected_indices = vec![
            0, 3, 1, 1, 3, 2, // triangles making up the top (+y) facing side.
            4, 5, 7, 5, 6, 7, // bottom (-y)
            8, 11, 9, 9, 11, 10, // right (+x)
            12, 13, 15, 13, 14, 15, // left (-x)
            16, 19, 17, 17, 19, 18, // back (+z)
            20, 21, 23, 21, 22, 23, // forward (-z)
        ];

        assert_eq!(vertices, &expected_vertices);
        assert_eq!(indices, &expected_indices);
    }
}
