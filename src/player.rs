use crate::enemies::enemy_projectile::EnemyProjectile;
use crate::enemies::Enemy;
use crate::main_camera::MainCamera;
use crate::PhysicsLayers;
use crate::{loading::TextureAssets, DynamicActorBundle, GameState};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_xpbd_2d::prelude::*;

pub const PLAYER_CENTER: Vec2 = Vec2::new(0.0, 8.75);
pub const PLAYER_SIZE: Vec2 = Vec2::new(0.75, 1.5);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseData {
            world_pos: Vec2::ZERO,
            vec_from_player: Vec2::Y,
        })
        .add_systems(OnEnter(GameState::Playing), spawn_player)
        .add_systems(
            Update,
            (
                mouse_input,
                aim,
                launch,
                projectile_destruction,
                projectile_timeouts,
                hit,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
pub(crate) struct PlayerProjectile {
    pub size: i32,
}

#[derive(Component)]
struct Charging {
    timer: Timer,
}

#[derive(Component)]
struct Timeout {
    timer: Timer,
}

#[derive(Component)]
struct Fired;

#[derive(Resource, Default)]
struct MouseData {
    world_pos: Vec2,
    vec_from_player: Vec2,
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            // texture: textures.texture_bevy.clone(),
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(PLAYER_SIZE),
                ..Default::default()
            },
            transform: Transform::from_translation(PLAYER_CENTER.extend(0.0)),
            ..Default::default()
        })
        .insert(Player)
        .insert(RigidBody::Static)
        .insert(Collider::cuboid(PLAYER_SIZE.x / 2.0, PLAYER_SIZE.y / 2.0))
        .insert(CollidingEntities::default())
        .insert(CollisionLayers::new(
            [PhysicsLayers::Player],
            [PhysicsLayers::Enemy, PhysicsLayers::EnemyProj],
        ));
}

fn spawn_projectile(commands: &mut Commands, texture_assets: Res<TextureAssets>) -> Entity {
    let entity = &commands
        .spawn(SpriteBundle {
            texture: texture_assets.circle.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::ONE),
                color: Color::GREEN,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(-10.0, 45.0, 0.0)),
            ..Default::default()
        })
        .insert(PlayerProjectile { size: 1 })
        .insert(DynamicActorBundle {
            rigidbody: RigidBody::Static,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collider: Collider::ball(0.5),
            collision_layers: CollisionLayers::new(
                [PhysicsLayers::PlayerProj],
                [
                    PhysicsLayers::Ground,
                    PhysicsLayers::Enemy,
                    PhysicsLayers::EnemyProj,
                ],
            ),
            friction: Friction {
                static_coefficient: 0.0,
                dynamic_coefficient: 0.0,
                combine_rule: CoefficientCombine::Min,
            },
            restitution: Restitution {
                coefficient: 2.0,
                combine_rule: Default::default(),
            },
            mass: Mass(100.0),
            ..Default::default()
        })
        .insert(Timeout {
            timer: Timer::from_seconds(3.0, TimerMode::Once),
        })
        .insert(Charging {
            timer: Timer::from_seconds(10.0, TimerMode::Repeating),
        })
        .insert(GravityScale { 0: 3.0 })
        .id();

    return *entity;
}

fn aim(
    time: Res<Time>,
    mouse_data: Res<MouseData>,
    mut proj_query: Query<(&mut PlayerProjectile, &mut Transform, &mut Charging)>,
) {
    for (mut proj, mut proj_trans, mut charge) in proj_query.iter_mut() {
        charge.timer.tick(time.delta());
        let c = (charge.timer.elapsed_secs().sin().powi(2) * 4.0) + 1.0;
        proj.size = c.round() as i32;
        proj_trans.scale = Vec3::ONE * (0.5 + (0.05 * proj.size as f32));
        proj_trans.translation = (2.0 * mouse_data.vec_from_player + PLAYER_CENTER).extend(0.0);
    }
}

fn launch(
    mut commands: Commands,
    mouse_data: Res<MouseData>,
    mut query: Query<(Entity, &mut LinearVelocity, &PlayerProjectile), With<Fired>>,
) {
    for (entity, mut vel, projectile) in query.iter_mut() {
        let velocity =
            mouse_data.vec_from_player * (((projectile.size as f32 - 1.0).atan() * 12.0) + 7.0);
        vel.x = velocity.x;
        vel.y = velocity.y;
        commands
            .entity(entity)
            .remove::<Fired>()
            .insert(SleepingDisabled);
    }
}

fn mouse_input(
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    input: Res<Input<MouseButton>>,
    cam_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    proj_query: Query<Entity, (With<PlayerProjectile>, With<Charging>)>,
    texture_assets: Res<TextureAssets>,
    mut commands: Commands,
    mut mouse_data: ResMut<MouseData>,
    mut events: EventReader<CursorMoved>,
) {
    let (camera, camera_transform) = cam_query.single();

    let Ok(window) = primary_window_query.get_single() else {
        return;
    };

    for e in events.iter() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let ndc = (e.position / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        mouse_data.world_pos = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();
    }

    let a = Vec2::Y;
    let b = (mouse_data.world_pos - PLAYER_CENTER).normalize();
    let mut angle = ((a.x * b.x) + (a.y * b.y)).acos();
    let cross = (a.x * b.y) - (a.y * b.x);

    if cross < 0.0 {
        angle *= -1.0;
    }

    mouse_data.vec_from_player = Vec2::from_angle(angle).rotate(Vec2::Y).normalize();

    if input.just_pressed(MouseButton::Left) {
        spawn_projectile(&mut commands, texture_assets);
    } else if input.just_released(MouseButton::Left) {
        for p in proj_query.iter() {
            commands
                .entity(p)
                .remove::<RigidBody>()
                .remove::<Charging>()
                .insert(RigidBody::Dynamic)
                .insert(Fired);
        }
    }
}

fn projectile_explode(commands: &mut Commands, projectile_entity: Entity) {
    commands.entity(projectile_entity).despawn();
}

fn projectile_timeouts(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Timeout), Without<Charging>>,
    time: Res<Time>,
) {
    for (entity, mut timeout) in query.iter_mut() {
        if timeout.timer.finished() {
            projectile_explode(&mut commands, entity);
        } else {
            timeout.timer.tick(time.delta());
        }
    }
}

fn projectile_destruction(
    mut commands: Commands,
    mut proj_query: Query<(Entity, &CollidingEntities), With<PlayerProjectile>>,
    enemy_query: Query<Entity, (With<Enemy>, Without<EnemyProjectile>)>,
) {
    for (proj_entity, colliding_entities) in proj_query.iter_mut() {
        for e in colliding_entities.iter() {
            for enemy_entity in enemy_query.iter() {
                if &enemy_entity == e {
                    projectile_explode(&mut commands, proj_entity);
                    break;
                }
            }
        }
    }
}

fn hit(
    mut state: ResMut<NextState<GameState>>,
    mut time: ResMut<Time>,
    query: Query<&CollidingEntities, With<Player>>,
) {
    for colliding_entities in query.iter() {
        if !colliding_entities.is_empty() {
            state.set(GameState::Menu);
            time.pause();
            return;
        }
    }
}
