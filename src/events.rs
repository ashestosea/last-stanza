use crate::{enemies::SpawnRates, loading::GameData, GameState};
use bevy::{prelude::*, reflect::TypePath, time::Stopwatch, utils::HashMap};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemySpawnsChanged>()
            .add_systems(OnEnter(GameState::Menu), load)
            .add_systems(Update, update.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Event)]
pub struct EnemySpawnsChanged {
    pub min_spawn_time: Option<f32>,
    pub max_spawn_time: Option<f32>,
    pub hopper: Option<f32>,
    pub climber: Option<f32>,
    pub lurker: Option<f32>,
    pub diver: Option<f32>,
    pub giant: Option<f32>,
    pub behemoth: Option<f32>,
}

#[derive(serde::Deserialize, Clone, TypePath, Asset)]
// #[uuid = "c2609287-9672-4cb8-b95d-afb0a8df2200"]
pub struct TimeTable {
    pub t: toml::value::Table,
}

#[derive(Resource, Default)]
struct SpawnRatesOverTime {
    table: HashMap<String, SpawnRates>,
}

impl SpawnRatesOverTime {
    fn new() -> Self {
        Self { ..default() }
    }
}

impl From<TimeTable> for SpawnRatesOverTime {
    fn from(time_table: TimeTable) -> Self {
        let mut result = Self::new();
        for t in time_table.t.iter() {
            result.table.insert(t.0.to_owned(), t.1.into());
        }

        result
    }
}

impl From<&toml::Value> for SpawnRates {
    fn from(value: &toml::Value) -> Self {
        let mut result = SpawnRates::default();
        if let Some(val) = value.get("min_spawn_time") {
            result.min_spawn_time = Some(val.as_float().unwrap() as f32);
        }
        if let Some(val) = value.get("max_spawn_time") {
            result.max_spawn_time = Some(val.as_float().unwrap() as f32);
        }
        if let Some(val) = value.get("hopper") {
            result.hopper = Some(val.as_integer().unwrap() as f32);
        }
        if let Some(val) = value.get("climber") {
            result.climber = Some(val.as_integer().unwrap() as f32);
        }
        if let Some(val) = value.get("lurker") {
            result.lurker = Some(val.as_integer().unwrap() as f32);
        }
        if let Some(val) = value.get("diver") {
            result.diver = Some(val.as_integer().unwrap() as f32);
        }
        if let Some(val) = value.get("giant") {
            result.giant = Some(val.as_integer().unwrap() as f32);
        }
        if let Some(val) = value.get("behemoth") {
            result.behemoth = Some(val.as_integer().unwrap() as f32);
        }

        let total_chance = result.all();

        result.hopper = Some(result.hopper.unwrap() / total_chance);
        result.climber = Some(result.climber.unwrap() / total_chance);
        result.lurker = Some(result.lurker.unwrap() / total_chance);
        result.diver = Some(result.diver.unwrap() / total_chance);
        result.giant = Some(result.giant.unwrap() / total_chance);
        result.behemoth = Some(result.behemoth.unwrap() / total_chance);

        result
    }
}

pub fn load(
    mut commands: Commands,
    game_data: Res<GameData>,
    spawn_rates_asset: Res<Assets<TimeTable>>,
) {
    if let Some(spawn_rates_time_table) = spawn_rates_asset.get(&game_data.spawn_rates) {
        commands.insert_resource(SpawnRatesOverTime::from(spawn_rates_time_table.to_owned()));
    }
}

fn update(
    time: Res<Time>,
    mut current_time: Local<Stopwatch>,
    mut spawn_rates_over_time: ResMut<SpawnRatesOverTime>,
    mut ev_writer: EventWriter<EnemySpawnsChanged>,
) {
    current_time.tick(time.delta());
    let spawn_table = spawn_rates_over_time.table.clone();
    let mut remove_key = "";
    for entry in spawn_table.iter() {
        if current_time.elapsed().as_secs() == entry.0.parse::<u64>().unwrap() {
            remove_key = entry.0;
            ev_writer.send(EnemySpawnsChanged {
                min_spawn_time: entry.1.min_spawn_time,
                max_spawn_time: entry.1.max_spawn_time,
                hopper: entry.1.hopper,
                climber: entry.1.climber,
                lurker: entry.1.lurker,
                diver: entry.1.diver,
                giant: entry.1.giant,
                behemoth: entry.1.behemoth,
            });
        }
    }

    if spawn_rates_over_time.table.contains_key(remove_key) {
        spawn_rates_over_time.table.remove(remove_key);
    }
}
