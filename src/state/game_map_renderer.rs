use bevy::prelude::{Commands, Handle, Sprite, SpriteSheetBundle, TextureAtlas, Transform};
use tiled::{FiniteTileLayer, Layer, LayerType, TileLayer};

use crate::resources::game_map_resource::GameMapResource;

pub fn render_map(game_map: &GameMapResource, command_buffer: &mut Commands) {
    let layers_iterator = game_map.tiled_map.layers();
    let layers_data = get_map_layers_data(layers_iterator);

    for finite_layer in layers_data {
        process_finite_layer(command_buffer, finite_layer, &game_map.sprite_sheet_handle);
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
    finite_layer: FiniteTileLayer,
    sprite_sheet_handle: &Handle<TextureAtlas>,
) {
    for y in 0..finite_layer.height() {
        for x in 0..finite_layer.width() {
            if let Some(tile) = finite_layer.get_tile(x as i32, y as i32) {
                let transform = Transform::from_xyz(x as f32, y as f32, 0.0);

                let sprite = Sprite::new(tile.id());
                log::info!("Spawning entity for map tile at ({}, {}) with sprite ID {}", x, y, tile.id());
                command_buffer.spawn(())
                    .insert(SpriteSheetBundle {
                        sprite,
                        atlas: sprite_sheet_handle,
                        transform,
                        ..Default::default()
                    });
            }
        }
    }
}