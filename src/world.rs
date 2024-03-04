use crate::{loading::TextureAssets, GameState, PhysicsLayers};
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_world);

        #[cfg(debug_assertions)]
        {
            app.add_systems(Update, debug_world.run_if(in_state(GameState::Playing)));
        }
    }
}

#[derive(Component)]
pub struct Ground;

#[derive(Bundle)]
struct WorldBundle {
    transform_bundle: TransformBundle,
    rigidbody: RigidBody,
    collider: Collider,
    collision_layers: CollisionLayers,
    friction: Friction,
    restitution: Restitution,
    ground: Ground,
}

impl Default for WorldBundle {
    fn default() -> Self {
        Self {
            transform_bundle: TransformBundle::default(),
            rigidbody: RigidBody::Static,
            collider: Collider::default(),
            collision_layers: CollisionLayers::new(
                [PhysicsLayers::Ground],
                [
                    PhysicsLayers::Behemoth,
                    PhysicsLayers::Climber,
                    PhysicsLayers::Diver,
                    PhysicsLayers::Enemy,
                    PhysicsLayers::Giant,
                    PhysicsLayers::Hopper,
                    PhysicsLayers::Lurker,
                    PhysicsLayers::PlayerProj,
                ],
            ),
            friction: Friction::new(0.0),
            restitution: Restitution::new(0.0),
            ground: Ground,
        }
    }
}

#[cfg(debug_assertions)]
fn debug_world(mut gizmos: Gizmos) {
    let step_height = 2.0;
    let step_decrement = 6.4;
    let step_count = 3;

    // Ground
    let mut pos = Vec3::new(0.0, -3.0, 0.0);
    let ground_shape = Vec2::new(100.0, 6.0);

    // Ground collider
    gizmos.rect_2d(pos.xy(), 0.0, ground_shape, Color::BLACK);

    let mut step_shape = Vec2::new(0.0, step_height);

    // Step colliders
    for i in 0..=step_count {
        if i < step_count {
            step_shape.x = (step_height * 11.0) - (i as f32 * step_decrement);
        } else {
            step_shape.x = 1.;
        }
        pos.y = 1.0 + step_height * i as f32;

        // Steps
        gizmos.rect_2d(pos.xy(), 0.0, step_shape, Color::RED);

        // Cliff sensors
        let cliff_shape = Vec2::new(step_shape.x + 1.5, 0.01);
        // commands.spawn(bevy::math::primitives::Rectangle)
        gizmos.primitive_2d(
            Rectangle::from_size(cliff_shape),
            Vec2::new(pos.x, pos.y - (step_shape.y / 4.0)),
            0.0,
            Color::GREEN,
        );
    }
}

fn spawn_world(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    let step_height = 2.0;
    let step_decrement = 6.4;
    let step_count = 3;

    // Ground
    let mut pos = Vec3::new(0.0, -3.0, 0.0);
    let ground_shape = Vec2::new(100.0, 6.0);

    // Ground texture
    commands.spawn(SpriteBundle {
        texture: texture_assets.ground.clone(),
        sprite: Sprite {
            anchor: bevy::sprite::Anchor::TopCenter,
            custom_size: Some(Vec2::new(30.0, 0.703125)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::ZERO),
        ..Default::default()
    });

    // Ground collider
    commands.spawn(WorldBundle {
        transform_bundle: TransformBundle {
            local: Transform::from_translation(pos),
            ..Default::default()
        },
        collider: Collider::rectangle(ground_shape.x, ground_shape.y),
        ..Default::default()
    });

    let mut step_shape = Vec2::new(0.0, step_height);

    // Ziggurat texture
    commands.spawn(SpriteBundle {
        texture: texture_assets.ziggurat.clone(),
        sprite: Sprite {
            anchor: bevy::sprite::Anchor::BottomCenter,
            custom_size: Some(Vec2::new(
                step_height * 11.0,
                step_height * (step_count + 1) as f32,
            )),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, -0.05, 0.0)),
        ..Default::default()
    });

    // Step colliders
    for i in 0..=step_count {
        if i < step_count {
            step_shape.x = (step_height * 11.0) - (i as f32 * step_decrement);
        } else {
            step_shape.x = 1.;
        }
        pos.y = 1.0 + step_height * i as f32;

        commands.spawn(WorldBundle {
            transform_bundle: TransformBundle {
                local: Transform::from_translation(pos),
                ..Default::default()
            },
            collider: Collider::rectangle(step_shape.x, step_shape.y),
            ..Default::default()
        });

        // Cliff sensor
        let cliff_shape = Vec2::new(step_shape.x + 1.5, 0.01);
        commands
            .spawn(TransformBundle {
                local: Transform::from_translation(Vec3::new(
                    pos.x,
                    pos.y - (step_shape.y / 4.0),
                    pos.z,
                )),
                ..Default::default()
            })
            .insert(Sensor)
            .insert(Collider::rectangle(cliff_shape.x, cliff_shape.y))
            .insert(CollisionLayers::new(
                [PhysicsLayers::CliffEdge],
                [
                    PhysicsLayers::Behemoth,
                    PhysicsLayers::Climber,
                    PhysicsLayers::Diver,
                    PhysicsLayers::Enemy,
                    PhysicsLayers::Giant,
                    PhysicsLayers::Hopper,
                    PhysicsLayers::Lurker,
                    PhysicsLayers::PlayerProj,
                ],
            ));
    }
}
