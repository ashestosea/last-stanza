use bevy::prelude::*;
// use bevy_inspector_egui::egui::Key;
// use bevy_editor_pls::*;
use last_stanza::GamePlugin;

fn main() {
	App::new()
	.add_plugins(DefaultPlugins)
	// .add_plugin(EditorPlugin)
	// .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
	// .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
	.add_plugin(GamePlugin)
	.run();
}


