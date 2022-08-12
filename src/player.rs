use crate::{DynamicActorBundle, GameState, MainCamera, PhysicsLayers};
use bevy::{prelude::*, render::camera::RenderTarget};
use heron::prelude::*;

const PLAYER_CENTER: Vec2 = Vec2::new(0., 37.5);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseData {
            world_pos: Vec2::ZERO,
            angle: 0.
        })
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_projectile))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(mouse_move))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(start_aim))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(aim))
        ;
    }
}

#[derive(Component)]
struct Player {
    angle: f32,
}

#[derive(Component)]
struct PlayerProjectile {
    size: f32,
}

#[derive(Component)]
struct Charging;

#[derive(Default)]
struct MouseData {
    world_pos: Vec2,
    angle: f32,
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
            transform: Transform::from_translation(PLAYER_CENTER.extend(0.)),
            ..Default::default()
        })
        .insert(Player { angle: 0. });
}

fn spawn_projectile(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(3., 3.)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(-10., 45., 0.)),
            ..Default::default()
        })
        .insert(PlayerProjectile { size: 0.5 })
        .insert_bundle(DynamicActorBundle {
            shape: CollisionShape::Sphere { radius: 3. },
            material: PhysicMaterial {
                friction: 0.,
                restitution: 1.5,
                ..Default::default()
            },
            layers: CollisionLayers::new(PhysicsLayers::PProj, PhysicsLayers::Ground)
                .with_mask(PhysicsLayers::Enemy),
            ..Default::default()
        });
}

fn start_aim(
    mut commands: Commands,
    mouse_data: Res<MouseData>,
    input: Res<Input<MouseButton>>,
    mut query: Query<(Entity, &mut PlayerProjectile, &mut Impulse)>
) {
    if input.just_pressed(MouseButton::Left) {
        let (entity, projectile, impulse) = query.single_mut();
        commands
            .entity(entity)
            .insert(Charging)
            .insert(Velocity::from_linear(Vec3::ZERO));
    } else if input.just_released(MouseButton::Left) {
        let (entity, projectile, mut impulse) = query.single_mut();

        let t = Vec2::Y;
        let result: Vec2 = Vec2::new(
            (mouse_data.angle.cos() * t.x) + (-mouse_data.angle.sin() * t.y),
            (mouse_data.angle.sin() * t.x) + (mouse_data.angle.cos() * t.y),
        );

        commands
            .entity(entity)
            .remove::<Velocity>()
            .remove::<Charging>();
        
        impulse.linear = result.extend(0.).normalize() * 50.;
    }
}

fn aim(
    mouse_data: Res<MouseData>,
    player_query: Query<&mut Player>,
    mut proj_query: Query<(&mut PlayerProjectile, &mut Transform, &mut Charging)>,
) {
    if proj_query.is_empty() {
        return;
    }

    let (mut projectile, mut projectile_trans, mut charging) = proj_query.single_mut();

    let t = Vec2::Y;
    let result: Vec2 = Vec2::new(
        (mouse_data.angle.cos() * t.x) + (-mouse_data.angle.sin() * t.y),
        (mouse_data.angle.sin() * t.x) + (mouse_data.angle.cos() * t.y),
    );

    projectile_trans.translation = 5. * result.extend(0.) + PLAYER_CENTER.extend(0.);
}

fn mouse_move(
    windows: Res<Windows>,
    query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut mouse_data: ResMut<MouseData>,
    mut events: EventReader<CursorMoved>,
) {
    let (camera, camera_transform) = query.single();

    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    for e in events.iter() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let ndc = (e.position / window_size) * 2. - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        mouse_data.world_pos = ndc_to_world.project_point3(ndc.extend(-1.)).truncate();
    }

    let a = Vec2::Y;
    let b = (mouse_data.world_pos - PLAYER_CENTER).normalize();
    let mut angle = ((a.x * b.x) + (a.y * b.y)).acos();
    let cross = (a.x * b.y) - (a.y * b.x);

    if cross < 0. {
        angle = angle * -1.;
    }

    mouse_data.angle = angle;
}
