use bevy::{
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
};
use greedy_meshing::*;

#[test]
fn naive_mesh_simple() {
    let size = 1;
    let chunk = Chunk::new(size);
    let chunk_mesh = generate_naive_mesh(&chunk.voxels, &chunk);

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
