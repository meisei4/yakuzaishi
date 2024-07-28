use std::collections::HashMap;

use bevy::prelude::{Commands, Entity, Query, Res};
use bevy::time::{Timer, TimerMode};
use bevy_ecs_tilemap::tiles::TileTextureIndex;

use crate::components::animated_tile::AnimatedTile;
use crate::components::animation_timer::AnimationTimer;
use crate::resources::animation_data::AnimationData;

pub fn setup_map_animation_data(mut commands: Commands) {
    let mut animations = HashMap::new();

    let animated_tile = AnimatedTile {
        start_idx: 40,
        end_idx: 54,
        speed: 0.50,
    };

    animations.insert(40, animated_tile);
    let animation_data = AnimationData { animations };
    commands.insert_resource(animation_data);
}

pub fn attach_animations_to_map(
    mut commands: Commands,
    query: Query<(Entity, &TileTextureIndex)>,
    animation_data: Res<AnimationData>,
) {
    for (entity, texture_index) in query.iter() {
        if let Some(animated_tile) = animation_data.animations.get(&texture_index.0) {
            commands.entity(entity).insert(AnimatedTile {
                start_idx: animated_tile.start_idx,
                end_idx: animated_tile.end_idx,
                speed: animated_tile.speed,
            });
            commands
                .entity(entity)
                .insert(AnimationTimer(Timer::from_seconds(
                    0.1,
                    TimerMode::Repeating,
                )));
        }
    }
}
