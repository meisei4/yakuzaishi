use bevy::prelude::{EventReader, EventWriter, Query, Res, Time};
use bevy_ecs_tilemap::prelude::TileTextureIndex;
use bevy_ecs_tilemap::tiles::TilePos;
use tracy_client::span;

use crate::components::animation_timer::AnimationTimer;
use crate::components::flying_entity_components::FlyingEntityComponents;
use crate::events::tile_animation::TileAnimationEvent;
use crate::systems::load_state::setup_map_animation_data::AnimatedTile;

pub fn animate_overlapped_tile_event_based(
    mut entity_query: Query<&FlyingEntityComponents>,
    mut overlap_event_writer: EventWriter<TileAnimationEvent>,
) {
    let _span = span!("tile animation event send");
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
        &AnimatedTile,
        &mut TileTextureIndex,
    )>,
) {
    let _span = span!("tile animation event read");
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

pub fn animate_overlapped_tile_continuous(
    time: Res<Time>,
    //mut entity_query: Query<&RotationalVehicleComponents>,
    mut entity_query: Query<&FlyingEntityComponents>,
    mut tile_query: Query<(
        &TilePos,
        &mut AnimationTimer,
        &AnimatedTile,
        &mut TileTextureIndex,
    )>,
) {
    let _span = span!("tile animation continuous");
    if let Ok(entity) = entity_query.get_single_mut() {
        let current_tile_pos = TilePos {
            x: entity.tile_coordinate_position.x as u32,
            y: entity.tile_coordinate_position.y as u32,
        };

        for (tile_pos, mut animation_timer, animated_tile, mut tilemap_texture_index) in
            tile_query.iter_mut()
        {
            if *tile_pos == current_tile_pos {
                // Tick the tile's animation timer while the vehicle is on the tile
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
