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
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Menu)
                    .load_collection::<FontAssets>()
                    .load_collection::<TextureAssets>()
                    .load_collection::<AudioAssets>()
                    .load_collection::<GameData>()
                                                    // .build(app)
            );
    }
}

#[derive(Resource, AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FantasqueSansMono-Bold.ttf")]
    pub fantasque_sans: Handle<Font>,
}

#[derive(Resource, AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/snare_hit_1.ogg")]
    pub snare_hit_1: Handle<AudioSource>,
    #[asset(path = "audio/snare_hit_2.ogg")]
    pub snare_hit_2: Handle<AudioSource>,
    #[asset(path = "audio/snare_hit_3.ogg")]
    pub snare_hit_3: Handle<AudioSource>,
    #[asset(path = "audio/snare_hit_max.ogg")]
    pub snare_hit_max: Handle<AudioSource>,
}

#[derive(Resource, AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/ground.png")]
    pub ground: Handle<Image>,
    #[asset(path = "textures/ziggurat.png")]
    pub ziggurat: Handle<Image>,
    #[asset(path = "textures/circle.png")]
    pub circle: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 72.0, tile_size_y = 72.0, columns = 4, rows = 1))]
    pub hopper_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "textures/hopper.png")]
    pub hopper: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 72.0, tile_size_y = 72.0, columns = 8, rows = 1))]
    pub explosion_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "textures/explosion.png")]
    pub explosion: Handle<Image>,
}

#[derive(Resource, AssetCollection)]
pub struct GameData {
    #[asset(path = "spawn-rates.time.toml")]
    pub spawn_rates: Handle<TimeTable>,
}
