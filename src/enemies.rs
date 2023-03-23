mod climber;
pub(crate) mod enemy_projectile;
mod giant;
mod hopper;
mod lurker;

use std::time::Duration;

pub use crate::enemies::giant::Giant;
use crate::events::EnemySpawnsChanged;
use crate::player::PlayerProjectile;
use crate::world::Ground;
use crate::{GameState, PhysicLayer};
use benimator::FrameRate;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use self::climber::{ClimberPlugin, ClimberSpawn};
use self::enemy_projectile::ProjectilePlugin;
use self::giant::{GiantPlugin, GiantSpawn};
use self::hopper::{HopperPlugin, HopperSpawn};
use self::lurker::{LurkerPlugin, LurkerSpawn};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Facing {
    Left,
    Right,
}

impl From<Facing> for f32 {
    fn from(val: Facing) -> Self {
        if val == Facing::Left {
            1.
        } else {
            -1.
        }
    }
}

impl From<Facing> for bool {
    fn from(val: Facing) -> Self {
        val != Facing::Left
    }
}

#[derive(Component)]
pub(crate) struct Enemy {
    pub health: i32,
    pub facing: Facing,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: 1,
            facing: Facing::Left,
        }
    }
}

#[derive(Component, Default)]
pub(crate) struct Explosion {
    pub power: i32,
    timer: Timer,
}

#[derive(Bundle)]
struct ExplosionBundle {
    sprite_bundle: SpriteSheetBundle,
    rigidbody: RigidBody,
    collider: Collider,
    collision_groups: CollisionGroups,
    sensor: Sensor,
    animation: ExplosionAnimation,
    animation_state: ExplosionAnimationState,
    explosion: Explosion,
}

impl Default for ExplosionBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: Default::default(),
            rigidbody: RigidBody::KinematicVelocityBased,
            collider: Default::default(),
            collision_groups: CollisionGroups::new(
                PhysicLayer::EXPLOSION.into(),
                PhysicLayer::ENEMY.into(),
            ),
            sensor: Sensor::default(),
            animation: ExplosionAnimation(benimator::Animation::from_indices(
                0..=8,
                FrameRate::from_fps(16.0),
            )),
            animation_state: Default::default(),
            explosion: Default::default(),
        }
    }
}
// Create the animation component
// Note: you may make the animation an asset instead of a component
#[derive(Component, Deref)]
struct ExplosionAnimation(benimator::Animation);

// Create the player component
#[derive(Default, Component, Deref, DerefMut)]
struct ExplosionAnimationState(benimator::State);

#[derive(Component)]
struct Lurker;

#[derive(Component)]
struct Diver;

#[derive(Component)]
struct Behemoth;

#[derive(Component, Default)]
pub(crate) struct Hop {
    pub grounded: bool,
    pub power: Vec2,
}

#[derive(Resource)]
struct SpawnTimer {
    timer: Timer,
}

#[derive(Resource, serde::Deserialize, Clone, Default)]
pub struct SpawnRates {
    pub min_spawn_time: Option<f32>,
    pub max_spawn_time: Option<f32>,
    pub hopper: Option<f32>,
    pub climber: Option<f32>,
    pub lurker: Option<f32>,
    pub diver: Option<f32>,
    pub giant: Option<f32>,
    pub behemoth: Option<f32>,
}

impl SpawnRates {
    pub fn all(&self) -> f32 {
        self.hopper.unwrap_or_default()
            + self.climber.unwrap_or_default()
            + self.lurker.unwrap_or_default()
            + self.diver.unwrap_or_default()
            + self.giant.unwrap_or_default()
            + self.behemoth.unwrap_or_default()
    }
}

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                update_enemy_spawns,
                enemy_spawner,
                hop,
                hop_grounding,
                enemy_hits,
                explosion_cleanup,
                explosion_animate,
            )
                .in_set(OnUpdate(GameState::Playing)),
        )
        .init_resource::<SpawnRates>()
        .insert_resource(SpawnTimer {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        })
        .add_plugin(HopperPlugin)
        .add_plugin(ClimberPlugin)
        .add_plugin(LurkerPlugin)
        .add_plugin(GiantPlugin)
        .add_plugin(ProjectilePlugin);
    }
}

