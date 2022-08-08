use crate::{GameState, PhysicsLayers};
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

fn spawn_world(mut commands: Commands) {
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

