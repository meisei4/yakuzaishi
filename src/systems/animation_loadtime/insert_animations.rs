use bevy::asset::Assets;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Res, ResMut, TextureAtlasLayout};

use crate::{
    ENVIRONMENT_ENTITY_ANIMATION_SPEED, ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_COLUMN_LENGTH,
    ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_END_IDX, ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_ROW_LENGTH,
    ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_START_IDX, TILE_ANIMATION_SPEED,
    TILE_ANIMATION_TEXTURE_END_IDX, TILE_ANIMATION_TEXTURE_START_IDX, TILE_SIZE,
    WAKE_ANIMATION_SPEED, WAKE_ANIMATION_TEXTURE_COLUMN_LENGTH, WAKE_ANIMATION_TEXTURE_END_IDX,
    WAKE_ANIMATION_TEXTURE_ROW_LENGTH, WAKE_ANIMATION_TEXTURE_START_IDX,
};
use crate::components::animation::AnimationComponent;
use crate::resources::animation::{
    AnimationAssets, AnimationResource, EnvironmentEntityAnimationAssets,
    EnvironmentEntityAnimationResource,
};
use crate::resources::tiled::TileAnimationResource;

pub fn insert_tile_animation_resources_into_world(mut commands: Commands) {
    let animation = AnimationComponent {
        start_idx: TILE_ANIMATION_TEXTURE_START_IDX,
        end_idx: TILE_ANIMATION_TEXTURE_END_IDX,
        speed: TILE_ANIMATION_SPEED,
    };
    //TODO: no idea why i have a hashmap here with the texture start idx as key, there was a reason when i first wrote it though
    let animation_data = TileAnimationResource { animation };
    commands.insert_resource(animation_data);
}

//TODO: Split this method into two different systems, and ADD MARKERS for overlay vs environmental animation
// and ADD MARKERS!
// and ADD MARKERS!
// and ADD MARKERS to distinguish the resource or asset type (or even just Entity... i dont know)
pub fn insert_overlay_animation_resources_into_world(
    mut commands: Commands,
    animation_assets: Res<AnimationAssets>,
    environment_entity_animation_assets: Res<EnvironmentEntityAnimationAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let overlay_animation_image = animation_assets.animation_image_handle.clone();

    let overlay_animation_texture_atlas = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::splat(TILE_SIZE),
        // TODO: this doesnt actual give the gradient or the index of the file for some reason, it only gives the first row ever time
        WAKE_ANIMATION_TEXTURE_COLUMN_LENGTH,
        WAKE_ANIMATION_TEXTURE_ROW_LENGTH,
        None,
        None,
    ));

    commands.insert_resource(AnimationResource {
        animation: AnimationComponent {
            start_idx: WAKE_ANIMATION_TEXTURE_START_IDX,
            end_idx: WAKE_ANIMATION_TEXTURE_END_IDX,
            speed: WAKE_ANIMATION_SPEED,
        },
        animation_image_handle: overlay_animation_image,
        animation_texture_atlas: overlay_animation_texture_atlas,
    });

    let environment_animation_image_handle = environment_entity_animation_assets
        .animation_image_handle
        .clone();

    let environment_texture_atlas_layout =
        texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            Vec2::splat(TILE_SIZE),
            ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_COLUMN_LENGTH,
            ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_ROW_LENGTH,
            None,
            None,
        ));

    commands.insert_resource(EnvironmentEntityAnimationResource {
        animation: AnimationComponent {
            start_idx: ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_START_IDX,
            end_idx: ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_END_IDX,
            speed: ENVIRONMENT_ENTITY_ANIMATION_SPEED,
        },
        animation_image_handle: environment_animation_image_handle,
        animation_texture_atlas: environment_texture_atlas_layout,
    });
}
