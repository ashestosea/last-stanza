use bevy::prelude::*;
// use bevy_inspector_egui::egui::Key;
// use bevy_editor_pls::*;
use last_stanza::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "The Last Stanza".into(),
                name: Some("last-stanza".into()),

                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        // .add_plugins(EditorPlugin)
        .add_plugins(GamePlugin)
        // .add_startup_system(spawn_window)
        .run();
}
