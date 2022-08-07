use std::{time::Duration, ops::AddAssign};

use bevy::{prelude::*, math::vec3, render::camera::{Camera2d, ScalingMode}};
// use bevy_inspector_egui::egui::Key;
// use bevy_editor_pls::*;
use heron::prelude::*;

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
	.add_system(jump)
	.add_system(grounded_check)
	.run();
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Jump {
	grounded: bool,
	just_jumped: bool
}

#[derive(PhysicsLayer)]
enum PhysicsLayers {
	World,
	Hopper
}

fn startup_system(mut commands: Commands) {
	// Setup camera
	let mut cam = OrthographicCameraBundle::<Camera2d>::new_2d();
	cam.transform.translation = vec3(0., 40., 0.);
	cam.orthographic_projection.scaling_mode = ScalingMode::FixedHorizontal;
	cam.orthographic_projection.scale = 100.;
	commands.spawn_bundle(cam);
	
	// Spawn world
	let world_shape = Vec2::new(1000., 10.);
	commands
		.spawn()
		.insert_bundle(SpriteBundle {
			transform: Transform::from_translation(Vec3::new(0., -5., 0.)),
			sprite: Sprite {
				color: Color::WHITE,
				custom_size: Some(world_shape),
				..default()
			},
			..default()
		})
		.insert(RigidBody::Static)
		.insert(CollisionShape::Cuboid {
			half_extends: world_shape.extend(0.) / 2.,
			border_radius: None
		})
		.insert(CollisionLayers::none()
			.with_group(PhysicsLayers::World)
			.with_mask(PhysicsLayers::Hopper));
}

fn enemy_spawner(
	mut commands: Commands,
	time: Res<Time>,
	mut duration: Local<Duration>
) {
	duration.add_assign(time.delta());
	
	if duration.as_millis() > 5000 {
		*duration = Duration::ZERO;
		
		let enemy_shape = Vec2::new(10.0, 10.0);
		commands
			.spawn()
			.insert(Enemy)
			.insert_bundle(SpriteBundle {
				transform: Transform::from_translation(Vec3::new(-100., 1., 0.)),
				sprite: Sprite {
					color: Color::BLACK,
					custom_size: Some(enemy_shape),
					..default()
				},
				..default()
			})
			.insert(	Jump {
				just_jumped: false,
				grounded: true,
			})
			.insert(RigidBody::Dynamic)
			.insert(RotationConstraints::lock())
			.insert(PhysicMaterial {
				density: 1.,
				friction: 2.,
				restitution: 0.
			})
			.insert(CollisionShape::Cuboid {
				half_extends: enemy_shape.extend(0.) / 2.,
				border_radius: None
			})
			.insert(Acceleration::from_linear(Vec3::ZERO))
			.insert(CollisionLayers::none()
				.with_group(PhysicsLayers::Hopper)
    			.with_masks(&[PhysicsLayers::World, PhysicsLayers::Hopper])
			);
	}
}

fn jump(mut accel_query: Query<(&mut Acceleration, &mut Jump)>) {
	accel_query.for_each_mut(|(mut accel, mut jump)| {
		if jump.grounded {
			jump.just_jumped = true;
			jump.grounded = false;
			accel.linear = Vec3::new(500., 1500., 0.);
		}
		else if jump.just_jumped {
			jump.just_jumped = false;
			accel.linear = Vec3::ZERO;
		}
	});
}

fn grounded_check(
	mut events: EventReader<CollisionEvent>,
	mut query: Query<(Entity, &mut Jump)>
) {
	for event in events.iter() {
		if event.is_stopped() {break;}
		
		query.for_each_mut(|(entity, mut jump)| {
			if event.rigid_body_entities().0 == entity || event.rigid_body_entities().1 == entity {
				if event.collision_layers().0.contains_group(PhysicsLayers::World) ||
					event.collision_layers().1.contains_group(PhysicsLayers::World) {
					jump.grounded = true;
					// println!("{} collided with {}", event.rigid_body_entities().0.id(), event.rigid_body_entities().1.id());
					// println!("    entity is {}", entity.id());
				}
			}
		});
	}
}
