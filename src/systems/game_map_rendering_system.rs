use amethyst::{
    core::transform::Transform,
    ecs::{prelude::*, Entities, LazyUpdate, Read, ReadExpect},
    renderer::{sprite::SpriteSheetHandle, SpriteRender, Transparent},
};

use tiled::{FiniteTileLayer, LayerTile};

use crate::{resources::game_map_resource::GameMapResource, TILE_SIZE};

pub struct MapRenderingSystem;

impl<'s> System<'s> for MapRenderingSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, GameMapResource>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, game_map, lazy_update): Self::SystemData) {
        for layer in game_map.tiled_map.layers() {
            match layer.layer_type() {
                tiled::LayerType::Tiles(tile_layer) => match tile_layer {
                    tiled::TileLayer::Finite(finite_layer) => {
                        process_finite_layer(
                            &finite_layer,
                            &entities,
                            &game_map.sprite_sheet_handle,
                            &lazy_update,
                        );
                    }
                    tiled::TileLayer::Infinite(_) => todo!(),
                },
                tiled::LayerType::Objects(_) => todo!(),
                tiled::LayerType::Image(_) => todo!(),
                tiled::LayerType::Group(_) => todo!(),
            }
        }
    }
}

fn process_finite_layer(
    finite_layer: &FiniteTileLayer,
    entities: &Entities,
    sprite_sheet_handle: &SpriteSheetHandle,
    lazy_update: &Read<LazyUpdate>,
) {
    for y in 0..finite_layer.height() {
        for x in 0..finite_layer.width() {
            if let Some(tile) = finite_layer.get_tile(x as i32, y as i32) {
                let transform: Transform = create_transform_for_tile(x, y);
                let sprite_render: SpriteRender =
                    create_sprite_render_for_tile(tile, sprite_sheet_handle);

                lazy_update
                    .create_entity(entities)
                    .with(transform)
                    .with(sprite_render)
                    .with(Transparent)
                    .build();
            }
        }
    }
}

fn create_transform_for_tile(x: u32, y: u32) -> Transform {
    let mut transform: Transform = Transform::default();
    transform.set_translation_xyz(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0);
    transform
}

fn create_sprite_render_for_tile(
    tile: LayerTile,
    sprite_sheet_handle: &SpriteSheetHandle,
) -> SpriteRender {
    SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: tile.id() as usize,
    }
}
