use crate::{events::TimeTable, GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
// use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TomlAssetPlugin::<TimeTable>::new(&["time.toml"]))
            .add_loading_state(
                LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
            )
            .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
            .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
            .add_collection_to_loading_state::<_, GameData>(GameState::Loading);
        // .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
    }
}

#[derive(Resource, AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FantasqueSansMono-Bold.ttf")]
    pub fantasque_sans: Handle<Font>,
}

// #[derive(AssetCollection)]
// pub struct AudioAssets {
//     #[asset(path = "audio/flying.ogg")]
//     pub flying: Handle<AudioSource>,
// }

#[derive(Resource, AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/ground.png")]
    pub ground: Handle<Image>,
    #[asset(path = "textures/ziggurat.png")]
    pub ziggurat: Handle<Image>,
    #[asset(path = "textures/circle.png")]
    pub circle: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 72.0, tile_size_y = 72.0, columns = 4, rows = 1))]
    #[asset(path = "textures/hopper.png")]
    pub hopper: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 72.0, tile_size_y = 72.0, columns = 8, rows = 1))]
    #[asset(path = "textures/explosion.png")]
    pub explosion: Handle<TextureAtlas>,
}

#[derive(Resource, AssetCollection)]
pub struct GameData {
    #[asset(path = "spawn-rates.time.toml")]
    pub spawn_rates: Handle<TimeTable>,
}
