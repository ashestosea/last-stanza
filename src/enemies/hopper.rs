use crate::enemies::{Enemy, Facing, Hop, SpawnProjectile, ExplosionBundle, Explosion};
use crate::loading::TextureAssets;
use crate::{DynamicActorBundle, GameState, PhysicsLayers};
use bevy::prelude::*;
use heron::prelude::*;
use rand::Rng;

const HOPPER_SHAPE: Vec2 = Vec2::new(2., 2.);

#[derive(Component, Default)]
pub(crate) struct HopperSpawn;

#[derive(Component, Default)]
struct Hopper;

#[derive(Bundle, Default)]
struct HopperBundle {
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    #[bundle]
    dynamic_actor_bundle: DynamicActorBundle,
    rotation_constraints: RotationConstraints,
    enemy: Enemy,
    hopper: Hopper,
    hop: Hop,
}

pub struct HopperPlugin;

impl Plugin for HopperPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(spawn))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(shoot))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(health))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(animate));
    }
}

fn spawn(
    query: Query<(Entity, &HopperSpawn)>,
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
) {
    for (entity, _spawn) in query.iter() {
        commands.entity(entity).despawn();

        let facing = if rand::thread_rng().gen_bool(0.5) {
            Facing::Left
        } else {
            Facing::Right
        };
        let facing_mul: f32 = facing.into();

        let power = Vec2::new(
            rand::thread_rng().gen_range(1.0..2.0) * facing_mul,
            rand::thread_rng().gen_range(15.0..15.01),
        );

        commands.spawn().insert_bundle(HopperBundle {
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: texture_assets.hopper.clone(),
                transform: Transform::from_translation(Vec3::new(24. * -facing_mul, 6., 0.)),
                sprite: TextureAtlasSprite {
                    flip_x: facing.into(),
                    custom_size: Some(HOPPER_SHAPE),
                    ..default()
                },
                ..Default::default()
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
                        PhysicsLayers::PlayerProj,
                    ]),
                ..Default::default()
            },
            rotation_constraints: RotationConstraints::lock(),
            enemy: Enemy { health: 1, facing },
            hop: Hop {
                grounded: false,
                power,
            },
            ..Default::default()
        });
    }
}

fn shoot(mut commands: Commands, query: Query<&Transform, With<Hopper>>) {
    for t in query.iter() {
        if rand::thread_rng().gen_range(0.0..1.0) > 0.9999 {
            commands
                .spawn()
                .insert(SpawnProjectile { pos: t.translation });
        }
    }
}

fn animate(mut query: Query<(&mut TextureAtlasSprite, &Velocity)>) {
    for (mut texture, velocity) in query.iter_mut() {
        if velocity.linear.y > 0.2 {
            texture.index = 0;
        } else if velocity.linear.y < -0.2 {
            texture.index = 2;
        } else {
            texture.index = 1;
        }
    }
}

fn health(
    mut commands: Commands,
    query: Query<(Entity, &Enemy, &Transform), (With<Hopper>, Changed<Enemy>)>,
    texture_assets: Res<TextureAssets>,
) {
    for (entity, enemy, trans) in query.iter() {
        if enemy.health <= 0 {
            let _ = &commands.entity(entity).despawn();
                println!("enemy ded");

            // Spawn Explosion
            commands.spawn().insert_bundle(ExplosionBundle {
                sprite_bundle: SpriteSheetBundle {
                    texture_atlas: texture_assets.explosion.clone(),
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::new(
                            enemy.health.abs() as f32 * 2.,
                            enemy.health.abs() as f32 * 2.,
                        )),
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
                    timer: Timer::from_seconds(0.5, false),
                },
                ..Default::default()
            });
        }
    }
}
