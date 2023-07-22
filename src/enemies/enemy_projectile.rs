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
pub(crate) struct EnemyProjectile;

#[derive(Bundle, Default)]
struct ProjectileChildBundle {
    sprite: SpriteBundle,
    dynamic_actor_bundle: DynamicActorBundle,
    active_collision_types: ActiveCollisionTypes,
    sensor: Sensor,
    projectile: EnemyProjectile,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn, projectile_destruction, animate).in_set(GameState::Playing),
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
            .spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(spawn.pos.x, spawn.pos.y, 0.0),
                    ..Default::default()
                },
                RigidBody::KinematicVelocityBased,
                LockedAxes::ROTATION_LOCKED,
                Velocity::linear((PLAYER_CENTER - spawn.pos).normalize()),
            ))
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
                        rigidbody: RigidBody::Fixed,
                        collider: Collider::ball(0.3),
                        collision_groups: CollisionGroups::new(
                            (PhysicLayer::ENEMY | PhysicLayer::ENEMY_PROJ).into(),
                            (PhysicLayer::PLAYER
                                | PhysicLayer::PLAYER_PROJ
                                | PhysicLayer::EXPLOSION)
                                .into(),
                        ),
                        ..Default::default()
                    },
                    active_collision_types: ActiveCollisionTypes::KINEMATIC_STATIC,
                    ..Default::default()
                });
            });
    }
}

fn projectile_destruction(
    query: Query<(Entity, &CollidingEntities), With<EnemyProjectile>>,
    mut commands: Commands,
) {
    for (entity, colliding_entities) in query.iter() {
        if !colliding_entities.is_empty() {
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
