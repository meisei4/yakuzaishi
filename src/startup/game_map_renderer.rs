use std::path::Path;

use bevy::prelude::{Assets, AssetServer, Commands, Handle, Rect, Res, ResMut, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout, Transform, Vec2};
use tiled::{FiniteTileLayer, Layer, LayerType, Loader, TileLayer, Tileset};

use crate::{MAP_FILE_PATH, TILESET_FILE_PATH, TILESET_TEXTURE_FILE_PATH};

pub fn render_map(
    command_buffer: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut tiled_loader = Loader::new();

    let tile_set = tiled_loader.load_tsx_tileset(Path::new(TILESET_FILE_PATH)).expect("Failed to load tileset");

    let texture_atlas_layout = create_texture_atlas_map(&tile_set);

    let tiled_map = tiled_loader.load_tmx_map(Path::new(MAP_FILE_PATH)).expect("Failed to load tilemap");

    let layers_iterator = tiled_map.layers();
    let layers_data = get_map_layers_data(layers_iterator);
    let texture_atlas_layout_handle = texture_atlas_layouts.add(texture_atlas_layout);
    for finite_layer in layers_data {
        //process_finite_layer(command_buffer, asset_server, &texture_atlas_layout_handle, finite_layer);
    }
}

fn create_texture_atlas_map(
    tile_set: &Tileset,
) -> TextureAtlasLayout {
    let columns = tile_set.columns;
    let rows = (tile_set.tilecount as f32 / columns as f32).ceil() as usize;

    let mut texture_atlas_layout = TextureAtlasLayout::new_empty(Vec2 { x: columns as f32, y: rows as f32 });
    for y in 0..rows {
        for x in 0..columns {
            let sprite_rect = Rect {
                min: Vec2::new(x as f32 * tile_set.tile_width as f32, y as f32 * tile_set.tile_height as f32),
                max: Vec2::new((x + 1) as f32 * tile_set.tile_width as f32, (y + 1) as f32 * tile_set.tile_height as f32),
            };
            texture_atlas_layout.add_texture(sprite_rect);
        }
    }
    texture_atlas_layout
}

fn get_map_layers_data<'map>(layers: impl ExactSizeIterator<Item=Layer<'map>>) -> Vec<FiniteTileLayer<'map>> {
    let mut layers_data: Vec<FiniteTileLayer> = Vec::new();

    for layer in layers {
        if let LayerType::Tiles(TileLayer::Finite(finite_layer)) = layer.layer_type() {
            layers_data.push(finite_layer);
        }
    }
    layers_data
}

fn process_finite_layer(
    mut command_buffer: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layout_handle: &Handle<TextureAtlasLayout>,
    finite_layer: FiniteTileLayer,
) {
    for y in 0..finite_layer.height() {
        for x in 0..finite_layer.width() {
            if let Some(tile) = finite_layer.get_tile(x as i32, y as i32) {
                let transform = Transform::from_xyz(x as f32, y as f32, 0.0);

                log::info!("Spawning entity for map tile at ({}, {}) with tile ID {}", x, y, tile.id());
                command_buffer.spawn(())
                    .insert(SpriteSheetBundle {
                        texture: asset_server.load(TILESET_TEXTURE_FILE_PATH),
                        atlas: TextureAtlas { layout: texture_atlas_layout_handle.clone(), index: 0 },
                        transform,
                        ..Default::default()
                    });
            }
        }
    }
}