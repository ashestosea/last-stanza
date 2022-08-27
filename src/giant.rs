use crate::enemies::{Enemy, Facing, Hop};
use crate::{DynamicActorBundle, PhysicsLayers};
use bevy::prelude::*;
use heron::prelude::*;
use rand::Rng;

const GIANT_SHAPE: Vec2 = Vec2::new(3., 6.);

#[derive(Component, Default)]
pub(crate) struct Giant;

#[derive(Bundle, Default)]
struct GiantBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    #[bundle]
    dynamic_actor_bundle: DynamicActorBundle,
    rotation_constraints: RotationConstraints,
    enemy: Enemy,
    giant: Giant,
    hop: Hop,
}

impl Giant {
    pub fn spawn(mut commands: Commands, facing: Facing, start_x: f32) {
        let mul: f32 = facing.into();
        let power = Vec2::new(
            rand::thread_rng().gen_range(1.0..2.0) * mul,
            rand::thread_rng().gen_range(15.0..15.01),
        );

        commands.spawn().insert_bundle(GiantBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(start_x, 6., 0.)),
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(GIANT_SHAPE),
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
                    half_extends: GIANT_SHAPE.extend(0.) / 2.,
                    border_radius: None,
                },
                layers: CollisionLayers::none()
                    .with_groups(&[PhysicsLayers::Enemy, PhysicsLayers::Giant])
                    .with_masks(&[
                        PhysicsLayers::Ground,
                        PhysicsLayers::Giant,
                        PhysicsLayers::PProj,
                    ]),
                ..Default::default()
            },
            rotation_constraints: RotationConstraints::lock(),
            enemy: Enemy { health: 15, facing },
            hop: Hop {
                grounded: false,
                power,
            },
            ..Default::default()
        });
    }
}
