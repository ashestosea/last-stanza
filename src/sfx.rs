use crate::{enemies::Explosion, loading::AudioAssets, GameState};
use bevy::prelude::*;

#[derive(Resource, Default)]
struct ExplosionsLog {
    pub log: Vec<Entity>,
}
#[derive(Resource, Default)]
struct CurrentExplosionsLog {
    pub log: Vec<Entity>,
}

#[derive(Resource, Default)]
struct ExplosionSoundTimer(Timer);

pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (explosion_sounds)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(ExplosionSoundTimer(Timer::from_seconds(
        0.32,
        TimerMode::Once,
    )));
    commands.insert_resource(ExplosionsLog { log: Vec::new() });
    commands.insert_resource(CurrentExplosionsLog { log: Vec::new() });
}

fn explosion_sounds(
    mut commands: Commands,
    mut timer: ResMut<ExplosionSoundTimer>,
    mut explosions_log: ResMut<ExplosionsLog>,
    mut current_explosions_log: ResMut<CurrentExplosionsLog>,
    explosions: Query<Entity, With<Explosion>>,
    time: Res<Time>,
    audio_assets: Res<AudioAssets>,
) {
    timer.0.tick(time.delta());

    for entity in explosions.iter() {
        let seen = explosions_log.log.contains(&entity);
        
        if !seen {
            explosions_log.log.push(entity);
        }
        
        if !seen && !current_explosions_log.log.contains(&entity) {
            current_explosions_log.log.push(entity);
        }
    }

    if timer.0.finished() {
        timer.0.reset();

        if current_explosions_log.log.len() < 1 {
            return;
        }

        let drum = match current_explosions_log.log.len() {
            1 => audio_assets.snare_hit_1.clone(),
            2 => audio_assets.snare_hit_2.clone(),
            3 => audio_assets.snare_hit_3.clone(),
            _ => audio_assets.snare_hit_max.clone(),
        };

        current_explosions_log.log.clear();

        commands.spawn(AudioBundle {
            source: drum,
            ..Default::default()
        });
    }
}
