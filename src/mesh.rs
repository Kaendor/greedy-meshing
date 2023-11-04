use bevy::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Debug, PartialEq, Eq)]
pub enum FaceDirection {
    Top,
    Bottom,
    Right,
    Left,
    Back,
    Forward,
}

impl FaceDirection {
    fn vertices(&self) -> [[f32; 3]; 4] {
        match self {
            FaceDirection::Top => [
                [-0.5, 0.5, -0.5], // vertex with index 0
                [0.5, 0.5, -0.5],  // vertex with index 1
                [0.5, 0.5, 0.5],   // etc. until 23
                [-0.5, 0.5, 0.5],
            ],
            FaceDirection::Bottom => [
                [-0.5, -0.5, -0.5],
                [0.5, -0.5, -0.5],
                [0.5, -0.5, 0.5],
                [-0.5, -0.5, 0.5],
            ],
            FaceDirection::Right => [
                [0.5, -0.5, -0.5],
                [0.5, -0.5, 0.5],
                [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
                [0.5, 0.5, -0.5],
            ],
            FaceDirection::Left => [
                [-0.5, -0.5, -0.5],
                [-0.5, -0.5, 0.5],
                [-0.5, 0.5, 0.5],
                [-0.5, 0.5, -0.5],
            ],
            FaceDirection::Back => [
                [-0.5, -0.5, 0.5],
                [-0.5, 0.5, 0.5],
                [0.5, 0.5, 0.5],
                [0.5, -0.5, 0.5],
            ],
            FaceDirection::Forward => [
                [-0.5, -0.5, -0.5],
                [-0.5, 0.5, -0.5],
                [0.5, 0.5, -0.5],
                [0.5, -0.5, -0.5],
            ],
        }
    }

    // 0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
    // 4,5,7 , 5,6,7, // bottom (-y)
    // 8,11,9 , 9,11,10, // right (+x)
    // 12,13,15 , 13,14,15, // left (-x)
    // 16,19,17 , 17,19,18, // back (+z)
    // 20,21,23 , 21,22,23, // forward (-z)
    fn indices(&self, i: usize) -> [usize; 6] {
        let i = i * 4;
        match self {
            FaceDirection::Top | FaceDirection::Right | FaceDirection::Back => {
                [i, i + 3, i + 1, i + 1, i + 3, i + 2]
            }
            FaceDirection::Bottom | FaceDirection::Left | FaceDirection::Forward => {
                [i, i + 1, i + 3, i + 1, i + 2, i + 3]
            }
        }
    }

    fn normals(&self) -> [[f32; 3]; 4] {
        match self {
            FaceDirection::Top => [
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
            ],
            FaceDirection::Bottom => [
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
            ],
            FaceDirection::Right => [
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
            ],
            FaceDirection::Left => [
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
            ],
            FaceDirection::Back => [
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ],
            FaceDirection::Forward => [
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
            ],
        }
    }
}

pub fn create_cube_vertices_at(pos: &UVec3) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<usize>) {
    // Each array is an [x, y, z] coordinate in local space.
    // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
    // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
    // f.vertices().into_iter().zip(f.normals().into_iter())
    let (pos, n) = FaceDirection::iter()
        // for each face, associate normals
        .flat_map(|f| f.vertices().into_iter().zip(f.normals().into_iter()))
        // add the required position to each vertex
        .map(|(v, n)| (Vec3::from_array(v) + pos.as_vec3(), n))
        .map(|(v, n)| (v.to_array(), n))
        .unzip();

    let indices = FaceDirection::iter()
        .enumerate()
        .flat_map(|(i, f)| f.indices(i))
        .collect();

    (pos, n, indices)
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::FaceDirection;

    #[test]
    fn indice_gen() {
        let expected_indices = vec![
            0, 3, 1, 1, 3, 2, // triangles making up the top (+y) facing side.
            4, 5, 7, 5, 6, 7, // bottom (-y)
            8, 11, 9, 9, 11, 10, // right (+x)
            12, 13, 15, 13, 14, 15, // left (-x)
            16, 19, 17, 17, 19, 18, // back (+z)
            20, 21, 23, 21, 22, 23, // forward (-z)
        ];
        let indices: Vec<usize> = FaceDirection::iter()
            .enumerate()
            .flat_map(|(i, f)| f.indices(i))
            .collect();

        assert_eq!(indices, expected_indices);
    }
}
