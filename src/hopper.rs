use crate::enemies::{Enemy, Hop};
use crate::{DynamicActorBundle, PhysicsLayers};
use bevy::prelude::*;
use heron::prelude::*;

#[derive(Component)]
pub(crate) struct Hopper;

#[derive(Bundle)]
pub(crate) struct HopperBundle {
    #[bundle]
    pub(crate) sprite_bundle: SpriteBundle,
    #[bundle]
    pub(crate) dynamic_actor_bundle: DynamicActorBundle,
    pub(crate) rotation_constraints: RotationConstraints,
    pub(crate) enemy: Enemy,
    pub(crate) hopper: Hopper,
    pub(crate) hop: Hop,
}

pub(crate) const HOPPER_SHAPE: Vec2 = Vec2::new(1., 2.);

impl Default for HopperBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: Default::default(),
            enemy: Enemy::default(),
            hopper: Hopper,
            hop: Hop::default(),
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
        }
    }
}