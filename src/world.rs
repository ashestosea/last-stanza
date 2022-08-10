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
    let step_height = 10.;
    let step_decrement = 32.;
    let step_count = 3;

    // Ground
    let mut pos = Vec3::new(0., -20., 0.);
    let ground_shape = Vec2::new(1000., 30.);
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::hsla(0.5, 1., 0.6, 1.),
                custom_size: Some(ground_shape),
                ..Default::default()
            },
            transform: Transform::from_translation(pos),
            ..Default::default()
        })
        .insert(CollisionShape::Cuboid {
            half_extends: ground_shape.extend(0.) / 2.,
            border_radius: None,
        })
        .insert(RigidBody::Static)
        .insert(
            CollisionLayers::none()
                .with_group(PhysicsLayers::Ground)
                .with_masks(&[PhysicsLayers::Debug, PhysicsLayers::Hopper]),
        );

    let mut step_shape = Vec2::new(0., step_height);
    for i in 0..=step_count {
        if i < step_count {
            step_shape.x = (step_height * 11.) - (i as f32 * step_decrement);
        } else {
            step_shape.x = 1.;
        }
        pos.y = step_height * i as f32;

        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::hsla(1., 1., 1., 1.),
                    custom_size: Some(step_shape),
                    ..Default::default()
                },
                transform: Transform::from_translation(pos),
                ..Default::default()
            })
            .insert(CollisionShape::Cuboid {
                half_extends: step_shape.extend(0.) / 2.,
                border_radius: None,
            })
            .insert(RigidBody::Static)
            .insert(
                CollisionLayers::none()
                    .with_group(PhysicsLayers::Ground)
                    .with_masks(&[PhysicsLayers::Debug, PhysicsLayers::Hopper]),
            );
    }
}
