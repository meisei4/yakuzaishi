use std::path::Path;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    ecs::prelude::WorldExt,
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, Texture},
    shred::Fetch,
};
use tiled::Loader as TiledLoader;

use crate::util;

pub struct GameMapResource {
    pub tiled_map: tiled::Map,
    pub sprite_sheet_handle: Handle<SpriteSheet>,
}

impl GameMapResource {
    pub fn new(
        world: &mut World,
        map_file_path: &str,
        tsx_file_path: &str,
        texture_file_path: &str,
    ) -> Self {
        let mut tiled_loader: TiledLoader = TiledLoader::new();
        let asset_loader: Fetch<'_, Loader> = world.read_resource::<Loader>();
        let texture_storage: Fetch<'_, AssetStorage<Texture>> =
            world.read_resource::<AssetStorage<Texture>>();
        let sprite_sheet_storage: Fetch<'_, AssetStorage<SpriteSheet>> =
            world.read_resource::<AssetStorage<SpriteSheet>>();

        log::info!("Loading TMX map file: {}", map_file_path);
        let tiled_map: tiled::Map = tiled_loader
            .load_tmx_map(Path::new(map_file_path))
            .expect("Failed to load tilemap");

        log::info!("TMX map file loaded successfully.");
        log::info!("Loading texture for sprite sheet: {}", texture_file_path);
        let texture_handle: Handle<Texture> = asset_loader.load(
            texture_file_path,
            ImageFormat::default(),
            (),
            &texture_storage,
        );
        log::info!("Texture loaded successfully.");

        log::info!("Loading TSX tileset file: {}", tsx_file_path);
        let tileset: tiled::Tileset = tiled_loader
            .load_tsx_tileset(Path::new(tsx_file_path))
            .expect("Failed to load tileset");
        log::info!("TSX tileset file loaded successfully.");

        log::info!("Converting tileset to sprite sheet.");
        let sprite_sheet_data: SpriteSheet =
            util::convert_tileset_to_sprite_sheet(&tileset, &texture_handle);
        log::info!("Tileset converted to sprite sheet successfully.");

        log::info!("Creating sprite sheet handle.");
        let sprite_sheet_handle: Handle<SpriteSheet> =
            asset_loader.load_from_data(sprite_sheet_data, (), &sprite_sheet_storage);
        log::info!("Sprite sheet handle created successfully.");

        GameMapResource {
            tiled_map,
            sprite_sheet_handle,
        }
    }
}
