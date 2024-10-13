use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Res, Resource, Transform};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::utils::default;
use bevy_asset::Handle;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_render::texture::Image;

use crate::{NINTENDO_DS_SCREEN_HEIGHT, NINTENDO_DS_SCREEN_WIDTH};

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
            sprite: Sprite {
                custom_size: Option::from(Vec2::new(
                    NINTENDO_DS_SCREEN_HEIGHT,
                    NINTENDO_DS_SCREEN_WIDTH,
                )),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 640.0 + NINTENDO_DS_SCREEN_HEIGHT, 1.0),
            global_transform: Default::default(),
            texture: moon_image,
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
        })
        .insert(MoonTag);
}
