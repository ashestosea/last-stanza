use crate::{DynamicActorBundle, GameState, PhysicsLayers};
use bevy::prelude::*;
use heron::prelude::*;
use rand::Rng;
use std::{ops::AddAssign, time::Duration};

pub struct EnemiesPlugin;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Hopper;

#[derive(Component)]
struct Climber;

#[derive(Component)]
struct Sneaker;

#[derive(Component)]
struct Diver;

#[derive(Component)]
struct Giant;

#[derive(Component)]
struct Behemoth;

#[derive(Bundle)]
struct HopperBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    #[bundle]
    dynamic_actor_bundle: DynamicActorBundle,
    rotation_constraints: RotationConstraints,
    enemy: Enemy,
    hopper: Hopper,
    hop: Hop,
}

const HOPPER_SHAPE: Vec2 = Vec2::new(1., 2.);

impl Default for HopperBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: Default::default(),
            enemy: Enemy,
            hopper: Hopper,
            hop: Hop {
                grounded: false,
                power: Vec2::new(
                    rand::thread_rng().gen_range(0.6..0.8),
                    rand::thread_rng().gen_range(6.0..6.01),
                ),
            },
            dynamic_actor_bundle: DynamicActorBundle {
                material: PhysicMaterial {
                    density: 1.,
                    friction: 2.,
                    restitution: 0.2,
                },
                shape: CollisionShape::Cuboid {
                    half_extends: HOPPER_SHAPE.extend(0.) / 2.,
                    border_radius: None,
                },
                layers: CollisionLayers::none()
                    .with_groups(&[PhysicsLayers::Enemy, PhysicsLayers::Hopper])
                    .with_masks(&[
                        PhysicsLayers::Ground,
                        PhysicsLayers::Hopper,
                        PhysicsLayers::PProj,
                    ]),
                ..Default::default()
            },
            rotation_constraints: RotationConstraints::lock(),
        }
    }
}

#[derive(Component)]
struct Hop {
    grounded: bool,
    power: Vec2,
}

struct SpawnTimer {
    timer: Timer
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
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(enemy_destruction))
    }
}

fn setup_enemy_spawns(mut commands: Commands) {
    let mut spawn_chances = EnemySpawnChances::default();
    spawn_chances.hopper = 0.1;

    commands.insert_resource(spawn_chances);
    commands.insert_resource(SpawnTimer {
        timer: Timer::from_seconds(1., true)
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

    let rng = rand::thread_rng().gen_range(0f32..1f32);
    let mut chance = spawn_chances.none();
    if rng < chance {
        return;
    }
    chance += spawn_chances.hopper;
    if rng < chance {
        commands.spawn().insert_bundle(HopperBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(-24., 6., 0.)),
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
}

fn hop(mut query: Query<(&mut Impulse, &Hop)>) {
    for (mut impulse, hop) in query.iter_mut() {
        if hop.grounded {
            impulse.linear = hop.power.extend(0.);
        }
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

fn enemy_destruction(mut commands: Commands, query: Query<(Entity, &Collisions), With<Enemy>>) {
    for (entity, collisions) in query.iter() {
        for c in collisions.collision_data() {
            if c.collision_layers().contains_group(PhysicsLayers::PProj) {
                commands.entity(entity).despawn();
            }
        }
    }
}

