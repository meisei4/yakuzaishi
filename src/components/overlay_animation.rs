use bevy::prelude::{Component, Handle, Image, Resource, TextureAtlasLayout};

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

#[derive(Resource)]
pub struct OverlayAnimationData {
    // TODO: this is all explicitly written out not using any Hashmaps or anything, maybe add hashmap structure later
    pub wake_animation: OverlayAnimation,
    pub splash_animation: OverlayAnimation,
    pub wake_texture_handle: Handle<Image>,
    pub splash_texture_handle: Handle<Image>,
    pub wake_texture_layout: Handle<TextureAtlasLayout>,
    pub splash_texture_layout: Handle<TextureAtlasLayout>,
}
