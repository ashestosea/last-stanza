use crate::{
    enemies::{Enemy, Facing},
    DynamicActorBundle, PhysicsLayers,
};
use bevy::prelude::*;
use heron::prelude::*;

#[derive(Component)]
pub(crate) struct Climber;

#[derive(Bundle)]
pub(crate) struct ClimberBundle {
    #[bundle]
    pub(crate) sprite_bundle: SpriteBundle,
    #[bundle]
    pub(crate) dynamic_actor_bundle: DynamicActorBundle,
    pub(crate) rotation_constraints: RotationConstraints,
    pub(crate) enemy: Enemy,
    pub(crate) clibmer: Climber,
}

pub(crate) const CLIMBER_SHAPE: Vec2 = Vec2::new(1., 2.);

pub(crate) fn spawn_climber(mut commands: Commands, facing: Facing, start_x: f32) {
    let facing_mul: f32 = facing.into();

    commands.spawn().insert_bundle(ClimberBundle {
        enemy: Enemy { health: 1, facing },
        sprite_bundle: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(start_x, 0., 0.)),
            sprite: Sprite {
                color: Color::BISQUE,
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
                    PhysicsLayers::PProj,
                ]),
            velocity: Velocity::from_linear(Vec3::new(facing_mul * 2., 0., 0.)),
            ..Default::default()
        },
        rotation_constraints: RotationConstraints::lock(),
        clibmer: Climber,
    });
}

pub(crate) fn climb(mut query: Query<(&mut Velocity, &Collisions, &Enemy), With<Climber>>) {
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
