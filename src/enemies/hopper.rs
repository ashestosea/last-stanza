use crate::enemies::enemy_projectile::ProjectileSpawn;
use crate::enemies::{Enemy, Explosion, ExplosionBundle, Facing, Hop};
use crate::loading::TextureAssets;
use crate::{DynamicActorBundle, GameState, PhysicLayer};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

const HOPPER_SHAPE: Vec2 = Vec2::new(2.0, 2.0);

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
    hop: Hop,
}

pub struct HopperPlugin;

impl Plugin for HopperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((spawn, shoot, health, animate).in_set(OnUpdate(GameState::Playing)));
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

        commands.spawn(HopperBundle {
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: texture_assets.hopper.clone(),
                transform: Transform::from_translation(Vec3::new(24.0 * -facing_mul, 6.0, 0.0)),
                sprite: TextureAtlasSprite {
                    flip_x: facing.into(),
                    custom_size: Some(HOPPER_SHAPE),
                    ..default()
                },
                ..Default::default()
            },
            dynamic_actor_bundle: DynamicActorBundle {
                collider: Collider::cuboid(HOPPER_SHAPE.x / 2.0, HOPPER_SHAPE.y / 2.0),
                collision_groups: CollisionGroups::new(
                    (PhysicLayer::ENEMY | PhysicLayer::HOPPER).into(),
                    (PhysicLayer::GROUND | PhysicLayer::HOPPER | PhysicLayer::PLAYER_PROJ).into(),
                ),
                friction: Friction::coefficient(2.0),
                restitution: Restitution::coefficient(0.2),
                ..Default::default()
            },
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
        if rand::thread_rng().gen_range(0.0..1.0) > 0.99 {
            commands.spawn(ProjectileSpawn {
                pos: t.translation.truncate(),
            });
        }
    }
}

fn animate(mut query: Query<(&mut TextureAtlasSprite, &Velocity)>) {
    for (mut texture, velocity) in query.iter_mut() {
        if velocity.linvel.y > 0.2 {
            texture.index = 0;
        } else if velocity.linvel.y < -0.2 {
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
            commands.spawn(ExplosionBundle {
                sprite_bundle: SpriteSheetBundle {
                    texture_atlas: texture_assets.explosion.clone(),
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::new(
                            enemy.health.abs() as f32 * 2.0,
                            enemy.health.abs() as f32 * 2.0,
                        )),
                        ..Default::default()
                    },
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
