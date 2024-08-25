use bevy::asset::{AssetServer, Handle};
use bevy::audio::AudioSource;
use bevy::prelude::{Commands, Res, Resource};
use bevy_asset_loader::asset_collection::AssetCollection;

//TODO: This is the correct bevy_asset_loader usage
#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio_data/samurai_break.wav")]
    pub background: Handle<AudioSource>,
}

//TODO: here is how I would have been doing it:

pub fn add_audio_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let audio_handle_preform = asset_server.load("audio_data/samurai.ogg");

    commands.insert_resource(AudioAssets {
        background: audio_handle_preform,
    });
}
