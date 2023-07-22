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
            .add_plugins(LoadingPlugin)
            .add_plugins(EventsPlugin)
            .add_plugins(MainCameraPlugin)
            .add_plugins(MenuPlugin)
            //     .add_plugins(ActionsPlugin)
            //     .add_plugins(InternalAudioPlugin)
            .add_plugins(WorldPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemiesPlugin)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(72.0))
            // .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Update, cleanup_far_entities.in_set(GameState::Playing))
            .add_systems(Startup, setup_rapier);
        // .insert_resource(Gravity::from(Vec2::new(0.0, -9.81)));

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugins(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugins(LogDiagnosticsPlugin::default());
        // }
    }
}

fn setup_rapier(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::Y * (-9.81 * 0.75);
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
    #[derive(Component, Reflect)]
    #[reflect(Component, Hash, PartialEq)]
    #[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
    pub struct PhysicLayer: u32 {
        const GROUND = 1 << 0;
        const CLIFF_EDGE = 1 << 1;
        const PLAYER = 1 << 2;
        const ENEMY = 1 << 3;
        const HOPPER = 1 << 4;
        const CLIMBER = 1 << 5;
        const LURKER = 1 << 6;
        const DIVER = 1 << 7;
        const GIANT = 1 << 8;
        const BEHEMOTH = 1 << 9;
        const PLAYER_PROJ = 1 << 10;
        const ENEMY_PROJ = 1 << 11;
        const EXPLOSION = 1 << 12;
        const DEBUG = 1 << 13;

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
    active_events: ActiveEvents,
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
            active_events: ActiveEvents::COLLISION_EVENTS,
            friction: Default::default(),
            restitution: Default::default(),
            mass: ColliderMassProperties::Density(5000.0),
            velocity: Default::default(),
        }
    }
}
