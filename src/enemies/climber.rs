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
pub(crate) struct Climber {
    climb_reset_timer: Timer,
}

#[derive(Bundle)]
struct ClimberBundle {
    sprite_bundle: SpriteBundle,
    dynamic_actor_bundle: DynamicActorBundle,
    external_impulse: ExternalImpulse,
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
                ..Default::default()
            },
            dynamic_actor_bundle: DynamicActorBundle {
                rigidbody: RigidBody::Dynamic,
                collider: Collider::cuboid(CLIMBER_SHAPE.x, CLIMBER_SHAPE.y),
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
                restitution: Restitution::ZERO,
                position: Position(Vec2::new(16.0 * -facing_mul, CLIMBER_SHAPE.y / 2.0)),
                velocity: LinearVelocity(Vec2::new(facing_mul * 2.0, 0.0)),
                ..Default::default()
            },
            external_impulse: Default::default(),
            enemy: Enemy { health: 1, facing },
            climber: Climber {
                climb_reset_timer: Timer::from_seconds(1.0, TimerMode::Once),
            },
        });
    }
}

fn climb(
    mut query: Query<(
        &mut ExternalImpulse,
        &CollidingEntities,
        &Enemy,
        &mut Climber,
    )>,
    sensor_query: Query<(Entity, &CollisionLayers), With<Sensor>>,
    time: Res<Time>,
    evt_reader: EventReader<CollisionStarted>
) {
    // for event in evt_reader.iter() {
        // event.0.type
    // }
    
    
    for (mut imp, colliding_entities, enemy, mut climber) in query.iter_mut() {
        if climber.climb_reset_timer.finished() {
            climber.climb_reset_timer.reset();
            for e in colliding_entities.iter() {
                for (entity, collision_layers) in sensor_query.iter() {
                    if e == &entity {
                        dbg!(collision_layers);
                        if collision_layers.contains_group(PhysicsLayers::CliffEdge) {
                            let mul: f32 = enemy.facing.into();
                            imp.set_impulse(Vec2::new(10.0 * mul, 8000.0));
                            dbg!(imp);
                            return;
                        }
                    }
                }
            }
        } else {
            climber.climb_reset_timer.tick(time.delta());
        }
    }
}

fn health(
    mut commands: Commands,
    query: Query<(Entity, &Enemy, &Position), (With<Climber>, Changed<Enemy>)>,
    texture_assets: Res<TextureAssets>,
) {
    for (entity, enemy, pos) in query.iter() {
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
                    ..Default::default()
                },
                position: Position(pos.0),
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
