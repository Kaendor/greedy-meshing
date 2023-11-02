use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct DiagnosticInspectorPlugin;

impl Plugin for DiagnosticInspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, diagnostic_display)
            .add_plugins(EguiPlugin);
    }
}

fn diagnostic_display(store: Res<DiagnosticsStore>, mut contexts: EguiContexts) {
    egui::Window::new("Diagnostics").show(contexts.ctx_mut(), |ui| {
        for diag in store.iter() {
            let value = diag.value().unwrap_or_default();
            ui.label(format!("{}: {:.4}", diag.name.clone(), value));
        }
    });
}
