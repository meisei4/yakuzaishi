use bevy::asset::Handle;
use bevy::prelude::{Image, Resource, TextureAtlasLayout};

use crate::components::animation_components::AnimationComponent;

//TODO: Ticket # 16 i have no idea if i should be wrapping up Components in Resources, think about it
// from bevy:
// Resources allow you to store a single global instance of some data type, independently of entities.
// - Use them for data that is truly global for your app, such as configuration/settings.
// - Resources make it easy for you to access such data from anywhere.
// Examples from prelude:
// - TextureAtlas - (AKA a sprite sheet in other gamedev circles) makes sense because these are
//        pretty much  that should be reused throughout the game world
// - Handle (no idea why yet, but they are pointers so I think its to allow you to make your own?)

#[derive(Resource)]
pub struct OverlayAnimationResource {
    pub wake_animation: AnimationComponent,
    pub overlay_animation_image_handle: Handle<Image>,
    pub overlay_animation_texture_atlas: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct ControlledAnimationResource {
    pub controlled_animation_image_handle: Handle<Image>,
    pub controlled_animation_texture_atlas: Handle<TextureAtlasLayout>,
}
