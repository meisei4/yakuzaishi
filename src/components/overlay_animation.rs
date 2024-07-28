use std::collections::HashMap;

use bevy::prelude::{Component, Handle, Resource};
use bevy::sprite::TextureAtlas;
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Component)]
pub struct OverlayAnimation {
    pub tile_pos: TilePos,
    pub animation_type: AnimationType,
    pub z_index: f32,
    pub start_idx: i32,
    pub end_idx: i32,
    pub speed: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationType {
    Wake,
    Splash,
}

#[derive(Resource)]
pub struct OverlayAnimationData {
    // TODO: not sure how to organize this animations attribute and the hashmap key
    pub animations: HashMap<AnimationType, OverlayAnimation>,
    pub wake_texture: Handle<TextureAtlas>,
    pub splash_texture: Handle<TextureAtlas>,
}
