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
    enemy: Enemy,
    hopper: Hopper,
    hop: Hop,
}

#[derive(Component)]
struct Hop {
    grounded: bool,
    power: Vec2,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(enemy_spawner))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(hop))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(hopper_grounding))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(enemy_destruction))
    }
}

fn enemy_spawner(mut commands: Commands, time: Res<Time>, mut duration: Local<Duration>) {
    duration.add_assign(time.delta());

    if duration.as_millis() > 1000 {
        *duration = Duration::ZERO;
        if rand::thread_rng().gen_range(0f32..1f32) < 0.85 {
            return;
        }

        let enemy_shape = Vec2::new(1., 2.);
        commands
            .spawn()
            .insert(Enemy)
            .insert_bundle(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(-24., 6., 0.)),
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(enemy_shape),
                    ..default()
                },
                ..default()
            })
            .insert(Hop {
                grounded: true,
                power: Vec2::new(
                    rand::thread_rng().gen_range(0.6..0.8),
                    rand::thread_rng().gen_range(8.0..8.01),
                ),
            })
            .insert_bundle(DynamicActorBundle {
                material: PhysicMaterial {
                    density: 1.,
                    friction: 2.,
                    restitution: 0.01,
                },
                shape: CollisionShape::Cuboid {
                    half_extends: enemy_shape.extend(0.) / 2.,
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
            })
            .insert(RotationConstraints::lock());
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

fn enemy_destruction(
    mut commands: Commands,
    query: Query<(Entity, &Collisions), With<Enemy>>
) {
    for (entity, collisions) in query.iter() {
        for c in collisions.collision_data() {
            if c.collision_layers().contains_group(PhysicsLayers::PProj) {
                commands.entity(entity).despawn();
            }
        }
    }
}

