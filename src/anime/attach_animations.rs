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
use crate::anime::anime_component::AnimationTimer;
use crate::anime::anime_res::{
    AnimationResource, EnvironmentEntityAnimationResource, PlayerEntityAnimationResource,
};
use crate::kinetic_entity::{EnvironmentEntityTag, PlayerEntityTag};
use crate::map::tiled::TileAnimationResource;

pub fn attach_animations_to_individual_tile_entities(
    mut commands: Commands,
    animation_data: Res<TileAnimationResource>,
    query: Query<(Entity, &TileTextureIndex)>,
) {
    for (entity, texture_index) in query.iter() {
        //TODO: this is horrendous, look into how to add the anime data directly upon tile processing
        if texture_index.0 == animation_data.animation.start_idx {
            commands
                .entity(entity)
                .insert(animation_data.animation.clone());
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

// TODO: BIG TODO, I dont think the whole "insert, attach" is at all a proper decoupling. its overkill,
pub fn attach_base_textures_to_player_entities(
    mut commands: Commands,
    player_entity_animation_resource: Res<PlayerEntityAnimationResource>,
    query: Query<Entity, With<PlayerEntityTag>>,
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
