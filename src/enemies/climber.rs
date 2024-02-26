use crate::{
    enemies::{Enemy, Explosion, ExplosionBundle, Facing},
    loading::TextureAssets,
    DynamicActorBundle, GameState, PhysicsLayers,
};
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

const CLIMBER_SHAPE: Vec2 = Vec2::new(1.0, 2.0);

#[derive(Component, Default)]
pub(crate) struct ClimberSpawn;

#[derive(Component)]
pub(crate) struct Climber;

#[derive(Bundle)]
struct ClimberBundle {
    sprite_bundle: SpriteBundle,
    dynamic_actor_bundle: DynamicActorBundle,
    enemy: Enemy,
    climber: Climber,
}

pub struct ClimberPlugin;

impl Plugin for ClimberPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (climb, spawn, health).run_if(in_state(GameState::Playing)),
        );
        // .add_systems(
        //     Update,
        //     debug_colliders.run_if(in_state(GameState::Playing)),
        // );
    }
}

#[allow(dead_code)]
fn debug_colliders(query: Query<&Position, With<Climber>>, mut gizmos: Gizmos) {
    for p in query.iter() {
        gizmos.rect_2d(p.0, 0.0, CLIMBER_SHAPE, Color::PINK);
    }
}

fn spawn(query: Query<(Entity, &ClimberSpawn)>, mut commands: Commands) {
    for (entity, _spawn) in query.iter() {
        commands.entity(entity).despawn();

        let facing = if rand::thread_rng().gen_bool(0.5) {
            Facing::Left
        } else {
            Facing::Right
        };
        let facing_mul: f32 = facing.into();

        commands.spawn(ClimberBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::MIDNIGHT_BLUE,
                    custom_size: Some(CLIMBER_SHAPE),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(16.0 * -facing_mul, 1.0, 0.0)),
                ..Default::default()
            },
            dynamic_actor_bundle: DynamicActorBundle {
                rigidbody: RigidBody::Dynamic,
                collider: Collider::rectangle(CLIMBER_SHAPE.x, CLIMBER_SHAPE.y),
                collision_layers: CollisionLayers::new(
                    [PhysicsLayers::Enemy, PhysicsLayers::Climber],
                    [
                        PhysicsLayers::Ground,
                        PhysicsLayers::CliffEdge,
                        PhysicsLayers::Player,
                        PhysicsLayers::PlayerProj,
                        PhysicsLayers::Explosion,
                    ],
                ),
                friction: Friction::ZERO,
                restitution: Restitution::PERFECTLY_INELASTIC,
                mass: Mass(1.0),
                velocity: LinearVelocity(Vec2::new(facing_mul * 2.0, 0.0)),
                ..Default::default()
            },
            enemy: Enemy { health: 1, facing },
            climber: Climber,
        });
    }
}

fn climb(
    mut query: Query<(&mut LinearVelocity, &CollidingEntities, &Enemy), With<Climber>>,
    sensor_query: Query<(Entity, &CollisionLayers), With<Sensor>>,
) {
    for (mut velocity, colliding_entities, enemy) in query.iter_mut() {
        for e in colliding_entities.iter() {
            for (entity, collision_layers) in sensor_query.iter() {
                if e == &entity
                    && collision_layers
                        .memberships
                        .has_all(PhysicsLayers::CliffEdge)
                {
                    let mul: f32 = enemy.facing.into();
                    velocity.x = 1.0 * mul;
                    velocity.y = 9.0;
                    return;
                }
            }
        }
    }
}

fn health(
    mut commands: Commands,
    query: Query<(Entity, &Enemy, &Transform), (With<Climber>, Changed<Enemy>)>,
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
