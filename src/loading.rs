use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
// use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                // .with_collection::<AudioAssets>()
                // .with_collection::<TextureAssets>()
                .continue_to_state(GameState::Menu),
        );
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FantasqueSansMono-Bold.ttf")]
    pub fantasque_sans: Handle<Font>,
}

// #[derive(AssetCollection)]
// pub struct AudioAssets {
//     #[asset(path = "audio/flying.ogg")]
//     pub flying: Handle<AudioSource>,
// }

// #[derive(AssetCollection)]
// pub struct TextureAssets {
//     #[asset(path = "textures/bevy.png")]
//     pub texture_bevy: Handle<Image>,
// }
