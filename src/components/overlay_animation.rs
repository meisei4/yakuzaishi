use bevy::asset::Handle;
use bevy::prelude::{Component, TextureAtlasLayout};

// TODO: merge this and the AnimationData component???? (all that component and resource stuff needs to be organized better
// TODO: SERIOUSLY FIGURE OUT HOW TO ORGANIZE THESE KINDS OF OBJECTS BETTER
#[derive(Component, Clone, Copy)]
pub struct OverlayAnimation {
    pub start_idx: i32,
    pub end_idx: i32,
    pub speed: f32,
    pub z_index: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationType {
    Wake,
    Splash,
}

#[derive(Component)]
pub struct OverlayAnimationTextureAtlas {
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
    pub index: usize,
}
