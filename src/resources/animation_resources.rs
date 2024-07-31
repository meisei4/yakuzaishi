use std::collections::HashMap;

use bevy::asset::Handle;
use bevy::prelude::{Image, Resource};

use crate::components::animated_tile::AnimatedTile;
use crate::components::overlay_animation::{OverlayAnimation, OverlayAnimationTextureAtlas};

// TODO: SERIOUSLY FIGURE OUT HOW TO ORGANIZE THESE KINDS OF OBJECTS BETTER

#[derive(Resource)]
pub struct TileAnimationData {
    pub animations: HashMap<u32, AnimatedTile>,
}

#[derive(Resource)]
pub struct OverlayAnimationData {
    // TODO: this is all explicitly written out not using any Hashmaps or anything, maybe add hashmap structure later
    pub wake_animation: OverlayAnimation,
    pub overlay_animation_image_handle: Handle<Image>, //TODO: wrap this just like the OverlayAnimationTextureAtlas???
    pub overlay_animation_texture_atlas: OverlayAnimationTextureAtlas,
}
