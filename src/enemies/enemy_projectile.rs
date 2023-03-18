use crate::enemies::Enemy;
use crate::loading::TextureAssets;
use crate::player::PLAYER_CENTER;
use crate::{DynamicActorBundle, GameState, PhysicLayer};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PROJECTILE_SHAPE: Vec2 = Vec2::new(0.3, 0.3);

#[derive(Component, Default)]
pub(crate) struct ProjectileSpawn {
    pub pos: Vec2,
}

#[derive(Component, Default)]
pub(crate) struct Projectile;

#[derive(Bundle, Default)]
struct ProjectileParentBundle {
    spacial_bundle: SpatialBundle,
    dynamic_actor_bundle: DynamicActorBundle,
    projectile: Projectile,
    enemy: Enemy,
}

#[derive(Bundle, Default)]
struct ProjectileChildBundle {
    sprite: SpriteBundle,
    dynamic_actor_bundle: DynamicActorBundle,
    sensor: Sensor,
    projectile: Projectile,
    enemy: Enemy,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (spawn, health, projectile_destruction, animate).in_set(OnUpdate(GameState::Playing)),
        );
    }
}

fn spawn(
    query: Query<(Entity, &ProjectileSpawn)>,
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
) {
    for (entity, spawn) in query.iter() {
        commands.entity(entity).despawn();

        commands
            .spawn(ProjectileParentBundle {
                spacial_bundle: SpatialBundle {
                    transform: Transform::from_xyz(spawn.pos.x, spawn.pos.y, 0.0),
                    ..Default::default()
                },
                dynamic_actor_bundle: DynamicActorBundle {
                    rigidbody: RigidBody::KinematicVelocityBased,
                    collider: Collider::ball(0.001),
                    collision_groups: CollisionGroups::default(),
                    locked_axes: LockedAxes::ROTATION_LOCKED,
                    velocity: Velocity::linear((PLAYER_CENTER - spawn.pos).normalize()),
                    ..Default::default()
                },
                projectile: Default::default(),
                enemy: Enemy {
                    health: 1,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(ProjectileChildBundle {
                    sprite: SpriteBundle {
                        transform: Transform::from_translation(Vec3::ZERO),
                        texture: texture_assets.circle.clone(),
                        sprite: Sprite {
                            color: Color::PINK,
                            custom_size: Some(PROJECTILE_SHAPE),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    dynamic_actor_bundle: DynamicActorBundle {
                        collider: Collider::ball(0.3),
                        collision_groups: CollisionGroups::new(
                            (PhysicLayer::ENEMY | PhysicLayer::ENEMY_PROJ).into(),
                            (PhysicLayer::PLAYER | PhysicLayer::PLAYER_PROJ).into(),
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
    }
}

fn health(
    mut commands: Commands,
    query: Query<(&Parent, &Enemy), (With<Projectile>, Changed<Enemy>)>,
    // parent_query: Query<&Transform>,
    // texture_assets: Res<TextureAssets>,
) {
    for (parent, enemy) in query.iter() {
        if enemy.health <= 0 {
            let _ = &commands.entity(parent.get()).despawn_recursive();
            // let ex_pos = parent_query.get(parent.get()).unwrap().translation;
            // Spawn Explosion
            // commands.spawn(ExplosionBundle {
            //     sprite_bundle: SpriteSheetBundle {
            //         texture_atlas: texture_assets.explosion.clone(),
            //         sprite: TextureAtlasSprite {
            //             custom_size: Some(Vec2::new(
            //                 enemy.health.abs() as f32 * 2.0,
            //                 enemy.health.abs() as f32 * 2.0,
            //             )),
            //             ..Default::default()
            //         },
            //         transform: Transform::from_translation(ex_pos),
            //         ..Default::default()
            //     },
            //     collision_shape: CollisionShape::Sphere {
            //         radius: enemy.health.abs() as f32,
            //     },
            //     explosion: Explosion {
            //         power: enemy.health.abs(),
            //         timer: Timer::from_seconds(0.5, false),
            //     },
            //     ..Default::default()
            // });
        }
    }
}

fn projectile_destruction(
    query: Query<(Entity, &CollidingEntities), With<Projectile>>,
    mut commands: Commands,
) {
    for (entity, colliding_entities) in query.iter() {
        if !colliding_entities.is_empty() {
            println!("enemy proj collision");
            commands.entity(entity).despawn();
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
