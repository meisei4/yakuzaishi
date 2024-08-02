use std::collections::HashMap;

use bevy::asset::{Asset, Handle};
use bevy::prelude::{Resource, TypePath};
use bevy_ecs_tilemap::map::TilemapTexture;

use crate::components::animation_components::AnimationComponent;

#[derive(TypePath, Asset)]
pub struct TiledMap {
    pub map: tiled::Map,
    pub tilemap_textures: HashMap<usize, TilemapTexture>,
}

#[derive(Resource)]
pub struct TiledMapResource {
    pub handle: Handle<TiledMap>,
}

#[derive(Resource)]
pub struct TileAnimationResource {
    pub animations: HashMap<u32, AnimationComponent>,
}
