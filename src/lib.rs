mod menu;
mod world;
mod player;
mod enemies;

use crate::menu::MenuPlugin;
use crate::world::WorldPlugin;
use crate::player::PlayerPlugin;
use crate::enemies::EnemiesPlugin;

use bevy::app::App;
use bevy::prelude::*;
use heron::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
	Loading,
	Menu,
	Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
			.add_state(GameState::Menu)
        //     .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
        //     .add_plugin(ActionsPlugin)
        //     .add_plugin(InternalAudioPlugin)
            .add_plugin(WorldPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemiesPlugin)
			.add_plugin(PhysicsPlugin::default())
			.insert_resource(Gravity::from(Vec2::new(0., -9.81)));

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}

#[derive(PhysicsLayer)]
enum PhysicsLayers {
	World,
	Hopper,
	Debug
}