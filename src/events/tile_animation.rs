use bevy::prelude::Event;
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Event)]
pub struct TileAnimationEvent {
    pub tile_pos: TilePos,
}
