use crate::{GameState, PhysicsLayers};
use bevy::prelude::*;
use heron::prelude::*;

pub struct WorldPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_world));
    }
}

fn spawn_world(mut commands: Commands) {
    let step_size = 20.;

    // Ground
    let mut collider_pos = Vec3::new(0., 0., 0.);
    commands
        .spawn()
        .insert_bundle(TransformBundle {
            local: Transform::from_translation(collider_pos),
            ..Default::default()
        })
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3 {
                x: step_size * 5.,
                y: 0.1,
                z: 0.,
            },
            border_radius: None,
        })
        .insert(RigidBody::Static)
        .insert(
            CollisionLayers::none()
                .with_group(PhysicsLayers::Ground)
                .with_masks(&[PhysicsLayers::Debug, PhysicsLayers::Hopper]),
        );

    let mut collider_half_extents = Vec3::new((step_size * 5.) / 2., 0.1, 0.);
    for i in 0..=4 {
        // Colliders
        commands
            .spawn()
            .insert_bundle(TransformBundle {
                local: Transform::from_translation(collider_pos),
                ..Default::default()
            })
            .insert(CollisionShape::Cuboid {
                half_extends: collider_half_extents,
                border_radius: None,
            })
            .insert(RigidBody::Static)
            .insert(
                CollisionLayers::none()
                    .with_group(PhysicsLayers::Ground)
                    .with_masks(&[PhysicsLayers::Debug, PhysicsLayers::Hopper]),
            );

        // Sprites
        commands.spawn().insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::hsla(1., 1., 1., 1.),
                custom_size: Some(Vec2::new(
                    collider_half_extents.x * 2.,
                    collider_half_extents.y,
                )),
                ..Default::default()
            },
            transform: Transform::from_translation(collider_pos),
            ..Default::default()
        });

        collider_half_extents.x = collider_half_extents.x - (step_size / 2.);
        collider_pos.y = collider_pos.y + (step_size / 2.);
    }

    // Left Walls
    let collider_half_extents = Vec3::new(0.1, step_size / 2., 0.);
    let mut collider_pos = Vec3::new(-step_size * 2., step_size / 4., 0.);
    for i in 0..=3 {
        // Colliders
        commands
            .spawn()
            .insert_bundle(TransformBundle {
                local: Transform::from_translation(collider_pos),
                ..Default::default()
            })
            .insert(CollisionShape::Cuboid {
                half_extends: collider_half_extents,
                border_radius: None,
            })
            .insert(RigidBody::Static)
            .insert(
                CollisionLayers::none()
                    .with_group(PhysicsLayers::Wall)
                    .with_masks(&[PhysicsLayers::Debug, PhysicsLayers::Hopper]),
            );

        // Sprites
        commands.spawn().insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::hsla(1., 1., 1., 1.),
                custom_size: Some(Vec2::new(
                    collider_half_extents.x * 2.,
                    collider_half_extents.y,
                )),
                ..Default::default()
            },
            transform: Transform::from_translation(collider_pos),
            ..Default::default()
        });

        collider_pos.x = collider_pos.x + (step_size / 2.);
        collider_pos.y = collider_pos.y + (step_size / 2.);
    }

    // Right Walls
    let collider_half_extents = Vec3::new(0.1, step_size / 2., 0.);
    let mut collider_pos = Vec3::new(step_size * 2., step_size / 4., 0.);
    for i in 0..=3 {
        // Colliders
        commands
            .spawn()
            .insert_bundle(TransformBundle {
                local: Transform::from_translation(collider_pos),
                ..Default::default()
            })
            .insert(CollisionShape::Cuboid {
                half_extends: collider_half_extents,
                border_radius: None,
            })
            .insert(RigidBody::Static)
            .insert(
                CollisionLayers::none()
                    .with_group(PhysicsLayers::Wall)
                    .with_masks(&[PhysicsLayers::Debug, PhysicsLayers::Hopper]),
            );

        // Sprites
        commands.spawn().insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::hsla(1., 1., 1., 1.),
                custom_size: Some(Vec2::new(
                    collider_half_extents.x * 2.,
                    collider_half_extents.y,
                )),
                ..Default::default()
            },
            transform: Transform::from_translation(collider_pos),
            ..Default::default()
        });

        collider_pos.x = collider_pos.x - (step_size / 2.);
        collider_pos.y = collider_pos.y + (step_size / 2.);
    }
}