fn update_enemy_spawns(
    mut spawn_change_ev: EventReader<EnemySpawnsChanged>,
    mut spawn_rates: ResMut<SpawnRates>,
) {
    for e in spawn_change_ev.iter() {
        if let Some(val) = e.min_spawn_time {
            spawn_rates.min_spawn_time = Some(val as f32);
        }
        if let Some(val) = e.max_spawn_time {
            spawn_rates.max_spawn_time = Some(val as f32);
        }
        if let Some(val) = e.hopper {
            spawn_rates.hopper = Some(val as f32);
        }
        if let Some(val) = e.climber {
            spawn_rates.climber = Some(val as f32);
        }
        if let Some(val) = e.lurker {
            spawn_rates.lurker = Some(val as f32);
        }
        if let Some(val) = e.diver {
            spawn_rates.diver = Some(val as f32);
        }
        if let Some(val) = e.giant {
            spawn_rates.giant = Some(val as f32);
        }
        if let Some(val) = e.behemoth {
            spawn_rates.behemoth = Some(val as f32);
        }
    }
}

fn enemy_spawner(
    time: Res<Time>,
    spawn_rates: Res<SpawnRates>,
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    spawn_timer.timer.tick(time.delta());

    if !spawn_timer.timer.finished() {
        return;
    }

    let min_spawn_time = spawn_rates.min_spawn_time.unwrap_or_default();
    let max_spawn_time = spawn_rates.max_spawn_time.unwrap_or_default();
    let dur = rand::thread_rng().gen_range(min_spawn_time..max_spawn_time);
    spawn_timer.timer.set_duration(Duration::from_secs_f32(dur));

    let mut rng = rand::thread_rng().gen_range(0f32..1f32);

    // Hopper
    if rng <= spawn_rates.hopper.unwrap_or_default() {
        // Hopper::spawn(commands, facing, start_x);
        commands.spawn(HopperSpawn);
        return;
    }
    rng -= spawn_rates.hopper.unwrap_or_default();

    // Climber
    if rng <= spawn_rates.climber.unwrap_or_default() {
        commands.spawn(ClimberSpawn);
        return;
    }
    rng -= spawn_rates.climber.unwrap_or_default();

    // Lurker
    if rng <= spawn_rates.lurker.unwrap_or_default() {
        commands.spawn(LurkerSpawn);
        return;
    }
    rng -= spawn_rates.lurker.unwrap_or_default();

    // Giant
    if rng <= spawn_rates.giant.unwrap_or_default() {
        commands.spawn(GiantSpawn);
        // return;
    }
    // rng -= spawn_chances.giant.unwrap_or_default();
}

fn hop(
    mut query: Query<(
        &Enemy,
        &Velocity,
        &mut ExternalImpulse,
        &CollidingEntities,
        &Hop,
    )>,
) {
    for (enemy, vel, mut imp, colliding_entities, hop) in query.iter_mut() {
        if hop.grounded {
            imp.impulse = hop.power;
        } else if colliding_entities.is_empty() {
            // Nudge Hopping actor if it's stalled out
            if vel.linvel.x.abs() < 0.1 && vel.linvel.y.abs() < 0.1 {
                let mul: f32 = enemy.facing.into();
                imp.impulse = Vec2::X * 2.0 * mul;
            }
        }
    }
}

fn hop_grounding(
    mut hop_query: Query<(Entity, &mut Hop)>,
    ground_query: Query<Entity, With<Ground>>,
    rapier_context: Res<RapierContext>,
) {
    for (hop_entity, mut hop) in hop_query.iter_mut() {
        hop.grounded = false;

        for ground_entity in ground_query.iter() {
            if let Some(contact) = rapier_context.contact_pair(hop_entity, ground_entity) {
                for manifold in contact.manifolds() {
                    if manifold.normal() == Vec2::Y {
                        hop.grounded = true;
                        break;
                    }
                }
            }
        }
    }
}

fn enemy_hits(
    proj_query: Query<(Entity, &PlayerProjectile)>,
    explosion_query: Query<(Entity, &Explosion)>,
    mut query: Query<(&mut Enemy, &CollidingEntities)>,
) {
    for (mut enemy, colliding_entities) in query.iter_mut() {
        for coll_entity in colliding_entities.iter() {
            for (proj_entity, projectile) in proj_query.iter() {
                if coll_entity == proj_entity {
                    enemy.health -= projectile.size;
                }
            }

            for (ex_entity, explosion) in explosion_query.iter() {
                if coll_entity == ex_entity {
                    enemy.health -= explosion.power;
                }
            }
        }
    }
}

fn explosion_animate(
    time: Res<Time>,
    mut query: Query<(
        &mut ExplosionAnimationState,
        &mut TextureAtlasSprite,
        &ExplosionAnimation,
    )>,
) {
    for (mut player, mut texture, animation) in query.iter_mut() {
        // Update the state
        player.update(animation, time.delta());

        // Update the texture atlas
        texture.index = player.frame_index();
    }
}

fn explosion_cleanup(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Explosion), With<Collider>>,
    mut commands: Commands,
) {
    for (entity, mut explosion) in query.iter_mut() {
        explosion.timer.tick(time.delta());
        commands.entity(entity).insert(Collider::ball(0.0));

        if explosion.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
