use bevy::prelude::{EventReader, EventWriter, Query, Res, Time};
use bevy_ecs_tilemap::prelude::TileTextureIndex;
use bevy_ecs_tilemap::tiles::TilePos;
use tracy_client::span;

use crate::components::animation_components::{AnimationComponent, AnimationTimer};
use crate::components::controllable_entity_components::ControllableEntityComponents;
use crate::events::tile_animation::TileAnimationEvent;

pub fn animate_overlapped_tile_event_based(
    mut entity_query: Query<&ControllableEntityComponents>,
    mut overlap_event_writer: EventWriter<TileAnimationEvent>,
) {
    let _span = span!("tile animation_asset_prep event send");
    if let Ok(entity) = entity_query.get_single_mut() {
        let current_tile_pos = TilePos {
            x: entity.tile_coordinate_position.x as u32,
            y: entity.tile_coordinate_position.y as u32,
        };
        overlap_event_writer.send(TileAnimationEvent {
            tile_pos: current_tile_pos,
        });
    }
}

pub fn handle_overlap_event(
    time: Res<Time>,
    mut event_reader: EventReader<TileAnimationEvent>,
    mut tile_query: Query<(
        &TilePos,
        &mut AnimationTimer,
        &AnimationComponent,
        &mut TileTextureIndex,
    )>,
) {
    let _span = span!("tile animation_asset_prep event read");
    for event in event_reader.read() {
        for (tile_pos, mut animation_timer, animated_tile, mut tilemap_texture_index) in
            tile_query.iter_mut()
        {
            if *tile_pos == event.tile_pos {
                animation_timer.tick(time.delta());
                if animation_timer.just_finished() {
                    tilemap_texture_index.0 = if tilemap_texture_index.0 == animated_tile.end_idx {
                        animated_tile.start_idx
                    } else {
                        tilemap_texture_index.0 + 1
                    };
                }
            }
        }
    }
}

// TODO: decide which system makes more sense, also this is to provide a comparison between continuous and event based

pub fn animate_overlapped_tile_continuous(
    time: Res<Time>,
    mut entity_query: Query<&ControllableEntityComponents>,
    mut tile_query: Query<(
        &TilePos,
        &mut AnimationTimer,
        &AnimationComponent,
        &mut TileTextureIndex,
    )>,
) {
    let _span = span!("tile animation_asset_prep continuous");
    if let Ok(entity) = entity_query.get_single_mut() {
        let current_tile_pos = TilePos {
            x: entity.tile_coordinate_position.x as u32,
            y: entity.tile_coordinate_position.y as u32,
        };

        for (tile_pos, mut animation_timer, animated_tile, mut tilemap_texture_index) in
            tile_query.iter_mut()
        {
            if *tile_pos == current_tile_pos {
                animation_timer.tick(time.delta());
                if animation_timer.just_finished() {
                    tilemap_texture_index.0 = if tilemap_texture_index.0 == animated_tile.end_idx {
                        animated_tile.start_idx
                    } else {
                        tilemap_texture_index.0 + 1
                    };
                }
            }
        }
    }
}
