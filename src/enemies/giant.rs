use crate::enemies::{Enemy, Facing, Hop};
use crate::{DynamicActorBundle, GameState, PhysicsLayers};
use bevy::prelude::*;
use heron::prelude::*;
use rand::Rng;

const GIANT_SHAPE: Vec2 = Vec2::new(3., 6.);

#[derive(Component, Default)]
pub(crate) struct GiantSpawn;

#[derive(Component, Default)]
pub struct Giant;

#[derive(Bundle, Default)]
struct GiantBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    #[bundle]
    dynamic_actor_bundle: DynamicActorBundle,
    rotation_constraints: RotationConstraints,
    enemy: Enemy,
    giant: Giant,
    hop: Hop,
}

pub struct GiantPlugin;

impl Plugin for GiantPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(spawn));
    }
}

fn spawn(query: Query<(Entity, &GiantSpawn)>, mut commands: Commands) {
    for (entity, _spawn) in query.iter() {
        commands.entity(entity).despawn();

        let facing = if rand::thread_rng().gen_bool(0.5) {
            Facing::Left
        } else {
            Facing::Right
        };
        let facing_mul: f32 = facing.into();

        let power = Vec2::new(
            rand::thread_rng().gen_range(0.5..0.51) * facing_mul,
            rand::thread_rng().gen_range(10.5..11.0),
        );

        commands.spawn().insert_bundle(GiantBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(24. * -facing_mul, 6., 0.)),
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(GIANT_SHAPE),
                    ..default()
                },
                ..Default::default()
            },
            dynamic_actor_bundle: DynamicActorBundle {
                material: PhysicMaterial {
                    density: 1.,
                    friction: 2.,
                    restitution: 0.2,
                },
                shape: CollisionShape::Cuboid {
                    half_extends: GIANT_SHAPE.extend(0.) / 2.,
                    border_radius: None,
                },
                layers: CollisionLayers::none()
                    .with_groups(&[PhysicsLayers::Enemy, PhysicsLayers::Giant])
                    .with_masks(&[
                        PhysicsLayers::Ground,
                        // PhysicsLayers::Giant,
                        PhysicsLayers::PlayerProj,
                    ]),
                ..Default::default()
            },
            rotation_constraints: RotationConstraints::lock(),
            enemy: Enemy { health: 50, facing },
            hop: Hop {
                grounded: false,
                power,
            },
            ..Default::default()
        });
    }
}
