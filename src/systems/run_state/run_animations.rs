use bevy::prelude::{Query, Res, Time};
use bevy_ecs_tilemap::prelude::TileTextureIndex;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::components::flying_entity_components::FlyingEntityComponents;
use crate::systems::load_state::load_animations::{AnimatedTile, AnimationTimer};

pub fn animate_overlapped_tile(
    time: Res<Time>,
    mut flying_entity_query: Query<&FlyingEntityComponents>,
    mut tile_query: Query<(
        &TilePos,
        &mut AnimationTimer,
        &AnimatedTile,
        &mut TileTextureIndex,
    )>,
) {
    if let Ok(flying_entity) = flying_entity_query.get_single_mut() {
        let current_tile_pos = TilePos {
            x: flying_entity.tile_coordinate_position.x as u32,
            y: flying_entity.tile_coordinate_position.y as u32,
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
