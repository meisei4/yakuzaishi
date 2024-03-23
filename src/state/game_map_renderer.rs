use amethyst::renderer::sprite::SpriteSheetHandle;
use tiled::{FiniteTileLayer, Layer, LayerType, TileLayer};

use crate::command_buffer::command_buffer::{CommandBuffer, EntityCreationCommand};
use crate::resources::game_map_resource::GameMapResource;
use crate::util::{create_sprite_render, create_transform};

pub fn render_map(game_map: &GameMapResource, command_buffer: &mut CommandBuffer) {
    let layers_iterator = game_map.tiled_map.layers();
    let layers_data = get_map_layers_data(layers_iterator);

    for finite_layer in layers_data {
        process_finite_layer(finite_layer, game_map.sprite_sheet_handle.clone(), command_buffer);
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

fn process_finite_layer(finite_layer: FiniteTileLayer, sprite_sheet_handle: SpriteSheetHandle, command_buffer: &mut CommandBuffer) {
    for y in 0..finite_layer.height() {
        for x in 0..finite_layer.width() {
            if let Some(tile) = finite_layer.get_tile(x as i32, y as i32) {
                let transform = create_transform(x as f32, y as f32);
                let sprite_render = create_sprite_render(tile.id() as usize, &sprite_sheet_handle);
                command_buffer.add_command(EntityCreationCommand { transform, sprite_render, vehicle_components: None });
            }
        }
    }
}