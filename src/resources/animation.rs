use bevy::asset::Handle;
use bevy::prelude::{Image, Resource, TextureAtlasLayout};
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::components::animation::AnimationComponent;

#[derive(Resource)]
pub struct AnimationResource {
    pub animation: AnimationComponent,
    pub animation_image_handle: Handle<Image>,
    pub animation_texture_atlas: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct EnvironmentEntityAnimationResource {
    pub animation: AnimationComponent,
    pub animation_image_handle: Handle<Image>,
    pub animation_texture_atlas: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct PlayerEntityAnimationResource {
    pub image_handle: Handle<Image>,
    pub texture_atlas: Handle<TextureAtlasLayout>,
}

// TODO: NEW VERSION
#[derive(AssetCollection, Resource)]
pub struct PlayerEntityAnimationAssets {
    #[asset(path = "sprite_data/iruka.png")]
    pub image_handle: Handle<Image>,
    pub texture_atlas: Handle<TextureAtlasLayout>,
}
