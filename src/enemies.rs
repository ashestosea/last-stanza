mod climber;
mod giant;
mod hopper;

pub use crate::enemies::giant::Giant;
use crate::events::EnemySpawnsChanged;
use crate::loading::TextureAssets;
use crate::player::{self, PlayerProjectile};
use crate::{GameState, PhysicsLayers};
use benimator::FrameRate;
use bevy::prelude::*;
use heron::prelude::*;
use rand::Rng;

use self::climber::{ClimberPlugin, ClimberSpawn};
use self::giant::{GiantPlugin, GiantSpawn};
use self::hopper::{HopperPlugin, HopperSpawn};

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
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    rigidbody: RigidBody,
    collision_shape: CollisionShape,
    collision_layers: CollisionLayers,
    animation: ExplosionAnimation,
    animation_state: ExplosionAnimationState,
    explosion: Explosion,
}

impl Default for ExplosionBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: Default::default(),
            rigidbody: RigidBody::Sensor,
            collision_shape: Default::default(),
            collision_layers: CollisionLayers::new(PhysicsLayers::PlayerProj, PhysicsLayers::Enemy),
            animation: ExplosionAnimation(benimator::Animation::from_indices(
                0..=8,
                FrameRate::from_fps(16.),
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
struct SpawnProjectile {
    pos: Vec3,
}

#[derive(Default, Component)]
struct EnemyProjectile;

#[derive(Bundle)]
struct EnemyProjectileBundle {
    #[bundle]
    sprite: SpriteBundle,
    rigidbody: RigidBody,
    collision_shape: CollisionShape,
    collision_layers: CollisionLayers,
    velocity: Velocity,
    projectile: EnemyProjectile,
}

// #[derive(Component, Deref)]
// struct ProjectileAnimation(benimator::Animation);

// // Create the player component
// #[derive(Default, Component, Deref, DerefMut)]
// struct ProjectileAnimationState(benimator::State);

impl Default for EnemyProjectileBundle {
    fn default() -> Self {
        Self {
            sprite: Default::default(),
            rigidbody: RigidBody::KinematicVelocityBased,
            collision_shape: CollisionShape::Sphere { radius: 0.3 },
            collision_layers: CollisionLayers::new(PhysicsLayers::EnemyProj, PhysicsLayers::Player),
            velocity: Default::default(),
            projectile: Default::default(),
        }
    }
}

#[derive(Component)]
struct Sneaker;

#[derive(Component)]
struct Diver;

#[derive(Component)]
struct Behemoth;

#[derive(Component, Default)]
pub(crate) struct Hop {
    pub grounded: bool,
    pub power: Vec2,
}

struct SpawnTimer {
    timer: Timer,
}

#[derive(serde::Deserialize, Clone, Default)]
pub struct SpawnRates {
    pub hopper: Option<f32>,
    pub climber: Option<f32>,
    pub sneaker: Option<f32>,
    pub diver: Option<f32>,
    pub giant: Option<f32>,
    pub behemoth: Option<f32>,
}

impl SpawnRates {
    fn none(&self) -> f32 {
        1. - self.hopper.unwrap_or_default()
            - self.climber.unwrap_or_default()
            - self.sneaker.unwrap_or_default()
            - self.diver.unwrap_or_default()
            - self.giant.unwrap_or_default()
            - self.behemoth.unwrap_or_default()
    }
}

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(update_enemy_spawns),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(enemy_spawner))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(projectile_spawner))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(hop))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(hop_grounding))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(enemy_hits))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(explosion_cleanup))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(explosion_animate))
        .init_resource::<SpawnRates>()
        .insert_resource(SpawnTimer {
            timer: Timer::from_seconds(1., true),
        })
        .add_plugin(HopperPlugin)
        .add_plugin(ClimberPlugin)
        .add_plugin(GiantPlugin);
    }
}

fn update_enemy_spawns(
    mut spawn_change_ev: EventReader<EnemySpawnsChanged>,
    mut spawn_rates: ResMut<SpawnRates>,
) {
    for e in spawn_change_ev.iter() {
        if let Some(val) = e.hopper {
            spawn_rates.hopper = Some(val);
        }
        if let Some(val) = e.climber {
            spawn_rates.climber = Some(val);
        }
        if let Some(val) = e.sneaker {
            spawn_rates.sneaker = Some(val);
        }
        if let Some(val) = e.diver {
            spawn_rates.diver = Some(val);
        }
        if let Some(val) = e.giant {
            spawn_rates.giant = Some(val);
        }
        if let Some(val) = e.behemoth {
            spawn_rates.behemoth = Some(val);
        }
    }
}

