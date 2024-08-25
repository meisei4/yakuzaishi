use bevy::audio::{AudioBundle, PlaybackSettings};
use bevy::prelude::{Commands, Res};

use crate::resources::audio::AudioAssets;

pub fn start_background_audio(mut commands: Commands, audio_assets: Res<AudioAssets>) {
    commands.spawn(AudioBundle {
        source: audio_assets.background.clone(),
        settings: PlaybackSettings::LOOP,
    });
}
