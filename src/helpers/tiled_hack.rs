use std::env;
use std::fs::File;
use std::io::{Cursor, Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use bevy::{
    asset::{AssetLoader, AssetPath, AsyncReadExt, io::Reader},
    log,
    prelude::{
        Added, Asset, AssetApp, AssetEvent, AssetId, Assets, Bundle, Commands, Component,
        DespawnRecursiveExt, Entity, EventReader, GlobalTransform, Handle, Image, Plugin, Query,
        Res, Transform, Update,
    },
    reflect::TypePath,
    utils::{BoxedFuture, HashMap},
};
use bevy_ecs_tilemap::prelude::*;
use thiserror::Error;

use crate::TILE_SIZE;

#[derive(Default)]
pub struct TiledMapPlugin;

impl Plugin for TiledMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset::<TiledMap>()
            .register_asset_loader(TiledLoader)
            .add_systems(Update, process_loaded_maps);
    }
}

#[derive(TypePath, Asset)]
pub struct TiledMap {
    pub map: tiled::Map,
    pub tilemap_textures: HashMap<usize, TilemapTexture>,
}

// Stores a list of tiled layers.
#[derive(Component, Default)]
pub struct TiledLayersStorage {
    pub storage: HashMap<u32, Entity>,
}

#[derive(Default, Bundle)]
pub struct TiledMapBundle {
    pub tiled_map: Handle<TiledMap>,
    pub storage: TiledLayersStorage,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub render_settings: TilemapRenderSettings,
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

impl tiled::ResourceReader for BytesResourceReader {
    type Resource = Box<dyn std::io::Read + Send + Sync>;
    type Error = std::io::Error;

    fn read_from(&mut self, path: &Path) -> std::result::Result<Self::Resource, Self::Error> {
        // Check if the path has a .tsx extension
        if let Some(extension) = path.extension() {
            if extension == "tsx" {
                // If the file is a .tsx file, attempt to load it from the filesystem
                let full_path = self.assets_path.join(path);
                let file =
                    File::open(&full_path).map_err(|err| Error::new(ErrorKind::NotFound, err))?;
                return Ok(Box::new(file));
            }
        }
        // If the path is not a .tsx file, load the byte data
        Ok(Box::new(Cursor::new(self.bytes.clone())))
    }
}

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
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let mut loader = tiled::Loader::with_cache_and_reader(
                tiled::DefaultResourceCache::new(),
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
                let img_source = if tmx_dir.ends_with("map_data") && img_source.starts_with("map_data") {
                    img_source.strip_prefix("map_data").unwrap()
                } else {
                    img_source
                };
                let tile_path = tmx_dir.join(img_source);
                log::info!("tile path: {}", tile_path.display());

                let asset_path = AssetPath::from(tile_path);
                log::info!("asset path: {}", asset_path.clone());

                let texture: Handle<Image> = load_context.load(asset_path.clone());

                let tilemap_texture = TilemapTexture::Single(texture.clone());

                tilemap_textures.insert(tileset_index, tilemap_texture);
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

pub fn process_loaded_maps(
    mut commands: Commands,
    mut map_events: EventReader<AssetEvent<TiledMap>>,
    maps: Res<Assets<TiledMap>>,
    tile_storage_query: Query<(Entity, &TileStorage)>,
    mut map_query: Query<(
        &Handle<TiledMap>,
        &mut TiledLayersStorage,
        &TilemapRenderSettings,
    )>,
    new_maps: Query<&Handle<TiledMap>, Added<Handle<TiledMap>>>,
) {
    let mut changed_maps = Vec::<AssetId<TiledMap>>::default();
    for event in map_events.read() {
        match event {
            AssetEvent::Added { id } => {
                log::info!("Map added!");
                changed_maps.push(*id);
            }
            AssetEvent::Modified { id } => {
                log::info!("Map changed!");
                changed_maps.push(*id);
            }
            AssetEvent::Removed { id } => {
                log::info!("Map removed!");
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_maps.retain(|changed_handle| changed_handle == id);
            }
            _ => continue,
        }
    }

    // If we have new map entities add them to the changed_maps list.
    for new_map_handle in new_maps.iter() {
        changed_maps.push(new_map_handle.id());
    }

    for changed_map in changed_maps.iter() {
        for (map_handle, mut layer_storage, render_settings) in map_query.iter_mut() {
            // only deal with currently changed map
            if map_handle.id() != *changed_map {
                continue;
            }
            if let Some(tiled_map) = maps.get(map_handle) {
                // TODO: Create a RemoveMap component..
                for layer_entity in layer_storage.storage.values() {
                    if let Ok((_, layer_tile_storage)) = tile_storage_query.get(*layer_entity) {
                        for tile in layer_tile_storage.iter().flatten() {
                            commands.entity(*tile).despawn_recursive()
                        }
                    }
                    // commands.entity(*layer_entity).despawn_recursive();
                }

                // The TilemapBundle requires that all tile images come exclusively from a single
                // tiled texture or from a Vec of independent per-tile images. Furthermore, all of
                // the per-tile images must be the same size. Since Tiled allows tiles of mixed
                // tilesets on each layer and allows differently-sized tile images in each tileset,
                // this means we need to load each combination of tileset and layer separately.
                for (tileset_index, tileset) in tiled_map.map.tilesets().iter().enumerate() {
                    let Some(tilemap_texture) = tiled_map.tilemap_textures.get(&tileset_index)
                        else {
                            log::warn!("Skipped creating layer with missing tilemap textures.");
                            continue;
                        };

                    let tile_spacing = TilemapSpacing {
                        x: tileset.spacing as f32,
                        y: tileset.spacing as f32,
                    };

                    for (layer_index, layer) in tiled_map.map.layers().enumerate() {
                        let tiled::LayerType::Tiles(tile_layer) = layer.layer_type() else {
                            log::info!(
                                "Skipping layer {} because only tile layers are supported.",
                                layer.id()
                            );
                            continue;
                        };

                        let tiled::TileLayer::Finite(layer_data) = tile_layer else {
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
                        // TODO ^ this i can add components to?

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
                                let tile_entity = commands
                                    .spawn(TileBundle {
                                        position: tile_pos,
                                        tilemap_id: TilemapId(layer_entity),
                                        texture_index: TileTextureIndex(texture_index),
                                        flip: TileFlip {
                                            x: layer_tile_data.unwrap().flip_h,
                                            y: !layer_tile_data.unwrap().flip_v,
                                            d: layer_tile_data.unwrap().flip_d,
                                        },
                                        ..Default::default()
                                    })
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
                            render_settings: *render_settings,
                            ..Default::default()
                        });

                        layer_storage
                            .storage
                            .insert(layer_index as u32, layer_entity);
                    }
                }
            }
        }
    }
}