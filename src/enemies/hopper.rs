use crate::enemies::enemy_projectile::ProjectileSpawn;
use crate::enemies::{Enemy, Explosion, ExplosionBundle, Facing, Hop};
use crate::loading::TextureAssets;
use crate::{DynamicActorBundle, GameState, PhysicsLayers};
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

use super::HopBundle;

const COLLIDER_SHAPE: Vec2 = Vec2::new(2.0, 2.0);

#[derive(Component, Default)]
pub(crate) struct HopperSpawn;

#[derive(Component, Default)]
struct Hopper;

#[derive(Bundle, Default)]
struct HopperBundle {
    sprite_bundle: SpriteSheetBundle,
    dynamic_actor_bundle: DynamicActorBundle,
    enemy: Enemy,
    hopper: Hopper,
    hop: HopBundle,
    external_force: ExternalForce,
}

pub struct HopperPlugin;

impl Plugin for HopperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn, shoot, health, animate).run_if(in_state(GameState::Playing)),
        );
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

        let height = rand::thread_rng().gen_range(5f32..10f32);

        commands.spawn(HopperBundle {
            sprite_bundle: SpriteSheetBundle {
                atlas: TextureAtlas {
                    layout: texture_assets.hopper_layout.clone(),
                    index: 0,
                },
                sprite: Sprite {
                    flip_x: facing.into(),
                    custom_size: Some(COLLIDER_SHAPE),
                    ..default()
                },
                texture: texture_assets.hopper.clone(),
                transform: Transform::from_translation(Vec3::new(16.0 * -facing_mul, height, 0.0)),
                ..Default::default()
            },
            dynamic_actor_bundle: DynamicActorBundle {
                collider: Collider::cuboid(COLLIDER_SHAPE.x / 2.0, COLLIDER_SHAPE.y / 2.0),
                collision_layers: CollisionLayers::new(
                    [PhysicsLayers::Enemy, PhysicsLayers::Hopper],
                    [
                        PhysicsLayers::Ground,
                        PhysicsLayers::Hopper,
                        PhysicsLayers::Player,
                        PhysicsLayers::PlayerProj,
                        PhysicsLayers::Explosion,
                    ],
                ),
                friction: Friction::new(2.0),
                restitution: Restitution::new(0.2),
                ..Default::default()
            },
            enemy: Enemy { health: 1, facing },
            hop: HopBundle {
                hop: Hop {
                    grounded: false,
                    power,
                },
                ray: RayCaster::new(
                    Vec2 {
                        x: 0.0,
                        y: -(COLLIDER_SHAPE.y / 2.0),
                    },
                    Vec2::NEG_Y,
                )
                .with_max_time_of_impact(0.1)
                .with_query_filter(SpatialQueryFilter::new().with_masks_from_bits(
                    PhysicsLayers::Ground.to_bits() | PhysicsLayers::Hopper.to_bits(),
                )),
            },
            ..Default::default()
        });
    }
}

fn shoot(mut commands: Commands, query: Query<&Transform, With<Hopper>>) {
    for t in query.iter() {
        if rand::thread_rng().gen_range(0.0..1.0) > 0.999 && t.translation.x.abs() < 15.0 {
            commands.spawn(ProjectileSpawn {
                pos: t.translation.truncate(),
            });
        }
    }
}

fn animate(mut query: Query<(&mut TextureAtlas, &LinearVelocity)>) {
    for (mut texture, velocity) in query.iter_mut() {
        if velocity.y > 0.2 {
            texture.index = 0;
        } else if velocity.y < -0.2 {
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

            // Spawn Explosion
            commands.spawn(ExplosionBundle {
                sprite_bundle: SpriteSheetBundle {
                    atlas: TextureAtlas {
                        layout: texture_assets.explosion_layout.clone(),
                        index: 0,
                    },
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(
                            enemy.health.abs() as f32 * 2.0,
                            enemy.health.abs() as f32 * 2.0,
                        )),
                        ..Default::default()
                    },
                    texture: texture_assets.explosion.clone(),
                    transform: Transform::from_translation(trans.translation),
                    ..Default::default()
                },
                collider: Collider::ball(enemy.health.abs() as f32),
                explosion: Explosion {
                    power: enemy.health.abs(),
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                },
                ..Default::default()
            });
        }
    }
}
