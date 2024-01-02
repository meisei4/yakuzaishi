use amethyst::{
    core::transform::Transform,
    ecs::{prelude::*, storage::MaskedStorage, world::EntitiesRes, Entities, Read},
    renderer::{sprite::SpriteSheetHandle, SpriteRender},
    shred::{Fetch, FetchMut},
};

use tiled::FiniteTileLayer;

use crate::resources::game_map_resource::GameMapResource;
use crate::util::{create_sprite_render, create_transform};

pub fn render_map(world: &mut World) {
    log::info!("try to load");
    let game_map: Fetch<'_, GameMapResource> = world.read_resource::<GameMapResource>();
    log::info!("loaded");
    let entities: Read<'_, EntitiesRes> = world.entities();
    let transforms: &mut Storage<'_, Transform, FetchMut<'_, MaskedStorage<Transform>>> =
        &mut world.write_storage::<Transform>();
    let sprite_renders: &mut Storage<'_, SpriteRender, FetchMut<'_, MaskedStorage<SpriteRender>>> =
        &mut world.write_storage::<SpriteRender>();
    for layer in game_map.tiled_map.layers() {
        match layer.layer_type() {
            tiled::LayerType::Tiles(tile_layer) => match tile_layer {
                tiled::TileLayer::Finite(finite_layer) => {
                    process_finite_layer(
                        &finite_layer,
                        &entities,
                        &game_map.sprite_sheet_handle,
                        transforms,
                        sprite_renders,
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

fn process_finite_layer<'a>(
    finite_layer: &FiniteTileLayer,
    entities: &Entities<'a>,
    sprite_sheet_handle: &'a SpriteSheetHandle,
    transforms: &mut WriteStorage<'a, Transform>,
    sprite_renders: &mut WriteStorage<'a, SpriteRender>,
) {
    for y in 0..finite_layer.height() {
        for x in 0..finite_layer.width() {
            if let Some(tile) = finite_layer.get_tile(x as i32, y as i32) {
                let transform: Transform = create_transform(x as f32, y as f32);
                let sprite_render: SpriteRender =
                    create_sprite_render(tile.id() as usize, sprite_sheet_handle);

                entities
                    .build_entity()
                    .with(transform, transforms)
                    .with(sprite_render, sprite_renders)
                    .build();
            }
        }
    }
}
