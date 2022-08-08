use std::{time::Duration, ops::AddAssign};

use bevy::{prelude::*, math::vec3, render::camera::{Camera2d, ScalingMode}};
// use bevy_inspector_egui::egui::Key;
// use bevy_editor_pls::*;
use heron::prelude::*;
use rand::Rng;

fn main() {
	App::new()
	.add_plugins(DefaultPlugins)
	// .add_plugin(EditorPlugin)
	// .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
	// .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
	.add_plugin(PhysicsPlugin::default())
	.insert_resource(Gravity::from(Vec2::new(0., -9.81)))
	.add_startup_system(startup_system)
	.add_system(enemy_spawner)
	.add_system(hop)
	.add_system(hopper_grounding)
	.run();
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Hop {
	grounded: bool,
	just_hopped: bool,
	power: Vec2
}

#[derive(PhysicsLayer)]
enum PhysicsLayers {
	World,
	Hopper,
	Debug
}

fn startup_system(mut commands: Commands) {
	// Setup camera
	let mut cam = OrthographicCameraBundle::<Camera2d>::new_2d();
	cam.transform.translation = vec3(0., 40., 0.);
	cam.orthographic_projection.scaling_mode = ScalingMode::FixedHorizontal;
	cam.orthographic_projection.scale = 100.;
	commands.spawn_bundle(cam);
	
	// Spawn world
	let world_shape = Vec2::new(244., 10.);
	let one = 10f32;
	let two = 20f32;
	let three = 30f32;
	let four = 40f32;
	commands
        .spawn()
        .insert_bundle(SpriteBundle {
			sprite: Sprite {
			color: Color::hsla(0., 0., 0., 0.),
			custom_size: Some(world_shape),
			..Default::default()
			},
			transform: Transform::from_translation(Vec3::new(0., -5., 0.)),
			..Default::default()
		})
        .insert(CollisionShape::HeightField {
            size: world_shape,
            heights: vec![vec![
				0., 0.,
				0., 0.,
				one, one,
				two, two,
				three, three,
				four, four,
				three, three,
				two, two,
				one, one,
				0., 0.,
				0., 0.]],
        })
        .insert(RigidBody::Static)
		.insert(CollisionLayers::none()
			.with_group(PhysicsLayers::World)
			.with_masks(&[PhysicsLayers::Debug, PhysicsLayers::Hopper]));
}

fn enemy_spawner(
	mut commands: Commands,
	time: Res<Time>,
	mut duration: Local<Duration>
) {
	duration.add_assign(time.delta());
	
	if duration.as_millis() > 1000 {
		*duration = Duration::ZERO;
		if rand::thread_rng().gen_range(0f32..=1f32) < 0.9 {
			return;
		}
		
		let enemy_shape = Vec2::new(10.0, 10.0);
		commands
			.spawn()
			.insert(Enemy)
			.insert_bundle(SpriteBundle {
				transform: Transform::from_translation(Vec3::new(-120., 5., 0.)),
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
    			.with_masks(&[PhysicsLayers::World, PhysicsLayers::Hopper])
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
			if c.collision_layers().contains_group(PhysicsLayers::World) {
				hop.grounded = true;
				break;
			}
		}
 	});
 }
