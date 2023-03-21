use crate::{
    enemies::{Enemy, Explosion, ExplosionBundle, Facing},
    loading::TextureAssets,
    DynamicActorBundle, GameState, PhysicLayer,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
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
    clibmer: Climber,
}

pub struct ClimberPlugin;

impl Plugin for ClimberPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((climb, spawn, health).in_set(OnUpdate(GameState::Playing)));
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
                transform: Transform::from_translation(Vec3::new(24.0 * -facing_mul, 0.0, 0.0)),
                sprite: Sprite {
                    color: Color::MIDNIGHT_BLUE,
                    custom_size: Some(CLIMBER_SHAPE),
                    ..default()
                },
                ..Default::default()
            },
            dynamic_actor_bundle: DynamicActorBundle {
                rigidbody: RigidBody::Dynamic,
                collider: Collider::cuboid(CLIMBER_SHAPE.x, CLIMBER_SHAPE.y),
                collision_groups: CollisionGroups::new(
                    (PhysicLayer::ENEMY | PhysicLayer::CLIMBER).into(),
                    (PhysicLayer::GROUND | PhysicLayer::CLIFF_EDGE | PhysicLayer::PLAYER_PROJ)
                        .into(),
                ),
                friction: Friction::coefficient(0.0),
                restitution: Restitution::coefficient(0.0),
                velocity: Velocity::linear(Vec2::new(facing_mul * 2.0, 0.0)),
                ..Default::default()
            },
            enemy: Enemy { health: 1, facing },
            clibmer: Climber,
        });
    }
}

fn climb(
    mut query: Query<(&mut Velocity, &CollidingEntities, &Enemy), With<Climber>>,
    sensor_query: Query<(Entity, &CollisionGroups), With<Sensor>>,
) {
    for (mut velocity, colliding_entities, enemy) in query.iter_mut() {
        for e in colliding_entities.iter() {
            for (entity, collision_groups) in sensor_query.iter() {
                if e == entity {
                    if collision_groups
                        .memberships
                        .contains(PhysicLayer::CLIFF_EDGE.into())
                    {
                        let mul: f32 = enemy.facing.into();
                        velocity.linvel = Vec2::new(1.0 * mul, 9.0);
                        return;
                    }
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