fn enemy_spawner(
    time: Res<Time>,
    spawn_chances: Res<SpawnRates>,
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    spawn_timer.timer.tick(time.delta());

    if !spawn_timer.timer.finished() {
        return;
    }

    let rng = rand::thread_rng().gen_range(0f32..1f32);
    let mut total_chance = spawn_chances.none();

    // Nothing
    if rng < total_chance {
        return;
    }

    // Hopper
    total_chance += spawn_chances.hopper.unwrap_or_default();
    if rng < total_chance {
        // Hopper::spawn(commands, facing, start_x);
        commands.spawn().insert(HopperSpawn);
        return;
    }

    // Climber
    total_chance += spawn_chances.climber.unwrap_or_default();
    if rng < total_chance {
        commands.spawn().insert(ClimberSpawn);
        return;
    }

    // Giant
    total_chance += spawn_chances.giant.unwrap_or_default();
    if rng < total_chance {
        commands.spawn().insert(GiantSpawn);
        #[allow(clippy::needless_return)]
        return;
    }
}

fn projectile_spawner(
    projectile_query: Query<(Entity, &SpawnProjectile)>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
) {
    for (entity, spawn) in projectile_query.iter() {
        // Spawn projectile
        println!("Spawn Projectile");
        let vec = (player::PLAYER_CENTER.extend(0.) - spawn.pos).normalize();

        commands.spawn().insert_bundle(EnemyProjectileBundle {
            sprite: SpriteBundle {
                texture: textures.circle.clone(),
                sprite: Sprite {
                    color: Color::PINK,
                    custom_size: Some(Vec2::new(0.3, 0.3)),
                    ..Default::default()
                },
                transform: Transform::from_translation(spawn.pos),
                ..Default::default()
            },
            velocity: Velocity::from_linear(vec),
            ..Default::default()
        });

        commands.entity(entity).despawn();
    }
}

fn hop(mut query: Query<(&Enemy, &mut Velocity, &Collisions, &Hop)>) {
    for (enemy, mut vel, collisions, hop) in query.iter_mut() {
        if hop.grounded {
            vel.linear = hop.power.extend(0.);
        } else if collisions.is_empty() {
            // Nudge Hopping actor if it's stalled out
            if vel.linear.x.abs() < 0.1 && vel.linear.y.abs() < 0.1 {
                let mul: f32 = enemy.facing.into();
                vel.linear = Vec3::X * 2. * mul;
            }
        }
    }
}

fn hop_grounding(mut query: Query<(&mut Hop, &Collisions)>) {
    for (mut hop, collisions) in query.iter_mut() {
        hop.grounded = false;

        for c in collisions.collision_data() {
            if c.collision_layers().contains_group(PhysicsLayers::Ground) {
                for n in c.normals() {
                    if *n == Vec3::Y {
                        hop.grounded = true;
                        return;
                    }
                }
            }
        }
    }
}

fn enemy_hits(
    proj_query: Query<(Entity, &PlayerProjectile)>,
    explosion_query: Query<(Entity, &Explosion)>,
    mut query: Query<(&mut Enemy, &Collisions)>,
) {
    for (mut enemy, collisions) in query.iter_mut() {
        for c in collisions.collision_data() {
            if c.collision_layers()
                .contains_group(PhysicsLayers::PlayerProj)
            {
                for (entity, projectile) in proj_query.iter() {
                    if c.collision_shape_entity() == entity {
                        enemy.health -= projectile.size;
                        break;
                    }
                }

                for (entity, explosion) in explosion_query.iter() {
                    if c.collision_shape_entity() == entity {
                        enemy.health -= explosion.power;
                        break;
                    }
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
    mut query: Query<(Entity, &mut Explosion), With<CollisionShape>>,
    mut commands: Commands,
) {
    for (entity, mut explosion) in query.iter_mut() {
        explosion.timer.tick(time.delta());
        commands
            .entity(entity)
            .insert(CollisionShape::Sphere { radius: 0. });

        if explosion.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
