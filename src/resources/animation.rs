use bevy::asset::Handle;
use bevy::prelude::{Image, Resource, TextureAtlasLayout};
use bevy_asset_loader::asset_collection::AssetCollection;

use crate::components::animation::AnimationComponent;

#[derive(AssetCollection, Resource)]
pub struct AnimationAssets {
    #[asset(path = "sprite_data/random_test_animations.png")]
    //WAKE_ANIMATION_FILE_PATH
    pub animation_image_handle: Handle<Image>,
}

#[derive(Resource)]
pub struct AnimationResource {
    pub animation: AnimationComponent,
    pub animation_image_handle: Handle<Image>,
    pub animation_texture_atlas: Handle<TextureAtlasLayout>,
}

#[derive(AssetCollection, Resource)]
pub struct EnvironmentEntityAnimationAssets {
    #[asset(path = "sprite_data/Ikiikiiruka.png")]
    pub animation_image_handle: Handle<Image>,
}

#[derive(Resource)]
pub struct EnvironmentEntityAnimationResource {
    pub animation: AnimationComponent,
    pub animation_image_handle: Handle<Image>,
    pub animation_texture_atlas: Handle<TextureAtlasLayout>,
}

// TODO: Figure out all the TextureAtlas nonsense aswell as the recoupling of "Insert" and "Attach"...
#[derive(AssetCollection, Resource)]
pub struct PlayerEntityAnimationAssets {
    #[asset(path = "sprite_data/iruka.png")]
    // PLAYER_ENTITY_TEXTURE_FILE_PATH
    pub image_handle: Handle<Image>,
}

//TODO: at somepoint the below will be removable see above TODO^^
#[derive(Resource)]
pub struct PlayerEntityAnimationResource {
    pub image_handle: Handle<Image>,
    pub texture_atlas: Handle<TextureAtlasLayout>,
}
