use crate::{
    enemies::{Enemy, Explosion, ExplosionBundle, Facing},
    loading::TextureAssets,
    DynamicActorBundle, GameState, PhysicsLayers,
};
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

const LURKER_SHAPE: Vec2 = Vec2::new(1.0, 2.0);

#[derive(Component, Default)]
pub(crate) struct LurkerSpawn;

#[derive(Component)]
pub(crate) struct Lurker {
    step: u8,
    timer: Timer,
}

#[derive(Bundle)]
struct LurkerBundle {
    sprite_bundle: SpriteBundle,
    dynamic_actor_bundle: DynamicActorBundle,
    // active_collision_types: ActiveCollisionTypes,
    external_force: ExternalForce,
    enemy: Enemy,
    lurker: Lurker,
}

pub struct LurkerPlugin;

impl Plugin for LurkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (lurk, spawn, health).in_set(GameState::Playing));
    }
}

fn spawn(query: Query<(Entity, &LurkerSpawn)>, mut commands: Commands) {
    for (entity, _spawn) in query.iter() {
        commands.entity(entity).despawn();

        let facing = if rand::thread_rng().gen_bool(0.5) {
            Facing::Left
        } else {
            Facing::Right
        };
        let facing_mul: f32 = facing.into();

        commands.spawn(LurkerBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(16.0 * -facing_mul, 3.0, 0.0)),
                sprite: Sprite {
                    color: Color::PURPLE,
                    custom_size: Some(LURKER_SHAPE),
                    ..default()
                },
                ..Default::default()
            },
            dynamic_actor_bundle: DynamicActorBundle {
                rigidbody: RigidBody::Dynamic,
                collider: Collider::cuboid(LURKER_SHAPE.x / 2.0, LURKER_SHAPE.y / 2.0),
                collision_layers: CollisionLayers::new(
                    [PhysicsLayers::Enemy, PhysicsLayers::Lurker],
                    [
                        PhysicsLayers::Ground,
                        PhysicsLayers::Player,
                        PhysicsLayers::PlayerProj,
                        PhysicsLayers::Explosion,
                    ],
                ),
                friction: Friction::new(10.0),
                restitution: Restitution::new(0.0),
                ..Default::default()
            },
            external_force: ExternalForce::new(Vec2::new(facing_mul * 7.0, 0.0)),
            enemy: Enemy { health: 1, facing },
            lurker: Lurker {
                step: 0,
                timer: Timer::from_seconds(
                    rand::thread_rng().gen_range(3f32..5f32),
                    TimerMode::Once,
                ),
            },
        });
    }
}

fn lurk(mut query: Query<(&Enemy, &mut ExternalForce, &mut Lurker)>, time: Res<Time>) {
    for (enemy, mut force, mut lurker) in query.iter_mut() {
        lurker.timer.tick(time.delta());
        if lurker.timer.finished() && lurker.step == 0 {
            lurker.step += 1;
            let mul: f32 = enemy.facing.into();
            force.set_force(Vec2::new(7.5 * mul, 28.0));
        }
    }
}

fn health(
    mut commands: Commands,
    query: Query<(Entity, &Enemy, &Transform), (With<Lurker>, Changed<Enemy>)>,
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
