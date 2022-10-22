use crate::{
    enemies::{Enemy, Facing, ExplosionBundle, Explosion},
    DynamicActorBundle, GameState, PhysicsLayers, loading::TextureAssets,
};
use bevy::prelude::*;
use heron::prelude::*;
use rand::Rng;

const CLIMBER_SHAPE: Vec2 = Vec2::new(1., 2.);

#[derive(Component, Default)]
pub(crate) struct ClimberSpawn;

#[derive(Component)]
pub(crate) struct Climber;

#[derive(Bundle)]
struct ClimberBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    #[bundle]
    dynamic_actor_bundle: DynamicActorBundle,
    rotation_constraints: RotationConstraints,
    enemy: Enemy,
    clibmer: Climber,
}

pub struct ClimberPlugin;

impl Plugin for ClimberPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(climb))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(spawn))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(health));
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

        commands.spawn().insert_bundle(ClimberBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(24. * -facing_mul, 0., 0.)),
                sprite: Sprite {
                    color: Color::MIDNIGHT_BLUE,
                    custom_size: Some(CLIMBER_SHAPE),
                    ..default()
                },
                ..Default::default()
            },
            dynamic_actor_bundle: DynamicActorBundle {
                rigidbody: RigidBody::Dynamic,
                material: PhysicMaterial {
                    friction: 0.,
                    restitution: 0.,
                    ..Default::default()
                },
                shape: CollisionShape::Cuboid {
                    half_extends: CLIMBER_SHAPE.extend(0.) / 2.,
                    border_radius: None,
                },
                layers: CollisionLayers::none()
                    .with_groups(&[PhysicsLayers::Enemy, PhysicsLayers::Climber])
                    .with_masks(&[
                        PhysicsLayers::Ground,
                        PhysicsLayers::CliffEdge,
                        PhysicsLayers::PlayerProj,
                    ]),
                velocity: Velocity::from_linear(Vec3::new(facing_mul * 2., 0., 0.)),
                ..Default::default()
            },
            rotation_constraints: RotationConstraints::lock(),
            enemy: Enemy { health: 1, facing },
            clibmer: Climber,
        });
    }
}

fn climb(mut query: Query<(&mut Velocity, &Collisions, &Enemy), With<Climber>>) {
    for (mut velocity, collision, enemy) in query.iter_mut() {
        for data in collision.collision_data() {
            if data
                .collision_layers()
                .contains_group(PhysicsLayers::CliffEdge)
            {
                let mul: f32 = enemy.facing.into();
                velocity.linear = Vec3::new(1. * mul, 9., 0.);
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
                println!("enemy ded");

            // Spawn Explosion
            commands.spawn().insert_bundle(ExplosionBundle {
                sprite_bundle: SpriteSheetBundle {
                    texture_atlas: texture_assets.explosion.clone(),
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::new(
                            enemy.health.abs() as f32 * 2.,
                            enemy.health.abs() as f32 * 2.,
                        )),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(trans.translation),
                    ..Default::default()
                },
                collision_shape: CollisionShape::Sphere {
                    radius: enemy.health.abs() as f32,
                },
                explosion: Explosion {
                    power: enemy.health.abs(),
                    timer: Timer::from_seconds(0.5, false),
                },
                ..Default::default()
            });
        }
    }
}
