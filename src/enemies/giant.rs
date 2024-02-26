use crate::enemies::{Enemy, Explosion, ExplosionBundle, Facing, Hop};
use crate::loading::TextureAssets;
use crate::player::PlayerProjectile;
use crate::{DynamicActorBundle, GameState, PhysicsLayers};
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

use super::HopBundle;

const COLLIDER_SHAPE: Vec2 = Vec2::new(3.0, 6.0);

#[derive(Component, Default)]
pub(crate) struct GiantSpawn;

#[derive(Component, Default)]
pub struct Giant;

#[derive(Bundle, Default)]
struct GiantBundle {
    sprite_bundle: SpriteBundle,
    dynamic_actor_bundle: DynamicActorBundle,
    enemy: Enemy,
    giant: Giant,
    hop: HopBundle,
    external_force: ExternalForce,
}

pub struct GiantPlugin;

impl Plugin for GiantPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn, hit, health).run_if(in_state(GameState::Playing)),
        );
    }
}

fn spawn(query: Query<(Entity, &GiantSpawn)>, mut commands: Commands) {
    for (entity, _spawn) in query.iter() {
        commands.entity(entity).despawn();

        let facing = if rand::thread_rng().gen_bool(0.5) {
            Facing::Left
        } else {
            Facing::Right
        };
        let facing_mul: f32 = facing.into();

        let power = Vec2::new(
            rand::thread_rng().gen_range(2.0..3.5) * facing_mul,
            rand::thread_rng().gen_range(50.5..55.0),
        );

        commands.spawn(GiantBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(COLLIDER_SHAPE),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(16.0 * -facing_mul, 6.0, 0.0)),
                ..Default::default()
            },
            dynamic_actor_bundle: DynamicActorBundle {
                collider: Collider::rectangle(COLLIDER_SHAPE.x, COLLIDER_SHAPE.y),
                collision_layers: CollisionLayers::new(
                    [PhysicsLayers::Enemy, PhysicsLayers::Giant],
                    [
                        PhysicsLayers::Ground,
                        PhysicsLayers::Player,
                        PhysicsLayers::PlayerProj,
                    ],
                ),
                friction: Friction::new(2.0),
                restitution: Restitution::new(0.2),
                ..Default::default()
            },
            enemy: Enemy { health: 20, facing },
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
                    Direction2d::NEG_Y,
                )
                .with_max_time_of_impact(0.1)
                .with_query_filter(SpatialQueryFilter::from_mask(
                    PhysicsLayers::Ground.to_bits() | PhysicsLayers::Hopper.to_bits(),
                )),
            },
            ..Default::default()
        });
    }
}

fn hit(
    proj_query: Query<(Entity, &PlayerProjectile)>,
    mut query: Query<(&mut ExternalForce, &Enemy, &CollidingEntities), With<Giant>>,
) {
    for (mut force, enemy, colliding_entities) in query.iter_mut() {
        for coll_entity in colliding_entities.iter() {
            for (proj_entity, projectile) in proj_query.iter() {
                if coll_entity == &proj_entity {
                    force.set_force(
                        Vec2::X * -f32::from(enemy.facing) * (projectile.size as f32) * 7.5,
                    );
                }
            }
        }
    }
}

fn health(
    mut commands: Commands,
    query: Query<(Entity, &Enemy, &Transform), (With<Giant>, Changed<Enemy>)>,
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
                collider: Collider::circle(enemy.health.abs() as f32),
                explosion: Explosion {
                    power: enemy.health.abs(),
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                },
                ..Default::default()
            });
        }
    }
}
