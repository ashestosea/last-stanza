#[cfg(debug_assertions)]
mod debug;
mod enemies;
pub mod events;
mod loading;
mod main_camera;
mod menu;
mod player;
mod world;

use crate::enemies::EnemiesPlugin;
use crate::events::EventsPlugin;
use crate::loading::LoadingPlugin;
use crate::main_camera::MainCameraPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;

use bevy::app::App;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
#[cfg(debug_assertions)]
use debug::DebugPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, SystemSet)]
enum GameState {
    #[default]
    Loading,
    Menu,
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins(LoadingPlugin)
            .add_plugins(EventsPlugin)
            .add_plugins(MainCameraPlugin)
            .add_plugins(MenuPlugin)
            .add_plugins(WorldPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemiesPlugin)
            .add_plugins(PhysicsPlugins::default())
            // .add_systems(Update, cleanup_far_entities.run_if(in_state(GameState::Playing)))
            // .insert_resource(Gravity::from(Vec2::new(0.0, -9.81)));
            ;

        #[cfg(debug_assertions)]
        {
            app.add_plugins(DebugPlugin);
        }
    }
}

// fn cleanup_far_entities(mut commands: Commands, query: Query<(Entity, &Transform)>) {
//     for (entity, transform) in query.iter() {
//         if transform.translation.x < -50.
//             || transform.translation.x > 50.
//             || transform.translation.y < -50.
//             || transform.translation.y > 50.
//         {
//             commands.entity(entity).despawn();
//         }
//     }
// }

#[derive(PhysicsLayer)]
pub(crate) enum PhysicsLayers {
    Ground,
    CliffEdge,
    Player,
    Enemy,
    Hopper,
    Climber,
    Lurker,
    Diver,
    Giant,
    Behemoth,
    PlayerProj,
    EnemyProj,
    Explosion,
    // Debug,
}

#[derive(Bundle)]
struct DynamicActorBundle {
    rigidbody: RigidBody,
    locked_axes: LockedAxes,
    collider: Collider,
    collision_layers: CollisionLayers,
    colliding_entities: CollidingEntities,
    friction: Friction,
    restitution: Restitution,
    mass: Mass,
    velocity: LinearVelocity,
}

impl Default for DynamicActorBundle {
    fn default() -> Self {
        Self {
            rigidbody: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collider: Default::default(),
            collision_layers: Default::default(),
            colliding_entities: Default::default(),
            friction: Default::default(),
            restitution: Default::default(),
            mass: Default::default(),
            velocity: Default::default(),
        }
    }
}
