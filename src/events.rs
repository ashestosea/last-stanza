use crate::{enemies::SpawnRates, loading::GameData, GameState};
use bevy::{prelude::*, time::Stopwatch, utils::HashMap};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemySpawnsChanged>()
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(load))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(update));
    }
}

pub struct EnemySpawnsChanged {
    pub hopper: Option<f32>,
    pub climber: Option<f32>,
    pub sneaker: Option<f32>,
    pub diver: Option<f32>,
    pub giant: Option<f32>,
    pub behemoth: Option<f32>,
}

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Clone)]
#[uuid = "c2609287-9672-4cb8-b95d-afb0a8df2200"]
pub struct TimeTable {
    pub t: toml::value::Table,
}

#[derive(Default)]
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
        if let Some(val) = value.get("hopper") {
            result.hopper = Some(val.as_float().unwrap() as f32);
        }
        if let Some(val) = value.get("climber") {
            result.climber = Some(val.as_float().unwrap() as f32);
        }
        if let Some(val) = value.get("sneaker") {
            result.sneaker = Some(val.as_float().unwrap() as f32);
        }
        if let Some(val) = value.get("diver") {
            result.diver = Some(val.as_float().unwrap() as f32);
        }
        if let Some(val) = value.get("giant") {
            result.giant = Some(val.as_float().unwrap() as f32);
        }
        if let Some(val) = value.get("behemoth") {
            result.behemoth = Some(val.as_float().unwrap() as f32);
        }

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
                hopper: entry.1.hopper,
                climber: entry.1.climber,
                sneaker: entry.1.sneaker,
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
