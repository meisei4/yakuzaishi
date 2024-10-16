use bevy::{asset::Handle, audio::AudioSource, prelude::Resource};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio_data/samurai.ogg")]
    pub background: Handle<AudioSource>,
}
