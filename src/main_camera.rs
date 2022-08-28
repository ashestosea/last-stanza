use crate::GameState;
use bevy::{math::vec3, prelude::*, render::camera::ScalingMode};

#[derive(Component)]
pub struct MainCamera;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_camera));
    }
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform::from_translation(vec3(0., 7., 0.)),
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedHorizontal(30.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainCamera);
}
