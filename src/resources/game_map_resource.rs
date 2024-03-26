use std::collections::HashMap;
use std::path::Path;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    ecs::prelude::WorldExt,
    Error,
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, Texture},
};
use tiled::{FiniteTileLayer, Loader as TiledLoader, Map, TileLayer};

use crate::components::game_map_tile_components::{GameMapTileComponents, TileType};
use crate::yakuzaishi_util;

pub struct GameMapResource {
    pub tiled_map: Map,
    pub sprite_sheet_handle: Handle<SpriteSheet>,
    pub tile_components: HashMap<(u32, u32), GameMapTileComponents>,
}

impl GameMapResource {
    pub fn load(world: &mut World, map_file_path: &str, tsx_file_path: &str, texture_file_path: &str) -> Result<Self, Error> {
        let mut tiled_loader = TiledLoader::new();

        let tileset = tiled_loader.load_tsx_tileset(Path::new(tsx_file_path)).expect("Failed to load tileset");

        let asset_loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        let texture_handle = asset_loader.load(texture_file_path, ImageFormat::default(), (), &texture_storage);

        let sprite_sheet_data = SpriteSheet {
            texture: texture_handle,
            sprites: yakuzaishi_util::create_sprites_from_tileset(&tileset),
        };
        let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        let sprite_sheet_handle = asset_loader.load_from_data(sprite_sheet_data, (), &sprite_sheet_storage);

        let tiled_map = tiled_loader.load_tmx_map(Path::new(map_file_path)).expect("Failed to load tilemap");

        let tile_components = Self::build_tile_components(&tiled_map);

        Ok(GameMapResource {
            tiled_map,
            sprite_sheet_handle,
            tile_components,
        })
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
                    components.insert((x, y), GameMapTileComponents::new(yakuzaishi_util::is_drivable_tile(tile), TileType::Normal));
                }
            }
        }
        components
    }
}
