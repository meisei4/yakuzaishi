use std::{
    collections::HashMap,
    path::Path,
};

use bevy::prelude::{AssetServer, Handle, Image, Rect, Res, Resource, TextureAtlas, TextureAtlasBuilder, Vec2};
use serde::Deserialize;
use tiled::{FiniteTileLayer, LayerTile, Loader as TiledLoader, Map, TileLayer, Tileset};

use crate::components::game_map_tile_components::{GameMapTileComponents, TileType};

#[derive(Resource, Deserialize)]
pub struct GameMapResource {
    pub tiled_map: Map,
    pub sprite_sheet_handle: Handle<TextureAtlas>,
    pub tile_components: HashMap<(u32, u32), GameMapTileComponents>, // TODO: this might not be following ECS rules GameMap might need components itself?
}

impl GameMapResource {
    pub fn load(asset_server: &Res<AssetServer>,
                map_file_path: &str,
                tsx_file_path: &str,
                texture_file_path: &str,
    ) -> Self {
        let mut tiled_loader = TiledLoader::new();

        let tileset = tiled_loader.load_tsx_tileset(Path::new(tsx_file_path)).expect("Failed to load tileset");

        let sprite_sheet_handle = Self::create_texture_atlas(&asset_server,
                                                             &texture_file_path,
                                                             &tileset);

        let tiled_map = tiled_loader.load_tmx_map(Path::new(map_file_path)).expect("Failed to load tilemap");

        let tile_components = Self::build_tile_components(&tiled_map);

        GameMapResource {
            tiled_map,
            sprite_sheet_handle,
            tile_components,
        }
    }

    fn build_tile_components(tiled_map: &Map) -> HashMap<(u32, u32), GameMapTileComponents> {
        let mut tile_components = HashMap::new();

        for layer in tiled_map.layers() {
            if let Some(TileLayer::Finite(finite_layer)) = layer.as_tile_layer() {
                let layer_components = Self::process_layer(finite_layer);
                tile_components.extend(layer_components);
            }
        }
        tile_components
    }

    fn process_layer(finite_layer: FiniteTileLayer) -> HashMap<(u32, u32), GameMapTileComponents> {
        let mut components = HashMap::new();
        for y in 0..finite_layer.height() {
            for x in 0..finite_layer.width() {
                if let Some(tile) = finite_layer.get_tile(x as i32, y as i32) {
                    components.insert((x, y), GameMapTileComponents::new(is_drivable_tile(tile), TileType::Normal));
                }
            }
        }
        components
    }


    fn create_texture_atlas(
        asset_server: &Res<AssetServer>,
        texture_file_path: &str,
        tileset: &Tileset,
    ) -> Handle<TextureAtlas> {
        let texture_handle: Handle<Image> = asset_server.load(texture_file_path);

        let mut atlas_builder = TextureAtlasBuilder::new();

        let columns = tileset.columns;
        let rows = (tileset.tilecount as f32 / columns as f32).ceil() as usize;

        for y in 0..rows {
            for x in 0..columns {
                let sprite_rect = Rect {
                    min: Vec2::new(x as f32 * tileset.tile_width as f32, y as f32 * tileset.tile_height as f32),
                    max: Vec2::new((x + 1) as f32 * tileset.tile_width as f32, (y + 1) as f32 * tileset.tile_height as f32),
                };
                atlas_builder.add_rect(sprite_rect);
            }
        }

        let texture_atlas = atlas_builder.finish(&texture_handle).unwrap();
        texture_atlas
    }
}

fn is_drivable_tile(tile: LayerTile) -> bool {
    // Define drivable tile logic here
    tile.id() != 17 // Assuming 17 is a non-drivable tile id
}
