use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use tiled::{LayerType, TileLayer};

use crate::TILE_SIZE;

#[derive(TypePath, Asset)]
pub struct TiledMap {
    pub map: tiled::Map,
    pub tilemap_textures: HashMap<usize, TilemapTexture>,
}

pub fn process_tiled_maps(
    mut commands: Commands,
    map_assets: Res<Assets<TiledMap>>,
    mut map_query: Query<&Handle<TiledMap>>,
) {
    if let Some(map_handle) = map_query.iter_mut().next() {
        if let Some(tiled_map) = map_assets.get(map_handle) {
            for (tileset_index, tileset) in tiled_map.map.tilesets().iter().enumerate() {
                let Some(tilemap_texture) = tiled_map.tilemap_textures.get(&tileset_index) else {
                    log::warn!("Skipped creating layer with missing tilemap textures.");
                    continue;
                };

                let tile_spacing = TilemapSpacing {
                    x: tileset.spacing as f32,
                    y: tileset.spacing as f32,
                };
                let Some(layer) = tiled_map.map.layers().next() else {
                    // no layers???????????????
                    continue;
                };
                let LayerType::Tiles(tile_layer) = layer.layer_type() else {
                    log::info!(
                        "Skipping layer {} because only tile layers are supported.",
                        layer.id()
                    );
                    continue;
                };

                let TileLayer::Finite(layer_data) = tile_layer else {
                    log::info!(
                        "Skipping layer {} because only finite layers are supported.",
                        layer.id()
                    );
                    continue;
                };
                let map_size = TilemapSize {
                    x: tiled_map.map.width,
                    y: tiled_map.map.height,
                };
                let grid_size = TilemapGridSize {
                    x: tiled_map.map.tile_width as f32,
                    y: tiled_map.map.tile_height as f32,
                };
                let mut tile_storage = TileStorage::empty(map_size);
                let layer_entity = commands.spawn_empty().id();
                for x in 0..map_size.x {
                    for y in 0..map_size.y {
                        let layer_tile = match layer_data.get_tile(x as i32, y as i32) {
                            Some(t) => t,
                            None => {
                                continue;
                            }
                        };

                        let layer_tile_data = layer_data.get_tile_data(x as i32, y as i32);
                        let texture_index = layer_tile.id();
                        let tile_pos = TilePos { x, y };
                        let animation_tile = if texture_index == 40 {
                            AnimatedTile {
                                start: 40,
                                end: 55,
                                speed: 0.75,
                            }
                        } else {
                            AnimatedTile {
                                start: 0,
                                end: 15,
                                speed: 1.0,
                            }
                        };

                        let tile_entity = commands
                            .spawn((
                                TileBundle {
                                    position: tile_pos,
                                    tilemap_id: TilemapId(layer_entity),
                                    texture_index: TileTextureIndex(texture_index),
                                    flip: TileFlip {
                                        x: layer_tile_data.unwrap().flip_h,
                                        y: !layer_tile_data.unwrap().flip_v, //TODO still need to flip each tile vertically for some reason
                                        d: layer_tile_data.unwrap().flip_d,
                                    },
                                    ..Default::default()
                                },
                                animation_tile,
                            ))
                            .id();
                        tile_storage.set(&tile_pos, tile_entity);
                    }
                }
                let map_type = TilemapType::Square;
                commands.entity(layer_entity).insert(TilemapBundle {
                    grid_size,
                    size: map_size,
                    storage: tile_storage,
                    texture: tilemap_texture.clone(),
                    tile_size: TilemapTileSize::new(TILE_SIZE, TILE_SIZE),
                    spacing: tile_spacing,
                    map_type,
                    ..Default::default()
                });
            }
        }
    }
}
