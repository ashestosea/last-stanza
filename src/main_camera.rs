use crate::{
    enemies::{Explosion, Giant, Hop},
    GameState,
};
use bevy::{prelude::*, render::camera::*};
use rand::Rng;

const CAM_POS: Vec3 = Vec3::new(0.0, 8.0, 0.0);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Default)]
struct CameraTrauma {
    trauma: f32,
    // shake: f32,
}

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera).add_systems(
            Update,
            (camera_shake, explosion_trauma, giant_steps).run_if(in_state(GameState::Playing)),
        );
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_translation(CAM_POS),
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedHorizontal(30.0),
                ..Default::default()
            },
            ..Default::default()
        },
        MainCamera,
        CameraTrauma::default(),
    ));
}

fn explosion_trauma(
    explosion_query: Query<&Explosion>,
    mut trauma_query: Query<&mut CameraTrauma>,
) {
    for ex in explosion_query.iter() {
        trauma_query.single_mut().trauma += (ex.power as f32 / 10.0).clamp(0.0, 1.0);
    }
}

fn giant_steps(giant_query: Query<&Hop, With<Giant>>, mut trauma_query: Query<&mut CameraTrauma>) {
    for g in giant_query.iter() {
        if g.grounded {
            trauma_query.single_mut().trauma += 0.5_f32.clamp(0.0, 1.0);
        }
    }
}

fn camera_shake(
    time: Res<Time>,
    mut trauma_query: Query<&mut CameraTrauma>,
    mut trans_query: Query<&mut Transform, With<MainCamera>>,
) {
    let mut trauma = trauma_query.single_mut();
    let mut trans = trans_query.single_mut();

    trans.translation = CAM_POS;
    trans.rotation = Quat::IDENTITY;

    if trauma.trauma.abs() < f32::EPSILON {
        return;
    }

    trauma.trauma = trauma.trauma.clamp(0.0, 1.333);
    let shake = trauma.trauma.powf(3.0);

    trans.translation.x += 0.1 * shake * rand::thread_rng().gen_range(-1.0..1.0);
    trans.translation.y += 0.1 * shake * rand::thread_rng().gen_range(-1.0..1.0);
    trans.rotate_z(0.005 * shake * rand::thread_rng().gen_range(-1.0..1.0));

    trauma.trauma -= time.delta_seconds() * 3.0;
}
