use bevy::asset::Handle;
use bevy::prelude::{Image, Resource};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct OverlayAnimationAssets {
    #[asset(path = "sprite_data/random_test_animations.png")]
    //WAKE_ANIMATION_FILE_PATH
    pub animation_image_handle: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct EnvironmentEntityAnimationAssets {
    #[asset(path = "sprite_data/Ikiikiiruka.png")]
    pub animation_image_handle: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct PlayerEntityAnimationAssets {
    #[asset(path = "sprite_data/iruka.png")]
    // PLAYER_ENTITY_TEXTURE_FILE_PATH
    pub image_handle: Handle<Image>,
}
