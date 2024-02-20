use crate::{loading::TextureAssets, GameState, PhysicsLayers};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_xpbd_2d::prelude::*;

pub struct WorldPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_world);
    }
}

#[derive(Component)]
pub struct Ground;

#[derive(Bundle)]
struct WorldBundle {
    rigidbody: RigidBody,
    collider: Collider,
    collision_layers: CollisionLayers,
    friction: Friction,
    restitution: Restitution,
    position: Position,
    ground: Ground,
}

impl Default for WorldBundle {
    fn default() -> Self {
        Self {
            rigidbody: RigidBody::Static,
            collider: Collider::default(),
            collision_layers: CollisionLayers::all_masks::<PhysicsLayers>()
                .add_group(PhysicsLayers::Ground),
            friction: Friction::new(0.0),
            restitution: Restitution::new(0.0),
            position: Position::default(),
            ground: Ground,
        }
    }
}

fn spawn_world(
    mut commands: Commands,
    // texture_assets: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let step_height = 2.0;
    let step_decrement = 6.4;
    let step_count = 3;
    let blue_material = materials.add(ColorMaterial::from(Color::rgb(0.2, 0.7, 0.9)));
    let red_material = materials.add(ColorMaterial::from(Color::rgb(0.9, 0.3, 0.3)));
    let green_material = materials.add(ColorMaterial::from(Color::rgb(0.3, 0.9, 0.3)));

    // Ground
    let mut pos = Vec2::new(0.0, -3.0);
    // let mut pos = Vec2::ZERO;
    let ground_shape = Vec2::new(100.0, 6.0);

    // Ground texture
    // commands.spawn(SpriteBundle {
    //     texture: texture_assets.ground.clone(),
    //     sprite: Sprite {
    //         anchor: bevy::sprite::Anchor::TopCenter,
    //         custom_size: Some(Vec2::new(30.0, 0.703125)),
    //         ..Default::default()
    //     },
    //     transform: Transform::from_translation(Vec3::ZERO),
    //     ..Default::default()
    // });

    // Ground collider
    commands.spawn((
        WorldBundle {
            collider: Collider::cuboid(ground_shape.x, ground_shape.y),
            position: Position(pos),
            restitution: Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
            ..Default::default()
        },
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(ground_shape).into()).into(),
            material: red_material.clone(),
            ..default()
        },
    ));

    let mut step_shape = Vec2::new(0.0, step_height);

    // Ziggurat texture
    // commands.spawn(SpriteBundle {
    //     texture: texture_assets.ziggurat.clone(),
    //     sprite: Sprite {
    //         anchor: bevy::sprite::Anchor::BottomCenter,
    //         custom_size: Some(Vec2::new(
    //             step_height * 11.0,
    //             step_height * (step_count + 1) as f32,
    //         )),
    //         ..Default::default()
    //     },
    //     transform: Transform::from_translation(Vec3::new(0.0, -0.05, 0.0)),
    //     ..Default::default()
    // });

    // Step colliders
    for i in 0..=step_count {
        if i < step_count {
            step_shape.x = (step_height * 11.0) - (i as f32 * step_decrement);
        } else {
            step_shape.x = 1.0;
        }
        pos.y = 1.0 + step_height * i as f32;

        let step_mesh = meshes.add(shape::Quad::new(step_shape).into());

        commands.spawn((
            WorldBundle {
                collider: Collider::cuboid(step_shape.x, step_shape.y),
                position: Position(pos),
                ..Default::default()
            },
            MaterialMesh2dBundle {
                mesh: step_mesh.clone().into(),
                material: blue_material.clone(),
                ..default()
            },
        ));

        // Cliff sensor
        let cliff_shape = Vec2::new(step_shape.x + 1.5, 0.01);
        let cliff_mesh = meshes.add(shape::Quad::new(cliff_shape).into());
        commands.spawn((
            RigidBody::Static,
            Sensor,
            Collider::cuboid(cliff_shape.x, cliff_shape.y),
            Position(Vec2::new(pos.x, pos.y - (step_shape.y / 2.0))),
            MaterialMesh2dBundle {
                mesh: cliff_mesh.clone().into(),
                material: green_material.clone(),
                ..default()
            },
            CollisionLayers::new(
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
            ),
        ));
    }
}
