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
	just_hopped: bool,
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
		if rand::thread_rng().gen_range(0f32..=1f32) < 0.95 {
			return;
		}
		
		let enemy_shape = Vec2::new(10.0, 10.0);
		commands
			.spawn()
			.insert(Enemy)
			.insert_bundle(SpriteBundle {
				transform: Transform::from_translation(Vec3::new(-120., 20., 0.)),
				sprite: Sprite {
					color: Color::BLACK,
					custom_size: Some(enemy_shape),
					..default()
				},
				..default()
			})
			.insert(	Hop {
				just_hopped: false,
				grounded: true,
				power: Vec2::new(
					rand::thread_rng().gen_range(150.0..300.0),
					rand::thread_rng().gen_range(700.0..1000.0))
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

fn hop(mut accel_query: Query<(&mut Acceleration, &mut Hop)>) {
	accel_query.for_each_mut(|(mut accel, mut hop)| {
		if hop.grounded {
			hop.just_hopped = true;
			hop.grounded = false;
			accel.linear = hop.power.extend(0.);
		}
		else if hop.just_hopped {
			hop.just_hopped = false;
			accel.linear = Vec3::ZERO;
		}
	});
}

fn hopper_grounding(mut query: Query<(&mut Hop, &Collisions)>) {
 	query.for_each_mut(|(mut hop, collisions)| {
		for c in collisions.collision_data() {
			if c.collision_layers().contains_group(PhysicsLayers::Ground) {
				hop.grounded = true;
				break;
			}
		}
 	});
 }
