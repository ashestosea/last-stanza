use crate::climber::climb;
use crate::hopper::{HopperBundle, HOPPER_SHAPE};
use crate::player::PlayerProjectile;
use crate::{GameState, PhysicsLayers};
use bevy::prelude::*;
use heron::prelude::*;
use rand::Rng;

pub struct EnemiesPlugin;

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

#[derive(Component)]
struct Sneaker;

#[derive(Component)]
struct Diver;

#[derive(Component)]
struct Giant;

#[derive(Component)]
struct Behemoth;

#[derive(Component, Default)]
pub(crate) struct Hop {
    grounded: bool,
    power: Vec2,
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

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_enemy_spawns))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(enemy_spawner))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(hop))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(hopper_grounding))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(climb))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(enemy_hits))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(enemy_health));
    }
}

fn setup_enemy_spawns(mut commands: Commands) {
    let spawn_chances = EnemySpawnChances {
        hopper: 0.1,
        climber: 0.1,
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
    mut commands: Commands,
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
        let power = Vec2::new(
            rand::thread_rng().gen_range(1.0..2.0) * -start_mul,
            rand::thread_rng().gen_range(15.0..15.01),
        );

        commands.spawn().insert_bundle(HopperBundle {
            enemy: Enemy { health: 1, facing },
            hop: Hop {
                grounded: false,
                power,
            },
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(start_x, 6., 0.)),
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(HOPPER_SHAPE),
                    ..default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
    }

    // Climber
    total_chance += spawn_chances.climber;
    if rng < total_chance {
        crate::climber::spawn_climber(commands, facing, start_x);
    }
}

fn hopper_grounding(mut query: Query<(&mut Hop, &Collisions)>) {
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

fn hop(mut query: Query<(&Enemy, &mut Velocity, &Collisions, &Hop)>) {
    for (enemy, mut vel, collisions, hop) in query.iter_mut() {
        if hop.grounded {
            vel.linear = hop.power.extend(0.);
        } else if collisions.is_empty() {
            // Nudge Hopper if it's stalled out
            if vel.linear.x.abs() < 0.1 && vel.linear.y.abs() < 0.1 {
                let mul: f32 = enemy.facing.into();
                vel.linear = Vec3::X * 2. * mul;
            }
        }
    }
}

fn enemy_hits(proj_query: Query<&PlayerProjectile>, mut query: Query<(&mut Enemy, &Collisions)>) {
    if proj_query.is_empty() {
        return;
    }
    let projectile = proj_query.single();

    for (mut enemy, collisions) in query.iter_mut() {
        for c in collisions.collision_data() {
            if c.collision_layers().contains_group(PhysicsLayers::PProj) {
                enemy.health -= projectile.size;
            }
        }
    }
}

fn enemy_health(mut commands: Commands, query: Query<(Entity, &Enemy), Changed<Enemy>>) {
    for (entity, enemy) in query.iter() {
        if enemy.health <= 0 {
            commands.entity(entity).despawn();
        }
    }
}
