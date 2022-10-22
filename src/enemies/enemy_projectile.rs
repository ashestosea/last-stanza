use crate::enemies::{Enemy, ExplosionBundle, Explosion};
use crate::loading::TextureAssets;
use crate::player::PLAYER_CENTER;
use crate::{GameState, PhysicsLayers};
use bevy::prelude::*;
use heron::prelude::*;

const PROJECTILE_SHAPE: Vec2 = Vec2::new(0.3, 0.3);

#[derive(Component, Default)]
pub(crate) struct ProjectileSpawn {
    pub pos: Vec3,
}

#[derive(Component, Default)]
struct Projectile;

#[derive(Bundle, Default)]
struct ProjectileParentBundle {
    #[bundle]
    spacial_bundle: SpatialBundle,
    rigidbody: RigidBody,
    collision_shape: CollisionShape,
    collision_layers: CollisionLayers,
    collisions: Collisions,
    rotation_constraints: RotationConstraints,
    velocity: Velocity,
    projectile: Projectile,
    enemy: Enemy,
}

#[derive(Bundle, Default)]
struct ProjectileChildBundle {
    #[bundle]
    sprite: SpriteBundle,
    rigidbody: RigidBody,
    collision_shape: CollisionShape,
    collision_layers: CollisionLayers,
    collisions: Collisions,
    // rotation_constraints: RotationConstraints,
    projectile: Projectile,
    enemy: Enemy,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(spawn))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(health))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(projectile_destruction),
            )
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(animate));
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
            .spawn_bundle(ProjectileParentBundle {
                spacial_bundle: SpatialBundle {
                    transform: Transform::from_translation(spawn.pos),
                    ..Default::default()
                },
                rigidbody: RigidBody::KinematicVelocityBased,
                collision_shape: CollisionShape::Sphere { radius: 0.001 },
                collision_layers: CollisionLayers::none(),
                rotation_constraints: RotationConstraints::lock(),
                velocity: Velocity::from_linear((PLAYER_CENTER.extend(0.) - spawn.pos).normalize()),
                projectile: Default::default(),
                enemy: Enemy {
                    health: 1,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(ProjectileChildBundle {
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
                    rigidbody: RigidBody::Sensor,
                    collision_shape: CollisionShape::Sphere { radius: 0.3 },
                    collision_layers: CollisionLayers::none()
                        .with_groups(&[PhysicsLayers::Enemy, PhysicsLayers::EnemyProj])
                        .with_masks(&[PhysicsLayers::Player, PhysicsLayers::PlayerProj]),
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
            // commands.spawn().insert_bundle(ExplosionBundle {
            //     sprite_bundle: SpriteSheetBundle {
            //         texture_atlas: texture_assets.explosion.clone(),
            //         sprite: TextureAtlasSprite {
            //             custom_size: Some(Vec2::new(
            //                 enemy.health.abs() as f32 * 2.,
            //                 enemy.health.abs() as f32 * 2.,
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
    query: Query<(Entity, &Collisions), With<Projectile>>,
    mut commands: Commands,
) {
    for (entity, collisions) in query.iter() {
        if !collisions.is_empty() {
            println!("enemy proj collision");
            commands.entity(entity).despawn();
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
