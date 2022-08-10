use crate::{GameState, PhysicsLayers};
use bevy::prelude::*;
use heron::prelude::*;
use rand::Rng;
use std::{time::Duration, ops::AddAssign};

pub struct EnemiesPlugin;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Hopper;

#[derive(Component)]
struct Climber;

#[derive(Component)]
struct Sneaker;

#[derive(Component)]
struct Diver;

#[derive(Component)]
struct Giant;

#[derive(Component)]
struct Behemoth;

#[derive(Bundle)]
struct HopperBundle {
	enemy: Enemy,
	hopper: Hopper,
	hop: Hop
}

#[derive(Component)]
struct Hop {
	grounded: bool,
	power: Vec2
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_system_set(SystemSet::on_update(GameState::Playing).with_system(enemy_spawner))
			.add_system_set(SystemSet::on_update(GameState::Playing).with_system(hop))
			.add_system_set(SystemSet::on_update(GameState::Playing).with_system(hopper_grounding));
    }
}

fn enemy_spawner(
	mut commands: Commands,
	time: Res<Time>,
	mut duration: Local<Duration>
) {
	duration.add_assign(time.delta());
	
	if duration.as_millis() > 1000 {
		*duration = Duration::ZERO;
		if rand::thread_rng().gen_range(0f32..1f32) < 0.85 {
			return;
		}
		
		let enemy_shape = Vec2::new(6.0, 11.0);
		commands
			.spawn()
			.insert(Enemy)
			.insert_bundle(SpriteBundle {
				transform: Transform::from_translation(Vec3::new(-120., 5.5, 0.)),
				sprite: Sprite {
					color: Color::BLACK,
					custom_size: Some(enemy_shape),
					..default()
				},
				..default()
			})
			.insert(Hop {
				grounded: true,
				power: Vec2::new(
					rand::thread_rng().gen_range(150.0..250.0),
					rand::thread_rng().gen_range(700.0..900.0)),
			})
			.insert(RigidBody::Dynamic)
			.insert(RotationConstraints::lock())
			.insert(PhysicMaterial {
				density: 1.,
				friction: 2.,
				restitution: 0.25
			})
			.insert(CollisionShape::Cuboid {
				half_extends: enemy_shape.extend(0.) / 2.,
				border_radius: None
			})
			.insert(Acceleration::from_linear(Vec3::ZERO))
			.insert(CollisionLayers::none()
				.with_group(PhysicsLayers::Hopper)
    			.with_masks(&[PhysicsLayers::Ground, PhysicsLayers::Hopper, PhysicsLayers::Wall])
			)
			.insert(Collisions::default()
		);
	}
}

fn hop(mut query: Query<(&mut Acceleration, &Hop)>) {
	for (mut accel, hop) in query.iter_mut() {
		if hop.grounded {
			accel.linear = hop.power.extend(0.);
		}
		else if accel.linear.length_squared() > 0. {
			accel.linear = Vec3::ZERO;
		}
	};
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
 	};
 }
