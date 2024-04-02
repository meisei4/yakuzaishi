use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Cursor, Error, ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AssetPath, BoxedFuture, Handle, LoadContext};
use bevy::prelude::Image;
use bevy_ecs_tilemap::map::TilemapTexture;
use futures_lite::AsyncReadExt;
use thiserror::Error;
use tiled::{DefaultResourceCache, Loader, ResourceReader};

use crate::helpers_hack::tiled_hack::TiledMap;

pub struct TiledLoader;

#[derive(Debug, Error)]
pub enum TiledAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load Tiled file: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for TiledLoader {
    type Asset = TiledMap;
    type Settings = ();
    type Error = TiledAssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let mut loader = Loader::with_cache_and_reader(
                DefaultResourceCache::new(),
                BytesResourceReader::new(&bytes, env::current_dir().unwrap().join("assets")),
            );
            let map = loader.load_tmx_map(load_context.path()).map_err(|e| {
                Error::new(ErrorKind::Other, format!("Could not load TMX map: {e}"))
            })?;

            let mut tilemap_textures = HashMap::default();

            for (tileset_index, tileset) in map.tilesets().iter().enumerate() {
                // Directly work with `img` assuming `tileset.image` is always `Some(img)`
                let img = tileset.image.as_ref().expect("Tileset image is required");

                let tmx_dir = load_context
                    .path()
                    .parent()
                    .expect("The asset load context was empty.");

                let img_source = Path::new(&img.source);
                let img_source =
                    if tmx_dir.ends_with("map_data") && img_source.starts_with("map_data") {
                        img_source.strip_prefix("map_data").unwrap()
                    } else {
                        img_source
                    };
                let tile_path = tmx_dir.join(img_source);
                bevy::log::info!("tile path: {}", tile_path.display());

                let asset_path = AssetPath::from(tile_path);
                bevy::log::info!("asset path: {}", asset_path.clone());

                let texture: Handle<Image> = load_context.load(asset_path.clone());

                let tilemap_texture_default = TilemapTexture::Single(texture.clone());
                
                
                tilemap_textures.insert(tileset_index, tilemap_texture_default);
            }

            let asset_map = TiledMap {
                map,
                tilemap_textures,
            };

            log::info!("Loaded map: {}", load_context.path().display());
            Ok(asset_map)
        })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["tmx"];
        EXTENSIONS
    }
}

struct BytesResourceReader {
    bytes: Arc<[u8]>,
    assets_path: PathBuf,
}

impl BytesResourceReader {
    fn new(bytes: &[u8], assets_path: PathBuf) -> Self {
        Self {
            bytes: Arc::from(bytes),
            assets_path,
        }
    }
}

impl ResourceReader for BytesResourceReader {
    type Resource = Box<dyn Read + Send + Sync>;
    type Error = Error;

    fn read_from(&mut self, path: &Path) -> Result<Self::Resource, Self::Error> {
        if let Some(extension) = path.extension() {
            if extension == "tsx" {
                let full_path = self.assets_path.join(path);
                let file =
                    File::open(&full_path).map_err(|err| Error::new(ErrorKind::NotFound, err))?;
                return Ok(Box::new(file));
            }
        }
        Ok(Box::new(Cursor::new(self.bytes.clone())))
    }
}
