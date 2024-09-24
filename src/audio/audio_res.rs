use bevy::asset::Handle;
use bevy::audio::AudioSource;
use bevy::prelude::Resource;
use bevy_asset_loader::asset_collection::AssetCollection;

//TODO: This is the correct bevy_asset_loader,
#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio_data/samurai.ogg")]
    pub background: Handle<AudioSource>,
}