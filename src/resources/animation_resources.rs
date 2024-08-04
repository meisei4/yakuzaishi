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

// TODO: Since this gets attached to the controllable entity, need to figure out a way to avoid it from being attached in certain cases
#[derive(Resource)]
pub struct AnimationResource {
    pub animation: AnimationComponent,
    pub animation_image_handle: Handle<Image>,
    pub animation_texture_atlas: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct EnvironmentalEntityAnimationResource {
    pub animation: AnimationComponent,
    pub animation_image_handle: Handle<Image>,
    pub animation_texture_atlas: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct ControlledAnimationResource {
    pub image_handle: Handle<Image>,
    pub texture_atlas: Handle<TextureAtlasLayout>,
}
