use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
struct Player {
	charge: f32
}

#[derive(Component)]
struct PlayerProjectile {
	is_max: bool
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player));
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            // texture: textures.texture_bevy.clone(),
			sprite: Sprite {
				color: Color::RED,
				custom_size: Some(Vec2::new(5., 5.)),
				..Default::default()
			},
            transform: Transform::from_translation(Vec3::new(0., 50., 1.)),
            ..Default::default()
        })
        .insert(Player {charge: 0.});
}

