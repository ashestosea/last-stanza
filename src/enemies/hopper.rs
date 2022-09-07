use crate::enemies::{Enemy, Facing, Hop};
use crate::loading::TextureAssets;
use crate::{DynamicActorBundle, GameState, PhysicsLayers};
use bevy::prelude::*;
use heron::prelude::*;
use rand::Rng;

const HOPPER_SHAPE: Vec2 = Vec2::new(2., 2.);

#[derive(Component, Default)]
pub(crate) struct HopperSpawn;

#[derive(Component, Default)]
struct Hopper;

#[derive(Bundle, Default)]
struct HopperBundle {
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    #[bundle]
    dynamic_actor_bundle: DynamicActorBundle,
    rotation_constraints: RotationConstraints,
    enemy: Enemy,
    hopper: Hopper,
    hop: Hop,
}

pub struct HopperPlugin;

impl Plugin for HopperPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(spawn))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(animate));
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
        let facing_mul: f32 = facing.into();

        let power = Vec2::new(
            rand::thread_rng().gen_range(1.0..2.0) * facing_mul,
            rand::thread_rng().gen_range(15.0..15.01),
        );

        commands.spawn().insert_bundle(HopperBundle {
            sprite_bundle: SpriteSheetBundle {
                texture_atlas: texture_assets.hopper.clone(),
                transform: Transform::from_translation(Vec3::new(24. * -facing_mul, 6., 0.)),
                sprite: TextureAtlasSprite {
                    flip_x: facing.into(),
                    custom_size: Some(HOPPER_SHAPE),
                    ..default()
                },
                ..Default::default()
            },
            dynamic_actor_bundle: DynamicActorBundle {
                material: PhysicMaterial {
                    density: 1.,
                    friction: 2.,
                    restitution: 0.2,
                },
                shape: CollisionShape::Cuboid {
                    half_extends: HOPPER_SHAPE.extend(0.) / 2.,
                    border_radius: None,
                },
                layers: CollisionLayers::none()
                    .with_groups(&[PhysicsLayers::Enemy, PhysicsLayers::Hopper])
                    .with_masks(&[
                        PhysicsLayers::Ground,
                        PhysicsLayers::Hopper,
                        PhysicsLayers::PProj,
                    ]),
                ..Default::default()
            },
            rotation_constraints: RotationConstraints::lock(),
            enemy: Enemy { health: 1, facing },
            hop: Hop {
                grounded: false,
                power,
            },
            ..Default::default()
        });
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
