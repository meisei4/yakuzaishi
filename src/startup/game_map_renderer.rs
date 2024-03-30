use bevy::prelude::{Assets, AssetServer, Commands, Handle, Image, Res, ResMut, Sprite, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout, Transform};
use tiled::{FiniteTileLayer, Layer, LayerType, Map, TileLayer};

use crate::VEHICLE_TEXTURE_FILE_PATH;

pub fn render_map(
    command_buffer: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    texture_atlas_layout: TextureAtlasLayout,
    tiled_map: Map,
) {
    let layers_iterator = tiled_map.layers();
    let layers_data = get_map_layers_data(layers_iterator);
    for finite_layer in layers_data {
        process_finite_layer(command_buffer, asset_server, &mut texture_atlas_layouts, &texture_atlas_layout, finite_layer);
    }
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
    command_buffer: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    texture_atlas_layout: &TextureAtlasLayout,
    finite_layer: FiniteTileLayer,
) {
    for y in 0..finite_layer.height() {
        for x in 0..finite_layer.width() {
            if let Some(tile) = finite_layer.get_tile(x as i32, y as i32) {
                let transform = Transform::from_xyz(x as f32, y as f32, 0.0);

                log::info!("Spawning entity for map tile at ({}, {}) with tile ID {}", x, y, tile.id());
                let layout = texture_atlas_layouts.add(texture_atlas_layout);
                command_buffer.spawn(())
                    .insert(SpriteSheetBundle {
                        texture: asset_server.load(VEHICLE_TEXTURE_FILE_PATH),
                        atlas: TextureAtlas { layout, index: 0 },
                        transform,
                        ..Default::default()
                    });
            }
        }
    }
}