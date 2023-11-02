use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
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
use mesh::create_cube_mesh;

pub const CHUNK_SIZE: usize = 3;
pub const CHUNK_SIZE_SQUARED: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_SIZE_CUBED: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

mod chunk;
mod diagnostics;
mod inspector;
mod mesh;

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
