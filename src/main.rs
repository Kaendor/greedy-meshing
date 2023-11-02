use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::{Face, PrimitiveTopology};
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;
use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    render::render_resource::WgpuFeatures,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use chunk::Chunk;
use diagnostics::MeshDiagnosticPlugin;
use inspector::DiagnosticInspectorPlugin;
use strum::{EnumIter, IntoEnumIterator};

pub const CHUNK_SIZE: usize = 3;
pub const CHUNK_SIZE_SQUARED: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_SIZE_CUBED: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

mod chunk;
mod diagnostics;
mod inspector;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                wgpu_settings: WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                },
            }),
            WireframePlugin,
            MeshDiagnosticPlugin,
            DiagnosticInspectorPlugin,
            PanOrbitCameraPlugin,
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Import the custom texture.
    // Create and save a handle to the mesh.
    let cube_mesh_handle: Handle<Mesh> = meshes.add(create_cube_mesh());

    let chunk = Chunk::new();

    for (i, v) in chunk.voxels.iter().enumerate() {
        let z = i / (CHUNK_SIZE_SQUARED);

        let ti = i - (z * CHUNK_SIZE_SQUARED);

        let x = ti % CHUNK_SIZE;
        let y = ti / CHUNK_SIZE;
        let index = UVec3::new(x as u32, y as u32, z as u32);
        commands.spawn((
            PbrBundle {
                mesh: cube_mesh_handle.clone(),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgba(0.2, 0.7, 0.1, 0.1),
                    alpha_mode: AlphaMode::Mask(0.5),
                    ..default()
                }),
                transform: Transform::from_xyz(index.x as f32, index.y as f32, index.z as f32),
                ..default()
            },
            Wireframe,
        ));
    }

    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_and_light_transform =
        Transform::from_xyz(1.8, 1.8, 1.8).looking_at(Vec3::ZERO, Vec3::Y);

    // Camera in 3D space.
    commands.spawn((
        Camera3dBundle {
            projection: OrthographicProjection {
                scale: 10.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(500., 500.0, 500.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    // Light up the scene.
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            range: 100.0,
            ..default()
        },
        transform: camera_and_light_transform,
        ..default()
    });

    // Text to describe the controls.
}

#[derive(EnumIter, Debug, PartialEq, Eq)]
enum FaceDirection {
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

fn create_cube_vertices_at(pos: &IVec3) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<usize>) {
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

fn create_cube_mesh() -> Mesh {
    let mut cube_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let (vertices, normals, indices) = create_cube_vertices_at(&IVec3::ZERO);

    info!("vertices: {:?}", vertices);
    info!("normals: {:?}", normals);
    info!("indices: {:?}", indices);

    #[rustfmt::skip]
    cube_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertices
    );

    // Set-up UV coordinated to point to the upper (V < 0.5), "dirt+grass" part of the texture.
    // Take a look at the custom image (assets/textures/array_texture.png)
    // so the UV coords will make more sense
    // Note: (0.0, 0.0) = Top-Left in UV mapping, (1.0, 1.0) = Bottom-Right in UV mapping
    #[rustfmt::skip]
    cube_mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            // Assigning the UV coords for the top side.
            [0.0, 0.2], [0.0, 0.0], [1.0, 0.0], [1.0, 0.25],
            // Assigning the UV coords for the bottom side.
            [0.0, 0.45], [0.0, 0.25], [1.0, 0.25], [1.0, 0.45],
            // Assigning the UV coords for the right side.
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the left side. 
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the back side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
            // Assigning the UV coords for the forward side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
        ],
    );

    // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
    // the surface.
    // Normals are required for correct lighting calculations.
    // Each array represents a normalized vector, which length should be equal to 1.0.
    #[rustfmt::skip]
    cube_mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
normals
    );

    // Create the triangles out of the 24 vertices we created.
    // To construct a square, we need 2 triangles, therefore 12 triangles in total.
    // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
    // by one, in a counter-clockwise order (relative to the position of the viewer, the order
    // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
    // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
    // further examples and the implementation of the built-in shapes.
    // vec![
    //         0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
    //         4,5,7 , 5,6,7, // bottom (-y)
    //         8,11,9 , 9,11,10, // right (+x)
    //         12,13,15 , 13,14,15, // left (-x)
    //         16,19,17 , 17,19,18, // back (+z)
    //         20,21,23 , 21,22,23, // forward (-z)
    //     ]
    let indices = indices.into_iter().map(|u| u as u32).collect();
    cube_mesh.set_indices(Some(Indices::U32(indices)));

    cube_mesh
}
