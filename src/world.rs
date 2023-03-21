use crate::{loading::TextureAssets, GameState, PhysicLayer};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct WorldPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_world.in_schedule(OnEnter(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Ground;

#[derive(Bundle)]
struct WorldBundle {
    transform_bundle: TransformBundle,
    rigidbody: RigidBody,
    collider: Collider,
    collision_groups: CollisionGroups,
    friction: Friction,
    restitution: Restitution,
    ground: Ground
}

impl Default for WorldBundle {
    fn default() -> Self {
        Self {
            transform_bundle: TransformBundle::default(),
            rigidbody: RigidBody::Fixed,
            collider: Collider::default(),
            collision_groups: CollisionGroups::new((PhysicLayer::GROUND).into(), Group::all()),
            friction: Friction::coefficient(0.0),
            restitution: Restitution::coefficient(0.0),
            ground: Ground
        }
    }
}

fn spawn_world(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    let step_height = 2.0;
    let step_decrement = 6.4;
    let step_count = 3;

    // Ground
    let mut pos = Vec3::new(0.0, -3.0, 0.0);
    let ground_shape = Vec2::new(50.0, 6.0);

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
        collider: Collider::cuboid(ground_shape.x / 2.0, ground_shape.y / 2.0),
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
            collider: Collider::cuboid(step_shape.x / 2.0, step_shape.y / 2.0),
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
            .insert(Collider::cuboid(cliff_shape.x / 2.0, cliff_shape.y / 2.0))
            .insert(CollisionGroups::new(
                PhysicLayer::CLIFF_EDGE.into(),
                Group::all(),
            ));
    }
}
