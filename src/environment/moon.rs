use bevy::{
    math::{Vec2, Vec3},
    prelude::{Commands, Component, Res, Resource, Transform},
    sprite::{Sprite, SpriteBundle},
    utils::default,
};
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

#[derive(Component, Debug, Clone)]
pub struct MoonLightSource {
    pub position: Vec2,
    pub intensity: f32,
    pub color: Vec3,
}

pub fn place_moon(mut commands: Commands, moon_asset: Res<MoonAsset>) {
    let moon_image = moon_asset.background.clone();
    // TODO: make a MoonLightBundle
    let moon_transform = Transform::from_xyz(0.0, 640.0 + NINTENDO_DS_SCREEN_HEIGHT, 1.0);
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::from(Vec2::new(
                        NINTENDO_DS_SCREEN_HEIGHT,
                        NINTENDO_DS_SCREEN_WIDTH,
                    )),
                    ..default()
                },
                transform: moon_transform,
                global_transform: Default::default(),
                texture: moon_image,
                visibility: Default::default(),
                inherited_visibility: Default::default(),
                view_visibility: Default::default(),
            },
            MoonLightSource {
                position: Vec2::new(moon_transform.translation.x, moon_transform.translation.y),
                intensity: 1.0,
                color: Vec3::new(1.0, 1.0, 1.0),
            },
        ))
        .insert(MoonTag);
}
