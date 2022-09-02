use crate::{loading::TextureAssets, GameState, PhysicsLayers};
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

#[derive(Bundle)]
struct WorldBundle {
    #[bundle]
    transform_bundle: TransformBundle,
    rigidbody: RigidBody,
    collision_shape: CollisionShape,
    collision_layers: CollisionLayers,
    material: PhysicMaterial,
}

impl Default for WorldBundle {
    fn default() -> Self {
        Self {
            transform_bundle: TransformBundle::default(),
            rigidbody: RigidBody::Static,
            collision_shape: Default::default(),
            collision_layers: CollisionLayers::all_masks::<PhysicsLayers>()
                .with_group(PhysicsLayers::Ground),
            material: PhysicMaterial {
                restitution: 0.,
                density: 0.,
                friction: 0.,
            },
        }
    }
}

fn spawn_world(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    let step_height = 2.;
    let step_decrement = 6.4;
    let step_count = 3;

    // Ground
    let mut pos = Vec3::new(0., -3., 0.);
    let ground_shape = Vec2::new(50., 6.);

    // Ground texture
    commands.spawn().insert_bundle(SpriteBundle {
        texture: texture_assets.ground.clone(),
        sprite: Sprite {
            anchor: bevy::sprite::Anchor::TopCenter,
            custom_size: Some(Vec2::new(30., 0.703125)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::ZERO),
        ..Default::default()
    });

    // Ground collider
    commands.spawn().insert_bundle(WorldBundle {
        transform_bundle: TransformBundle {
            local: Transform::from_translation(pos),
            ..Default::default()
        },
        collision_shape: CollisionShape::Cuboid {
            half_extends: ground_shape.extend(0.) / 2.,
            border_radius: None,
        },
        ..Default::default()
    });

    let mut step_shape = Vec2::new(0., step_height);

    // Ziggurat texture
    commands.spawn().insert_bundle(SpriteBundle {
        texture: texture_assets.ziggurat.clone(),
        sprite: Sprite {
            anchor: bevy::sprite::Anchor::BottomCenter,
            custom_size: Some(Vec2::new(
                step_height * 11.,
                step_height * (step_count + 1) as f32,
            )),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0., -0.05, 0.)),
        ..Default::default()
    });

    // Step colliders
    for i in 0..=step_count {
        if i < step_count {
            step_shape.x = (step_height * 11.) - (i as f32 * step_decrement);
        } else {
            step_shape.x = 1.;
        }
        pos.y = 1. + step_height * i as f32;

        commands.spawn().insert_bundle(WorldBundle {
            transform_bundle: TransformBundle {
                local: Transform::from_translation(pos),
                ..Default::default()
            },
            collision_shape: CollisionShape::Cuboid {
                half_extends: step_shape.extend(0.) / 2.,
                border_radius: None,
            },
            ..Default::default()
        });

        // Cliff sensor
        let cliff_shape = Vec3::new(step_shape.x + 1.5, 0.01, 0.);
        commands
            .spawn()
            .insert_bundle(TransformBundle {
                local: Transform::from_translation(Vec3::new(
                    pos.x,
                    pos.y - (step_shape.y / 4.),
                    pos.z,
                )),
                ..Default::default()
            })
            .insert(RigidBody::Sensor)
            .insert(CollisionShape::Cuboid {
                half_extends: cliff_shape / 2.,
                border_radius: None,
            })
            .insert(
                CollisionLayers::all_masks::<PhysicsLayers>().with_group(PhysicsLayers::CliffEdge),
            );
    }
}
