use crate::{events::TimeTable, GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
// use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TomlAssetPlugin::<TimeTable>::new(&["time.toml"]))
            // .add_plugins(
            //     ProgressPlugin::new(GameState::Loading).continue_to(GameState::Menu)
            // )
            // .add_systems(Update, print_state)
            .add_loading_state(
                LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu), // .on_failure_continue_to_state(GameState::LoadingFailed),
            )
            .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
                GameState::Loading,
                "fonts.assets.ron",
            )
            .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
                GameState::Loading,
                "textures.assets.ron",
            )
            .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
            .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
            .add_collection_to_loading_state::<_, GameData>(GameState::Loading);
        // .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
    }
}

#[derive(Resource, AssetCollection)]
pub struct FontAssets {
    #[asset(key = "ui_font")]
    pub ui_font: Handle<Font>,
}

// #[derive(AssetCollection)]
// pub struct AudioAssets {
//     #[asset(path = "audio/flying.ogg")]
//     pub flying: Handle<AudioSource>,
// }

#[derive(Resource, AssetCollection)]
pub struct TextureAssets {
    #[asset(key = "ground")]
    pub ground: Handle<Image>,
    #[asset(key = "ziggurat")]
    pub ziggurat: Handle<Image>,
    #[asset(key = "circle")]
    pub circle: Handle<Image>,
    #[asset(key = "hopper")]
    pub hopper: Handle<TextureAtlas>,
    #[asset(key = "explosion")]
    pub explosion: Handle<TextureAtlas>,
}

#[derive(Resource, AssetCollection)]
pub struct GameData {
    #[asset(path = "spawn-rates.time.toml")]
    pub spawn_rates: Handle<TimeTable>,
}
