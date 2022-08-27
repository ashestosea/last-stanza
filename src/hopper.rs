use crate::enemies::{Enemy, Hop, Facing};
use crate::{DynamicActorBundle, GameState, PhysicsLayers};
use bevy::prelude::*;
use heron::prelude::*;
use rand::Rng;

pub(crate) const HOPPER_SHAPE: Vec2 = Vec2::new(1., 2.);

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

pub struct HopperPlugin;

impl Plugin for HopperPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(hopper_grounding));
    }
}

fn hopper_grounding(mut query: Query<(&mut Hop, &Collisions)>) {
    for (mut hop, collisions) in query.iter_mut() {
        hop.grounded = false;

        for c in collisions.collision_data() {
            if c.collision_layers().contains_group(PhysicsLayers::Ground) {
                for n in c.normals() {
                    if *n == Vec3::Y {
                        hop.grounded = true;
                        return;
                    }
                }
            }
        }
    }
}

impl Hopper {
    pub fn spawn(mut commands: Commands, facing: Facing, start_x: f32) {
        let mul: f32 = facing.into();
        let power = Vec2::new(
            rand::thread_rng().gen_range(1.0..2.0) * mul,
            rand::thread_rng().gen_range(15.0..15.01),
        );

        commands.spawn().insert_bundle(HopperBundle {
            enemy: Enemy { health: 1, facing },
            hop: Hop {
                grounded: false,
                power,
            },
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(start_x, 6., 0.)),
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(HOPPER_SHAPE),
                    ..default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
