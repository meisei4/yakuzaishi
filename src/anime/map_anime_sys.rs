use bevy::core::Name;
use bevy::math::Vec3;
use bevy::prelude::{
    Commands, Entity, Event, EventReader, EventWriter, Query, Res, Time, Timer, TimerMode,
    Transform, With,
};
use bevy_ecs_tilemap::prelude::TileTextureIndex;
use bevy_ecs_tilemap::tiles::TilePos;
use tracy_client::span;

use crate::{
    TILE_ANIMATION_SPEED, TILE_ANIMATION_TEXTURE_END_IDX, TILE_ANIMATION_TEXTURE_START_IDX,
    TILE_SIZE,
};
use crate::anime::anime_component::{AnimationComponent, AnimationTimer};
use crate::anime::anime_res::TileAnimationResource;
use crate::kinetic_entity::PlayerEntityTag;

#[derive(Event)]
pub struct TileAnimationEvent {
    pub tile_pos: TilePos,
}

pub fn animate_overlapped_tiles_event_based(
    mut entity_query: Query<&Transform, With<PlayerEntityTag>>,
    mut overlap_event_writer: EventWriter<TileAnimationEvent>,
) {
    let _span = span!("tile animation_loadtime event send");
    for player_entity_transform in entity_query.iter_mut() {
        let current_tile_pos = calc_tile_pos(&player_entity_transform.translation);
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
    let _span = span!("tile animation_loadtime event read");

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
