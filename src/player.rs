use crate::{DynamicActorBundle, GameState, PhysicsLayers};
use bevy::prelude::*;
use heron::prelude::*;

pub struct PlayerPlugin;

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
    pos: Vec2,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseData { pos: Vec2::ZERO })
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_projectile))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(mouse_move))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(place_projectile));
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
            transform: Transform::from_translation(Vec3::new(0., 37.5, 0.)),
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
            acceleration: Acceleration::from_linear(Vec3::new(-1., -1., 0.)),
            ..Default::default()
        });
}

fn mouse_move(mut mouse_data: ResMut<MouseData>, mut events: EventReader<CursorMoved>) {
    for e in events.iter() {
        mouse_data.pos = e.position;
    }
}

fn place_projectile(
    mut query: Query<&mut Transform, With<PlayerProjectile>>,
    mouse_data: Res<MouseData>,
    input: Res<Input<MouseButton>>,
) {
    for b in input.get_just_pressed() {
        if *b == MouseButton::Left {
            let mut transform = query
                .get_single_mut()
                .expect("Can't find player projectile");
            transform.translation = mouse_data.pos.extend(0.);
        }
    }
}
