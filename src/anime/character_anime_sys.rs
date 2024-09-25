use bevy::asset::Assets;
use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Entity, Query, Res, ResMut, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout,
    Time, Timer, TimerMode, Transform, With,
};

use crate::{
    ENVIRONMENT_ENTITY_ANIMATION_SPEED, ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_COLUMN_LENGTH,
    ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_END_IDX, ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_ROW_LENGTH,
    ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_START_IDX, PLAYER_ENTITY_ANIMATION_TEXTURE_START_IDX,
    TILE_SIZE, WAKE_ANIMATION_SPEED, WAKE_ANIMATION_TEXTURE_COLUMN_LENGTH,
    WAKE_ANIMATION_TEXTURE_END_IDX, WAKE_ANIMATION_TEXTURE_ROW_LENGTH,
    WAKE_ANIMATION_TEXTURE_START_IDX,
};
use crate::anime::anime_component::{AnimationComponent, AnimationTimer};
use crate::anime::anime_res::{
    AnimationAssets, AnimationResource, EnvironmentEntityAnimationAssets,
    EnvironmentEntityAnimationResource,
};
use crate::kinetic_entity::{EnvironmentEntityTag, PlayerEntityTag};

// TODO: BIG TODO, I dont think the whole "insert, attach" is at all a proper decoupling. its overkill,
pub fn attach_animations_to_player_entities(
    mut commands: Commands,
    overlay_animation_data: Res<AnimationResource>,
    query: Query<Entity, With<PlayerEntityTag>>,
) {
    for entity in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn(SpriteSheetBundle {
                    texture: overlay_animation_data.animation_image_handle.clone(),
                    atlas: TextureAtlas {
                        layout: overlay_animation_data.animation_texture_atlas.clone(),
                        index: PLAYER_ENTITY_ANIMATION_TEXTURE_START_IDX,
                    },
                    transform: Transform::default(), // gets overwritten by the parent??
                    ..Default::default()
                })
                .insert(AnimationTimer(Timer::from_seconds(
                    overlay_animation_data.animation.speed,
                    TimerMode::Repeating,
                )))
                .insert(Name::new("PlayerEntityOverlayAnimation"));
        });
    }
}

pub fn attach_animations_to_environment_entities(
    mut commands: Commands,
    overlay_animation_data: Res<EnvironmentEntityAnimationResource>,
    query: Query<Entity, With<EnvironmentEntityTag>>,
) {
    for entity in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn(SpriteSheetBundle {
                    texture: overlay_animation_data.animation_image_handle.clone(),
                    atlas: TextureAtlas {
                        layout: overlay_animation_data.animation_texture_atlas.clone(),
                        index: ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_START_IDX as usize, //TODO see lib.rs todo about deciding datatypes
                    },
                    transform: Transform::default(), // gets overwritten by the parent??
                    ..Default::default()
                })
                .insert(AnimationTimer(Timer::from_seconds(
                    overlay_animation_data.animation.speed,
                    TimerMode::Repeating,
                )))
                .insert(Name::new("EnvironmentEntityOverlayAnimation"));
        });
    }
}

//TODO: Split this method into two different systems, and ADD MARKERS for overlay vs environmental anime
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

pub fn animate_overlay_animations(
    time: Res<Time>,
    overlay_animation_data: Res<AnimationResource>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (mut timer, mut overlay_atlas) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            //TODO: Is there some sort of implied Copy and or Clone happening here? I dont know why when
            // AnimationComponent doesnt derive copy or clone this next line doesnt work
            let animation = overlay_animation_data.animation;

            let current_index = overlay_atlas.index;

            let next_index = if current_index == animation.end_idx as usize {
                animation.start_idx as usize
            } else {
                current_index + 1
            };

            overlay_atlas.index = next_index;
        }
    }
}

pub fn animate_env_entity_animations(
    time: Res<Time>,
    overlay_animation_data: Res<EnvironmentEntityAnimationResource>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (mut timer, mut overlay_atlas) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            //TODO: Is there some sort of implied Copy and or Clone happening here? I dont know why when
            // AnimationComponent doesnt derive copy or clone this next line doesnt work
            let animation = overlay_animation_data.animation;

            let current_index = overlay_atlas.index;

            let next_index = if current_index == animation.end_idx as usize {
                animation.start_idx as usize
            } else {
                current_index + 1
            };

            overlay_atlas.index = next_index;
        }
    }
}
