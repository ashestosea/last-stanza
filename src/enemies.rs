use crate::climber::{Climber, ClimberPlugin};
use crate::giant::Giant;
use crate::hopper::Hopper;
use crate::loading::TextureAssets;
use crate::player::PlayerProjectile;
use crate::{GameState, PhysicsLayers};
use bevy::prelude::*;
use heron::prelude::*;
use rand::Rng;

#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) enum Facing {
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

#[derive(Component)]
pub(crate) struct Enemy {
    pub health: i32,
    pub facing: Facing,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: Default::default(),
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
    sprite_bundle: SpriteBundle,
    rigidbody: RigidBody,
    collision_shape: CollisionShape,
    collision_layers: CollisionLayers,
    explosion: Explosion,
}

impl Default for ExplosionBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: Default::default(),
            rigidbody: RigidBody::Sensor,
            collision_shape: Default::default(),
            collision_layers: CollisionLayers::new(PhysicsLayers::PProj, PhysicsLayers::Enemy),
            explosion: Default::default(),
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

struct EnemySpawnChances {
    hopper: f32,
    climber: f32,
    sneaker: f32,
    diver: f32,
    giant: f32,
    behemoth: f32,
}

pub struct EnemiesPlugin;

impl Default for EnemySpawnChances {
    fn default() -> Self {
        Self {
            hopper: 0.0,
            climber: 0.0,
            sneaker: 0.0,
            diver: 0.0,
            giant: 0.0,
            behemoth: 0.0,
        }
    }
}

impl EnemySpawnChances {
    fn none(&self) -> f32 {
        1. - self.hopper - self.climber - self.sneaker - self.diver - self.giant - self.behemoth
    }
}

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_enemy_spawns))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(enemy_spawner))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(hop))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(hop_grounding))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(enemy_hits))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(enemy_health))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(explosion_cleanup))
            .add_plugin(ClimberPlugin);
    }
}

fn setup_enemy_spawns(mut commands: Commands) {
    let spawn_chances = EnemySpawnChances {
        hopper: 0.15,
        climber: 0.1,
        giant: 0.05,
        ..Default::default()
    };

    commands.insert_resource(spawn_chances);
    commands.insert_resource(SpawnTimer {
        timer: Timer::from_seconds(1., true),
    });
}

fn enemy_spawner(
    time: Res<Time>,
    spawn_chances: Res<EnemySpawnChances>,
    commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    spawn_timer.timer.tick(time.delta());

    if !spawn_timer.timer.finished() {
        return;
    }

    let (facing, start_mul) = if rand::thread_rng().gen_bool(0.5) {
        (Facing::Left, -1.)
    } else {
        (Facing::Right, 1.)
    };

    let rng = rand::thread_rng().gen_range(0f32..1f32);
    let mut total_chance = spawn_chances.none();

    let start_x = 24. * start_mul;

    // Nothing
    if rng < total_chance {
        return;
    }

    // Hopper
    total_chance += spawn_chances.hopper;
    if rng < total_chance {
        Hopper::spawn(commands, facing, start_x);
        return;
    }

    // Climber
    total_chance += spawn_chances.climber;
    if rng < total_chance {
        Climber::spawn(commands, facing, start_x);
        return;
    }

    // Giant
    total_chance += spawn_chances.giant;
    if rng < total_chance {
        Giant::spawn(commands, facing, start_x);
        return;
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
            if c.collision_layers().contains_group(PhysicsLayers::PProj) {
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

fn enemy_health(
    mut commands: Commands,
    query: Query<(Entity, &Enemy, &Transform), Changed<Enemy>>,
    texture_assets: Res<TextureAssets>,
) {
    for (entity, enemy, trans) in query.iter() {
        if enemy.health <= 0 {
            let _ = &commands.entity(entity).despawn();

            // Spawn Explosion
            commands.spawn().insert_bundle(ExplosionBundle {
                sprite_bundle: SpriteBundle {
                    texture: texture_assets.circle.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(
                            enemy.health.abs() as f32 * 2.,
                            enemy.health.abs() as f32 * 2.,
                        )),
                        color: Color::YELLOW,
                        ..Default::default()
                    },
                    transform: Transform::from_translation(trans.translation),
                    ..Default::default()
                },
                collision_shape: CollisionShape::Sphere {
                    radius: enemy.health.abs() as f32,
                },
                explosion: Explosion {
                    power: enemy.health.abs(),
                    timer: Timer::from_seconds(0.25, false),
                },
                ..Default::default()
            });
        }
    }
}

fn explosion_cleanup(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Explosion)>,
    mut commands: Commands,
) {
    for (entity, mut explosion) in query.iter_mut() {
        explosion.timer.tick(time.delta());

        if explosion.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
