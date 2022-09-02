use crate::{
    enemies::{Explosion, Giant, Hop},
    GameState,
};
use bevy::{prelude::*, render::camera::ScalingMode};
use heron::utils::NearZero;
use rand::Rng;

const CAM_POS: Vec3 = Vec3::new(0., 8., 0.);

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
        app.insert_resource(Timer::from_seconds(0.125, false))
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_camera))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(camera_shake))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(explosion_trauma))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(giant_steps));
    }
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform::from_translation(CAM_POS),
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedHorizontal(30.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainCamera)
        .insert(CameraTrauma::default());
}

fn explosion_trauma(
    explosion_query: Query<&Explosion>,
    mut trauma_query: Query<&mut CameraTrauma>,
) {
    for ex in explosion_query.iter() {
        trauma_query.single_mut().trauma += (ex.power as f32 / 10.).clamp(0., 1.);
    }
}

fn giant_steps(giant_query: Query<&Hop, With<Giant>>, mut trauma_query: Query<&mut CameraTrauma>) {
    for g in giant_query.iter() {
        if g.grounded {
            trauma_query.single_mut().trauma += 0.5_f32.clamp(0., 1.);
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
    if trauma.trauma.is_near_zero() {
        return;
    }

    let shake = trauma.trauma.powf(3.);

    trans.translation.x += 0.1 * shake * rand::thread_rng().gen_range(-1.0..1.0);
    trans.translation.y += 0.1 * shake * rand::thread_rng().gen_range(-1.0..1.0);
    trans.rotate_z(0.005 * shake * rand::thread_rng().gen_range(-1.0..1.0));

    trauma.trauma -= time.delta_seconds() * 3.;
    trauma.trauma = trauma.trauma.clamp(0., 1.);
}
