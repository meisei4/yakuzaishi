use std::{
    env,
    fs::File,
    future::Future,
    io::{Cursor, Error, ErrorKind, Read},
    path::{Path, PathBuf},
    sync::Arc,
};

use bevy::{
    asset::{Asset, AssetLoader, AssetPath, io::Reader, LoadContext},
    prelude::{Resource, TypePath},
    utils::ConditionalSendFuture,
};
use bevy_asset::Handle;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_render::texture::Image;
// TODO: How do these next two "uses" even work?
use futures_lite::AsyncReadExt;
use thiserror::Error;
use tiled::{DefaultResourceCache, Loader, ResourceReader};

#[derive(AssetCollection, Resource)]
pub struct TiledMapAssets {
    #[asset(path = "map_data/water.tmx")]
    pub tiled_map: Handle<TiledMapSource>,
}

#[derive(TypePath, Asset)]
pub struct TiledMapSource {
    pub rs_tiled_map: tiled::Map,
    pub bevy_ecs_tilemap_textures: bevy_ecs_tilemap::map::TilemapTexture,
}

pub struct TiledLoader;

impl AssetLoader for TiledLoader {
    type Asset = TiledMapSource;
    type Settings = ();
    type Error = TiledLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> impl ConditionalSendFuture
           + Future<Output = Result<<Self as AssetLoader>::Asset, <Self as AssetLoader>::Error>>
    {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            //TODO: this is still not good
            // 1. asset path doesnt get recognized without this
            // 2. I just dont like this whole thing, need to learn rust more
            let asset_path: PathBuf = env::current_dir().unwrap().join("assets");

            let mut loader = Loader::with_cache_and_reader(
                DefaultResourceCache::new(),
                BytesResourceReader::new(&bytes, asset_path),
            );

            //TODO: No, this is wrong, broken, rs-tiled is no good
            // let mut loader = Loader::new();
            let map = loader
                .load_tmx_map(load_context.path())
                .map_err(|e| TiledLoaderError::TmxParse(e.to_string()))?;

            let tileset = map
                .tilesets()
                .first()
                .ok_or(TiledLoaderError::MissingTilesetImage)?;

            let img = tileset
                .image
                .as_ref()
                .ok_or(TiledLoaderError::MissingTilesetImage)?;

            let tmx_dir = load_context.path().parent().ok_or_else(|| {
                // TODO: why does this error have to have an extra message unlike the err_map one? can we fix that to be more clear?
                TiledLoaderError::TmxParse("TMX file has no parent directory".to_string())
            })?;

            let img_source = Path::new(&img.source);

            // TODO: this entire if statement is horrendous. it needs to be fixed.
            let img_source = if tmx_dir.ends_with("map_data") && img_source.starts_with("map_data")
            {
                img_source.strip_prefix("map_data").unwrap()
            } else {
                img_source
            };

            let tile_path = tmx_dir.join(img_source);

            let asset_path = AssetPath::from(tile_path);

            let texture: Handle<Image> = load_context.load(asset_path.clone());

            let tilemap_textures = bevy_ecs_tilemap::map::TilemapTexture::Single(texture.clone());

            let asset_map = TiledMapSource {
                rs_tiled_map: map,
                bevy_ecs_tilemap_textures: tilemap_textures,
            };

            Ok(asset_map)
        })
    }

    // TODO: what is this even for?
    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["tmx"];
        EXTENSIONS
    }
}

// TODO: rs-tiled allows too much customization, i dont like everything from here onwards (nor upwards)
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
                    File::open(full_path).map_err(|err| Error::new(ErrorKind::NotFound, err))?;
                return Ok(Box::new(file));
            }
        }
        Ok(Box::new(Cursor::new(self.bytes.clone())))
    }
}

#[derive(Error, Debug)]
pub enum TiledLoaderError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TMX Parsing Error: {0}")]
    TmxParse(String),

    #[error("Tileset image not found")]
    MissingTilesetImage,
}
