use bevy::{
    audio::{AudioBundle, PlaybackSettings},
    prelude::{Commands, Res},
};

use crate::audio::audio_res::AudioAssets;

pub fn start_background_audio(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn(AudioBundle {
        source: audio_assets.background.clone(),
        settings: PlaybackSettings::LOOP,
    });
}
