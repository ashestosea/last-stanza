mod enemies;
mod loading;
mod menu;
mod player;
mod world;

use crate::enemies::EnemiesPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;

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
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            //     .add_plugin(ActionsPlugin)
            //     .add_plugin(InternalAudioPlugin)
            .add_plugin(WorldPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemiesPlugin)
            .add_plugin(PhysicsPlugin::default())
            .insert_resource(Gravity::from(Vec2::new(0., -9.81)))
            ;

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}

#[allow(unused)]
#[derive(PhysicsLayer)]
enum PhysicsLayers {
    Ground,
    Enemy,
    Hopper,
    PProj,
    Debug,
}

#[derive(Bundle)]
struct DynamicActorBundle {
    rigidbody: RigidBody,
    material: PhysicMaterial,
    shape: CollisionShape,
    layers: CollisionLayers,
    collisions: Collisions,
    velocity: Velocity,
}

impl Default for DynamicActorBundle {
    fn default() -> Self {
        Self {
            rigidbody: RigidBody::Dynamic,
            material: Default::default(),
            shape: Default::default(),
            layers: Default::default(),
            collisions: Default::default(),
            velocity: Default::default(),
        }
    }
}

#[derive(Component)]
struct MainCamera;
