use amethyst::{ecs::prelude::*, renderer::sprite::SpriteSheetHandle};
use tiled::FiniteTileLayer;

use crate::resources::game_map_resource::GameMapResource;
use crate::util::{create_sprite_render, create_transform};

pub fn render_map(world: &mut World) {
    let layers_data = get_map_layers_data(world);
    for (finite_layer, sprite_sheet_handle) in layers_data {
        process_finite_layer(world, finite_layer, sprite_sheet_handle);
    }
}

fn get_map_layers_data(world: &World) -> Vec<(FiniteTileLayer, SpriteSheetHandle)> {
    let game_map = world.read_resource::<GameMapResource>();
    let sprite_sheet_handle = game_map.sprite_sheet_handle.clone();

    game_map
        .tiled_map
        .layers()
        .filter_map(|layer| {
            match layer.layer_type() {
                tiled::LayerType::Tiles(tile_layer) => match tile_layer {
                    tiled::TileLayer::Finite(finite_layer) => {
                        // Clone the data you need, avoiding returning references
                        Some((finite_layer.clone(), sprite_sheet_handle.clone()))
                    }
                    // Handle other layer types as needed
                    _ => None,
                },
                _ => None,
            }
        })
        .collect()
}

// Adjust process_finite_layer to accept the cloned data
fn process_finite_layer(
    world: &mut World,
    finite_layer: FiniteTileLayer,
    sprite_sheet_handle: SpriteSheetHandle,
) {
    for y in 0..finite_layer.height() {
        for x in 0..finite_layer.width() {
            if let Some(tile) = finite_layer.get_tile(x as i32, y as i32) {
                let transform = create_transform(x as f32, y as f32);
                let sprite_render = create_sprite_render(tile.id() as usize, &sprite_sheet_handle);

                world
                    .create_entity()
                    .with(transform)
                    .with(sprite_render)
                    .build();
            }
        }
    }
}
