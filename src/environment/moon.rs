use bevy::prelude::{Commands, Component, Res, Resource, Transform};
use bevy::sprite::SpriteBundle;
use bevy::utils::default;
use bevy_asset::Handle;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_render::texture::Image;

use crate::NINTENDO_DS_SCREEN_HEIGHT;

#[derive(Component, Default)]
pub struct MoonTag;

#[derive(AssetCollection, Resource)]
pub struct MoonAsset {
    #[asset(path = "sprite_data/ollie_bacon_daytime_moon.png")]
    pub background: Handle<Image>,
}

pub fn place_moon(mut commands: Commands, moon_asset: Res<MoonAsset>) {
    let moon_image = moon_asset.background.clone();
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(0.0, 640.0 + NINTENDO_DS_SCREEN_HEIGHT, 3.0),
            texture: moon_image,
            ..default()
        })
        .insert(MoonTag);
}
