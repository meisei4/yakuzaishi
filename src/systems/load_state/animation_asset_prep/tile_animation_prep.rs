use std::collections::HashMap;

use bevy::core::Name;
use bevy::prelude::{Commands, Entity, Query, Res};
use bevy::time::{Timer, TimerMode};
use bevy_ecs_tilemap::tiles::TileTextureIndex;

use crate::components::animation_components::{AnimationComponent, AnimationTimer};
use crate::resources::tiled_resources::TileAnimationResource;

pub fn insert_tile_animation_resources_into_gameworld(mut commands: Commands) {
    let mut animations = HashMap::new();

    let animated_tile = AnimationComponent {
        start_idx: 40,
        end_idx: 54,
        speed: 0.50,
    };

    animations.insert(40, animated_tile);
    let animation_data = TileAnimationResource { animations };
    commands.insert_resource(animation_data);
}

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
                    0.1,
                    TimerMode::Repeating,
                )))
                .insert(Name::new("TileAnimation"));
        }
    }
}
