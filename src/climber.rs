use crate::{enemies::Enemy, PhysicsLayers};
use bevy::prelude::*;
use heron::prelude::*;

#[derive(Component)]
pub(crate) struct Climber;

#[derive(Bundle)]
pub(crate) struct ClimberBundle {
    #[bundle]
    pub(crate) sprite_bundle: SpriteBundle,
    pub(crate) rigidbody: RigidBody,
    pub(crate) shape: CollisionShape,
    pub(crate) layers: CollisionLayers,
    pub(crate) collisions: Collisions,
    pub(crate) enemy: Enemy,
    pub(crate) clibmer: Climber
}

pub(crate) const CLIMBER_SHAPE: Vec2 = Vec2::new(1., 2.);

impl Default for ClimberBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: Default::default(),
            enemy: Enemy::default(),
            rigidbody: RigidBody::KinematicPositionBased,
            shape: CollisionShape::Cuboid {
                half_extends: CLIMBER_SHAPE.extend(0.) / 2.,
                border_radius: None,
            },
            layers: CollisionLayers::none()
                .with_groups(&[PhysicsLayers::Enemy, PhysicsLayers::Climber])
                .with_masks(&[
                    PhysicsLayers::PProj,
                ]),
            clibmer: Climber,
            collisions: Collisions::default()
        }
    }
}