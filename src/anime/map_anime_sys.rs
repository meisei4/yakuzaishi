use bevy::log::info;
use bevy::math::Vec3;
use bevy::prelude::{Event, EventReader, EventWriter, Query, Res, Time, With};
use bevy::utils::info;
use bevy_ecs_tilemap::prelude::TileTextureIndex;
use bevy_ecs_tilemap::tiles::TilePos;
use tracy_client::span;

use crate::anime::anime_components::{AnimationComponent, AnimationTimer};
use crate::kinetic_components::{KineticEntityComponents, PlayerEntityTag};
use crate::map::tiled_components::TileEntityTag;
use crate::TILE_SIZE;

#[derive(Event)]
pub struct TileAnimationEvent {
    pub tile_pos: TilePos,
}

pub fn animate_overlapped_tiles_event_based(
    mut entity_query: Query<&KineticEntityComponents, With<PlayerEntityTag>>,
    mut overlap_event_writer: EventWriter<TileAnimationEvent>,
) {
    let _span = span!("tile animation_loadtime event send");
    for player_entity in entity_query.iter_mut() {
        let current_tile_pos = calc_tile_pos(&player_entity.position);
        overlap_event_writer.send(TileAnimationEvent {
            tile_pos: current_tile_pos,
        });
    }
}

pub fn handle_overlap_event(
    time: Res<Time>,
    mut event_reader: EventReader<TileAnimationEvent>,
    mut tile_query: Query<
        (
            &TilePos,
            &mut AnimationTimer,
            &AnimationComponent,
            //TODO: at somepoint shouldnt i be able to convert all texture stuff to TextureAtlases?
            &mut TileTextureIndex,
        ),
        With<TileEntityTag>,
    >,
) {
    let _span = span!("tile animation_loadtime event read");

    for event in event_reader.read() {
        for (tile_pos, mut animation_timer, animation, mut tilemap_texture_index) in
            tile_query.iter_mut()
        {
            if *tile_pos == event.tile_pos {
                animation_timer.tick(time.delta());
                if animation_timer.just_finished() {
                    tilemap_texture_index.0 = if tilemap_texture_index.0 == animation.end_idx {
                        animation.start_idx
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
