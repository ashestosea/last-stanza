use bevy::prelude::*;
// use bevy_editor_pls::*;
use heron::prelude::*;

fn main() {
	App::new()
	.add_plugins(DefaultPlugins)
	// .add_plugin(EditorPlugin)
	// .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
	// .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
	.add_plugin(PhysicsPlugin::default())
	.run();
}
