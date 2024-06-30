use bevy::prelude::Event;
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Event)]
pub struct TileAnimationEvent {
    pub(crate) tile_pos: TilePos,
}
