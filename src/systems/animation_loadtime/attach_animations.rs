use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{
    Commands, Entity, Query, Res, SpriteSheetBundle, TextureAtlas, Timer, TimerMode, Transform,
    With,
};
use bevy_ecs_tilemap::prelude::TileTextureIndex;

use crate::{
    ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_START_IDX, PLAYER_ENTITY_ANIMATION_TEXTURE_START_IDX,
};
use crate::components::animation::{AnimationComponent, AnimationTimer};
use crate::components::environment::EnvironmentEntityComponents;
use crate::components::player::PlayerEntityComponents;
use crate::resources::animation::{
    AnimationResource, EnvironmentEntityAnimationResource, PlayerEntityAnimationResource,
};
use crate::resources::tiled::TileAnimationResource;

pub fn attach_animations_to_individual_tile_entities(
    mut commands: Commands,
    animation_data: Res<TileAnimationResource>,
    query: Query<(Entity, &TileTextureIndex)>,
) {
    for (entity, texture_index) in query.iter() {
        if let Some(animated_tile) = animation_data.animations.get(&texture_index.0) {
            commands.entity(entity).insert(AnimationComponent {
                start_idx: animated_tile.start_idx,
                end_idx: animated_tile.end_idx,
                speed: animated_tile.speed,
            });
            commands
                .entity(entity)
                .insert(AnimationTimer(Timer::from_seconds(
                    0.1, //TODO: look at the other attach methods to fix this magic number
                    TimerMode::Repeating,
                )))
                .insert(Name::new("TileAnimation"));
        }
    }
}

pub fn attach_base_textures_to_player_entities(
    mut commands: Commands,
    player_entity_animation_resource: Res<PlayerEntityAnimationResource>,
    query: Query<Entity, With<PlayerEntityComponents>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(SpriteSheetBundle {
            texture: player_entity_animation_resource.image_handle.clone(),
            atlas: TextureAtlas {
                layout: player_entity_animation_resource.texture_atlas.clone(),
                index: PLAYER_ENTITY_ANIMATION_TEXTURE_START_IDX,
            },
            ..Default::default()
        });
    }
}

pub fn attach_animations_to_player_entities(
    mut commands: Commands,
    overlay_animation_data: Res<AnimationResource>,
    query: Query<Entity, With<PlayerEntityComponents>>,
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
    query: Query<Entity, With<EnvironmentEntityComponents>>,
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
