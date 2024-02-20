use std::ops::Range;

use crate::enemies::enemy_projectile::ProjectileSpawn;
use crate::enemies::{Enemy, Explosion, ExplosionBundle, Facing, Hop};
use crate::loading::TextureAssets;
use crate::{DynamicActorBundle, GameState, PhysicsLayers};
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

use super::HopBundle;

#[derive(Component, Default)]
pub(crate) struct HopperSpawn;

#[derive(Component, Default)]
struct Hopper {
    pub collider_shape: Vec2,
    pub power: Vec2,
    pub spawn_point: Vec2,
}

impl Hopper {
    fn new(facing: Facing) -> Self {
        Self::new_all(
            facing,
            Vec2::new(2.0, 2.0),
            1.0..2.0,
            15.0..15.1,
            16.0..16.01,
            5.0..10.0,
        )
    }

    fn new_all(
        facing: Facing,
        collider_shape: Vec2,
        power_x: Range<f32>,
        power_y: Range<f32>,
        spawn_point_horz: Range<f32>,
        spawn_point_vert: Range<f32>,
    ) -> Self {
        let facing_mul = f32::from(facing);
        Self {
            collider_shape,
            power: Vec2::new(
                rand::thread_rng().gen_range(power_x.start..power_x.end) * facing_mul,
                rand::thread_rng().gen_range(power_y.start..power_y.end),
            ),
            spawn_point: Vec2::new(
                rand::thread_rng().gen_range(spawn_point_horz.start..spawn_point_horz.end)
                    * -facing_mul,
                rand::thread_rng().gen_range(spawn_point_vert.start..spawn_point_vert.end),
            ),
        }
    }
}

#[derive(Bundle, Default)]
struct HopperBundle {
    sprite_bundle: SpriteSheetBundle,
    dynamic_actor_bundle: DynamicActorBundle,
    enemy: Enemy,
    hopper: Hopper,
    hop: HopBundle,
    external_impulse: ExternalImpulse,
}

pub struct HopperPlugin;

impl Plugin for HopperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn, shoot, health, animate).run_if(in_state(GameState::Playing)),
        );
    }
}

fn spawn(
    query: Query<(Entity, &HopperSpawn)>,
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
) {
    for (entity, _spawn) in query.iter() {
        commands.entity(entity).despawn();

        let facing = if rand::thread_rng().gen_bool(0.5) {
            Facing::Left
        } else {
            Facing::Right
        };

        let hopper = Hopper::new(facing);
        let collider = Collider::cuboid(hopper.collider_shape.x, hopper.collider_shape.y);

        commands.spawn(HopperBundle {
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: texture_assets.hopper.clone(),
                sprite: TextureAtlasSprite {
                    flip_x: facing.into(),
                    custom_size: Some(hopper.collider_shape),
                    ..default()
                },
                ..Default::default()
            },
            dynamic_actor_bundle: DynamicActorBundle {
                collider: collider.clone(),
                collision_layers: CollisionLayers::new(
                    [PhysicsLayers::Enemy, PhysicsLayers::Hopper],
                    [
                        PhysicsLayers::Ground,
                        PhysicsLayers::Hopper,
                        PhysicsLayers::Player,
                        PhysicsLayers::PlayerProj,
                        PhysicsLayers::Explosion,
                    ],
                ),
                friction: Friction::new(20.0),
                // restitution: Restitution::new(0.2),
                restitution: Restitution::new(0.0).with_combine_rule(CoefficientCombine::Multiply),
                position: Position(hopper.spawn_point),
                ..Default::default()
            },
            enemy: Enemy { health: 1, facing },
            hop: HopBundle {
                hop: Hop {
                    grounded: false,
                    power: hopper.power,
                    hop_timer: Timer::from_seconds(0.5, TimerMode::Once),
                    hop_reset_timer: Timer::from_seconds(1.0, TimerMode::Once),
                },
                shape_caster: ShapeCaster::new(
                    collider,
                    Vec2 {
                        x: 0.0,
                        y: -hopper.collider_shape.y,
                        // y: 0.0
                    },
                    0.0,
                    Vec2::NEG_Y,
                )
                .with_max_time_of_impact(100.0)
                .with_query_filter(SpatialQueryFilter::new().with_masks([PhysicsLayers::Ground])),
            },
            ..Default::default()
        });
    }
}

fn shoot(mut commands: Commands, query: Query<&Position, With<Hopper>>) {
    for pos in query.iter() {
        if rand::thread_rng().gen_range(0.0..1.0) > 0.999 && pos.0.x.abs() < 15.0 {
            commands.spawn(ProjectileSpawn { pos: pos.0.trunc() });
        }
    }
}

fn animate(mut query: Query<(&mut TextureAtlasSprite, &LinearVelocity)>) {
    for (mut texture, velocity) in query.iter_mut() {
        if velocity.y > 0.2 {
            texture.index = 0;
        } else if velocity.y < -0.2 {
            texture.index = 2;
        } else {
            texture.index = 1;
        }
    }
}

fn health(
    mut commands: Commands,
    query: Query<(Entity, &Enemy, &Position), (With<Hopper>, Changed<Enemy>)>,
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
                collider: Collider::ball(enemy.health.abs() as f32),
                position: Position(pos.0),
                explosion: Explosion {
                    power: enemy.health.abs(),
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                },
                ..Default::default()
            });
        }
    }
}
