use crate::enemies::{Enemy, Facing, Hop, ExplosionBundle, Explosion};
use crate::loading::TextureAssets;
use crate::{DynamicActorBundle, GameState, PhysicLayer};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

const GIANT_SHAPE: Vec2 = Vec2::new(3.0, 6.0);

#[derive(Component, Default)]
pub(crate) struct GiantSpawn;

#[derive(Component, Default)]
pub struct Giant;

#[derive(Bundle, Default)]
struct GiantBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    #[bundle]
    dynamic_actor_bundle: DynamicActorBundle,
    locked_axes: LockedAxes,
    enemy: Enemy,
    giant: Giant,
    hop: Hop,
}

pub struct GiantPlugin;

impl Plugin for GiantPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(spawn))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(health));
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
            rand::thread_rng().gen_range(0.5..0.51) * facing_mul,
            rand::thread_rng().gen_range(10.5..11.0),
        );

        commands.spawn().insert_bundle(GiantBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(24.0 * -facing_mul, 6.0, 0.0)),
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(GIANT_SHAPE),
                    ..default()
                },
                ..Default::default()
            },
            dynamic_actor_bundle: DynamicActorBundle {
                collider: Collider::cuboid(GIANT_SHAPE.x, GIANT_SHAPE.y),
                collision_groups: CollisionGroups::new(
                    (PhysicLayer::ENEMY | PhysicLayer::GIANT).into(),
                    (PhysicLayer::GROUND | PhysicLayer::PLAYER_PROJ).into()
                ),
                friction: Friction::coefficient(2.0),
                restitution: Restitution::coefficient(0.2),
                ..Default::default()
            },
            enemy: Enemy { health: 50, facing },
            hop: Hop {
                grounded: false,
                power,
            },
            ..Default::default()
        });
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
                println!("enemy ded");

            // Spawn Explosion
            commands.spawn().insert_bundle(ExplosionBundle {
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
                    timer: Timer::from_seconds(0.5, false),
                },
                ..Default::default()
            });
        }
    }
}
