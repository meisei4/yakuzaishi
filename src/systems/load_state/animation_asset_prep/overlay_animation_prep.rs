use bevy::asset::{Assets, AssetServer};
use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Entity, Query, Res, ResMut, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout,
    Transform, With,
};
use bevy::time::{Timer, TimerMode};

use crate::{
    CONTROLLABLE_ENTITY_ANIMATION_TEXTURE_START_IDX, ENVIRONMENT_TEXTURE_FILE_PATH,
    OVERLAY_ANIMATION_TEXTURE_END_IDX, OVERLAY_ANIMATION_TEXTURE_START_IDX, TILE_SIZE,
    WAKE_ANIMATION_FILE_PATH, WAKE_ANIMATION_TILE_COLUMN_LENGTH, WAKE_ANIMATION_TILE_ROW_LENGTH,
};
use crate::components::animation_components::{AnimationComponent, AnimationTimer};
use crate::components::controllable_entity_components::ControllableEntityComponents;
use crate::resources::animation_resources::{
    AnimationResource, EnvironmentalEntityAnimationResource,
};

pub fn insert_overlay_animation_resources_into_gameworld(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let overlay_animation_image = asset_server.load(WAKE_ANIMATION_FILE_PATH);

    let overlay_animation_texture_atlas = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::splat(TILE_SIZE),
        // TODO: this doesnt actual give the gradient or the index of the file for some reason, it only gives the first row ever time
        WAKE_ANIMATION_TILE_COLUMN_LENGTH,
        WAKE_ANIMATION_TILE_ROW_LENGTH,
        None,
        None,
    ));

    commands.insert_resource(AnimationResource {
        // TODO: perhaps move the initial values of these kinds of structs to a CONST module (like libs.rs)
        animation: AnimationComponent {
            start_idx: OVERLAY_ANIMATION_TEXTURE_START_IDX,
            end_idx: OVERLAY_ANIMATION_TEXTURE_END_IDX,
            speed: 0.05,
        },
        animation_image_handle: overlay_animation_image,
        animation_texture_atlas: overlay_animation_texture_atlas,
    });

    ///TODO: This is the second animation i want, but i want it attached to a non controllable entity
    let environment_animation_image_handle = asset_server.load(ENVIRONMENT_TEXTURE_FILE_PATH);

    let environment_texture_atlas_layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(Vec2::splat(TILE_SIZE), 8, 3, None, None),
    );

    commands.insert_resource(EnvironmentalEntityAnimationResource {
        animation: AnimationComponent {
            start_idx: 0,
            end_idx: 18,
            speed: 0.1,
        },
        animation_image_handle: environment_animation_image_handle,
        animation_texture_atlas: environment_texture_atlas_layout,
    });
}

//TODO: THIS IS DANGEROUS METHOD. NEEDS TO BE GENERIC FOR ALL ENTITIES INCLUDING ENVIRONMENTAL ONES
pub fn attach_overlay_animations_to_controllable_entities(
    mut commands: Commands,
    overlay_animation_data: Res<AnimationResource>,
    query: Query<Entity, With<ControllableEntityComponents>>,
) {
    for entity in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn(SpriteSheetBundle {
                    texture: overlay_animation_data.animation_image_handle.clone(),
                    atlas: TextureAtlas {
                        layout: overlay_animation_data.animation_texture_atlas.clone(),
                        index: CONTROLLABLE_ENTITY_ANIMATION_TEXTURE_START_IDX,
                    },
                    transform: Transform::default(), // gets overwritten by the parent??
                    ..Default::default()
                })
                .insert(AnimationTimer(Timer::from_seconds(
                    overlay_animation_data.animation.speed,
                    TimerMode::Repeating,
                )))
                .insert(Name::new("OverlayAnimation"));
        });
    }
}
