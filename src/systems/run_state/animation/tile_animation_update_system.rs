use bevy::math::Vec3;
use bevy::prelude::{EventReader, EventWriter, Query, Res, Time, Transform, With};
use bevy_ecs_tilemap::prelude::TileTextureIndex;
use bevy_ecs_tilemap::tiles::TilePos;
use tracy_client::span;

use crate::components::animation_components::{AnimationComponent, AnimationTimer};
use crate::components::controllable_entity_components::VelocityVectorComponents;
use crate::events::tile_animation::TileAnimationEvent;
use crate::TILE_SIZE;

pub fn animate_overlapped_tile_event_based(
    mut entity_query: Query<&Transform, With<VelocityVectorComponents>>,
    mut overlap_event_writer: EventWriter<TileAnimationEvent>,
) {
    let _span = span!("tile animation_asset_prep event send");
    for controllable_entity_transform in entity_query.iter_mut() {
        let current_tile_pos = calc_tile_pos(&controllable_entity_transform.translation);
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

fn calc_tile_pos(translation: &Vec3) -> TilePos {
    let tile_x = ((translation.x + (TILE_SIZE / 2.0)) / TILE_SIZE).floor();
    let tile_y = ((translation.y + (TILE_SIZE / 2.0)) / TILE_SIZE).floor();
    return TilePos {
        x: tile_x as u32,
        y: tile_y as u32,
    };
}
