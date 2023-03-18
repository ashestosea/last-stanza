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
use bevy::reflect::FromReflect;
use bevy_rapier2d::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    Menu,
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(LoadingPlugin)
            .add_plugin(EventsPlugin)
            .add_plugin(MainCameraPlugin)
            .add_plugin(MenuPlugin)
            //     .add_plugin(ActionsPlugin)
            //     .add_plugin(InternalAudioPlugin)
            .add_plugin(WorldPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemiesPlugin)
            // .add_plugin(PhysicsPlugin::default())
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_system(cleanup_far_entities.in_set(OnUpdate(GameState::Playing)));
            // .insert_resource(Gravity::from(Vec2::new(0.0, -9.81)));

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}

fn cleanup_far_entities(mut commands: Commands, query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        if transform.translation.x < -50.
            || transform.translation.x > 50.
            || transform.translation.y < -50.
            || transform.translation.y > 50.
        {
            commands.entity(entity).despawn();
        }
    }
}

bitflags::bitflags! {
    /// A bit mask identifying groups for interaction.
    #[derive(Component, Reflect, FromReflect)]
    #[reflect(Component, Hash, PartialEq)]
    #[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
    pub struct PhysicLayer: u32 {
        /// The group n°1.
        const GROUND = 1 << 0;
        /// The group n°2.
        const CLIFF_EDGE = 1 << 1;
        /// The group n°3.
        const PLAYER = 1 << 2;
        /// The group n°4.
        const ENEMY = 1 << 3;
        /// The group n°5.
        const HOPPER = 1 << 4;
        /// The group n°6.
        const CLIMBER = 1 << 5;
        /// The group n°7.
        const GIANT = 1 << 6;
        /// The group n°8.
        const PLAYER_PROJ = 1 << 7;
        /// The group n°9.
        const ENEMY_PROJ = 1 << 8;
        /// The group n°10.
        const DEBUG = 1 << 9;

        /// All of the groups.
        const ALL = u32::MAX;
        /// None of the groups.
        const NONE = 0;
    }
}

impl Default for PhysicLayer {
    fn default() -> Self {
        PhysicLayer::ALL
    }
}

impl Into<bevy_rapier2d::prelude::Group> for PhysicLayer {
    fn into(self) -> bevy_rapier2d::prelude::Group {
        Group::from_bits_truncate(self.bits)
    }
}

#[derive(Bundle)]
struct DynamicActorBundle {
    rigidbody: RigidBody,
    locked_axes: LockedAxes,
    collider: Collider,
    collision_groups: CollisionGroups,
    colliding_entities: CollidingEntities,
    friction: Friction,
    restitution: Restitution,
    mass: ColliderMassProperties,
    velocity: Velocity,
}

impl Default for DynamicActorBundle {
    fn default() -> Self {
        Self {
            rigidbody: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collider: Default::default(),
            collision_groups: Default::default(),
            colliding_entities: Default::default(),
            friction: Default::default(),
            restitution: Default::default(),
            mass: ColliderMassProperties::Density(1.0),
            velocity: Default::default(),
        }
    }
}
