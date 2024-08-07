use bevy::asset::Assets;
use bevy::core::Name;
use bevy::prelude::{AssetServer, Bundle, Commands, Entity, Handle, Query, Res};
use bevy_ecs_tilemap::map::{
    TilemapGridSize, TilemapId, TilemapSize, TilemapSpacing, TilemapTileSize, TilemapType,
};
use bevy_ecs_tilemap::prelude::{TilePos, TileStorage};
use bevy_ecs_tilemap::TilemapBundle;
use bevy_ecs_tilemap::tiles::{TileBundle, TileTextureIndex};
use tiled::{LayerType, TileLayer};

use crate::{TILE_MAP_FILE_PATH, TILE_SIZE};
use crate::resources::tiled::TiledMap;

#[derive(Default, Bundle)]
pub struct TiledMapBundle {
    pub tiled_map: Handle<TiledMap>,
}

pub fn spawn_tiled_map_entity(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load(TILE_MAP_FILE_PATH);

    //TODO: If I dont spawn something here, then nothing shows up. But I still need to decouple.
    // I dont like the word spawn, i just want to insert a map resource into the gameworld to be honest
    commands
        .spawn(TiledMapBundle {
            tiled_map: map_handle,
            ..Default::default()
        })
        .insert(Name::new("TiledMap Bundle Entity"));
}

//TODO: this is alot from the custom solution from bevy_ecs_tiled, so this needs to be looked at later

pub fn process_tiled_maps(
    mut commands: Commands,
    map_assets: Res<Assets<TiledMap>>, //TODO: learn about all the plural Assets (including TextureAtlasLayouts etc)
    mut map_query: Query<&Handle<TiledMap>>, // TODO: why is this an &
) {
    if let Some(map_handle) = map_query.iter_mut().next() {
        if let Some(tiled_map) = map_assets.get(map_handle) {
            for tileset_index in 0..tiled_map.map.tilesets().len() {
                process_tileset(&mut commands, tiled_map, tileset_index);
            }
        }
    }
}

fn process_tileset(commands: &mut Commands, tiled_map: &TiledMap, tileset_index: usize) {
    let tileset = &tiled_map.map.tilesets()[tileset_index];
    let tilemap_texture = &tiled_map.tilemap_textures[&tileset_index];

    let tile_spacing = TilemapSpacing {
        x: tileset.spacing as f32,
        y: tileset.spacing as f32,
    };

    for layer in tiled_map.map.layers() {
        let layer_entity = commands.spawn_empty().id();
        if let LayerType::Tiles(tile_layer) = layer.layer_type() {
            let map_size = TilemapSize {
                x: tiled_map.map.width,
                y: tiled_map.map.height,
            };
            let grid_size = TilemapGridSize {
                x: tiled_map.map.tile_width as f32,
                y: tiled_map.map.tile_height as f32,
            };
            let tile_storage =
                process_tile_layer(commands, tile_layer, map_size, TilemapId(layer_entity));

            commands
                .entity(layer_entity)
                .insert(TilemapBundle {
                    grid_size,
                    size: map_size,
                    storage: tile_storage,
                    texture: tilemap_texture.clone(),
                    tile_size: TilemapTileSize::new(TILE_SIZE, TILE_SIZE),
                    spacing: tile_spacing,
                    map_type: TilemapType::Square,
                    ..Default::default()
                })
                .insert(Name::new("TiledMap Tiles Entity"));
        } else {
            log::info!(
                "Skipping layer {} because only tile layers are supported.",
                layer.id()
            );
        }
    }
}

fn process_tile_layer(
    commands: &mut Commands,
    tile_layer: TileLayer,
    map_size: TilemapSize,
    tilemap_id: TilemapId,
) -> TileStorage {
    let TileLayer::Finite(layer_data) = tile_layer else {
        log::info!("Skipping layer because only finite layers are supported.");
        return TileStorage::empty(map_size);
    };

    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            if let Some(layer_tile) = layer_data.get_tile(x as i32, y as i32) {
                let texture_index = layer_tile.id();
                let tile_pos = TilePos { x, y };
                let tile_entity = create_tile_entity(commands, tile_pos, tilemap_id, texture_index);
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    }

    tile_storage
}

fn create_tile_entity(
    commands: &mut Commands,
    tile_pos: TilePos,
    tilemap_id: TilemapId,
    texture_index: u32,
) -> Entity {
    let entity_builder = commands.spawn(TileBundle {
        position: tile_pos,
        tilemap_id,
        texture_index: TileTextureIndex(texture_index),
        // TODO: there may be some logic regarding flipping here that needs to be done, otherwise
        //  figure out how to get TOPLEFT coordinates to work
        ..Default::default()
    });
    entity_builder.id()
}
