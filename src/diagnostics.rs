use bevy::{
    diagnostic::{Diagnostic, DiagnosticId, Diagnostics, RegisterDiagnostic},
    prelude::*,
};

pub struct MeshDiagnosticPlugin;

pub const VERTICES_COUNT: DiagnosticId =
    DiagnosticId::from_u128(66088859776125344860643055657637203675);
impl Plugin for MeshDiagnosticPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_diagnostic(Diagnostic::new(VERTICES_COUNT, "vertices_count", 10))
            .add_systems(Update, vertice_counting);
    }
}

fn vertice_counting(
    mut diagnostics: Diagnostics,
    meshes: Query<&Handle<Mesh>>,
    assets: Res<Assets<Mesh>>,
) {
    diagnostics.add_measurement(VERTICES_COUNT, || {
        let sum = meshes.iter().fold(0, |mut acc, h| {
            let m = assets.get(h).expect("cube mesh");
            let v = m.count_vertices();
            acc += v;
            acc
        });
        sum as f64
    });
}
