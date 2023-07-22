use bevy::prelude::*;
// use bevy_inspector_egui::egui::Key;
// use bevy_editor_pls::*;
use last_stanza::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(EditorPlugin)
        // .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        // .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(GamePlugin)
        // .add_startup_system(spawn_window)
        .run();
}
