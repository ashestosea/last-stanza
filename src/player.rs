use crate::{DynamicActorBundle, GameState, MainCamera, PhysicsLayers};
use bevy::{prelude::*, render::camera::RenderTarget};
use heron::prelude::*;

const PLAYER_CENTER: Vec2 = Vec2::new(0., 37.5);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseData {
            world_pos: Vec2::ZERO,
            vec_from_player: Vec2::Y,
        })
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_projectile))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(mouse_input))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(aim))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(launch))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(projectile_destruction));
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerProjectile {
    size: f32,
}

#[derive(Component)]
struct Charging;

#[derive(Component)]
struct Fired;

#[derive(Default)]
struct MouseData {
    world_pos: Vec2,
    vec_from_player: Vec2,
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
        .insert(Player);
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

fn aim(
    mouse_data: Res<MouseData>,
    mut proj_query: Query<(&mut PlayerProjectile, &mut Transform), With<Charging>>,
) {
    for (mut proj, mut proj_trans) in proj_query.iter_mut() {
        proj_trans.translation = (5. * mouse_data.vec_from_player + PLAYER_CENTER).extend(0.);
    }
}

fn launch(
    mut commands: Commands,
    mouse_data: Res<MouseData>,
    mut query: Query<(Entity, &mut Impulse), (With<PlayerProjectile>, With<Fired>)>,
) {
    for (entity, mut impulse) in query.iter_mut() {
        impulse.linear = mouse_data.vec_from_player.extend(0.) * 50.;

        commands.entity(entity).remove::<Fired>();
    }
}

fn mouse_input(
    windows: Res<Windows>,
    input: Res<Input<MouseButton>>,
    cam_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    proj_query: Query<Entity, With<PlayerProjectile>>,
    mut commands: Commands,
    mut mouse_data: ResMut<MouseData>,
    mut events: EventReader<CursorMoved>,
) {
    let (camera, camera_transform) = cam_query.single();

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
        angle *= -1.;
    }

    mouse_data.vec_from_player = Vec2::from_angle(angle).rotate(Vec2::Y).normalize();

    if input.just_pressed(MouseButton::Left) {
        let entity = proj_query.single();
        commands
            .entity(entity)
            .insert(RigidBody::Dynamic)
            .insert(Charging)
            .insert(Velocity::from_linear(Vec3::ZERO));
    } else if input.just_released(MouseButton::Left) {
        let entity = proj_query.single();
        commands
            .entity(entity)
            .remove::<Velocity>()
            .remove::<Charging>()
            .insert(Fired);
    }
}

fn projectile_destruction(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Collisions), With<PlayerProjectile>>
) {
    for (entity, mut transform, collisions) in query.iter_mut() {
        for c in collisions.collision_data() {
            if c.collision_layers().contains_group(PhysicsLayers::Enemy) {
                transform.translation = Vec3::NEG_Y * 50.;
                commands.entity(entity).remove::<RigidBody>();
            }
        }
    }
}
