use amethyst::core::Transform;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::renderer::sprite::SpriteSheetHandle;
use amethyst::renderer::SpriteRender;
use amethyst::shred::World;
use amethyst::{
    assets::Handle,
    renderer::{Sprite, SpriteSheet, Texture},
};

use crate::components::base_components::BaseEntityComponents;
use crate::TILE_SIZE;
use tiled::{LayerTile, Tileset};

pub fn convert_tileset_to_sprite_sheet(
    tileset: &Tileset,
    texture_handle: &Handle<Texture>,
) -> SpriteSheet {
    let mut sprites: Vec<Sprite> = Vec::new();

    // Extract tileset image details with error handling
    let image: &tiled::Image = tileset.image.as_ref().unwrap();
    let tile_width: f32 = tileset.tile_width as f32;
    let tile_height: f32 = tileset.tile_height as f32;
    let image_width: f32 = image.width as f32;
    let image_height: f32 = image.height as f32;

    // Calculate the number of columns and rows in the tileset
    let columns = (image_width / tile_width).floor() as u32;
    let rows = (image_height / tile_height).floor() as u32;

    log::info!(
        "Creating sprites for each tile in the tileset. Columns: {}, Rows: {}",
        columns,
        rows
    );

    for y in 0..rows {
        for x in 0..columns {
            let sprite: Sprite = Sprite::from_pixel_values(
                image_width as u32,
                image_height as u32,
                tile_width as u32,
                tile_height as u32,
                x * tile_width as u32,
                y * tile_height as u32,
                [0.0, 0.0], // Offsets
                false,      // Flip horizontally
                true, // TODO lol this needs to be true because i think something about tiled messes up orientation
            );
            sprites.push(sprite);
            log::debug!("Sprite created at column {}, row {}", x, y);
        }
    }

    log::info!("All sprites created successfully. Total: {}", sprites.len());

    SpriteSheet {
        texture: texture_handle.clone(),
        sprites,
    }
}

pub fn create_transform(x: f32, y: f32) -> Transform {
    let mut transform: Transform = Transform::default();
    transform.set_translation_xyz(x * TILE_SIZE, y * TILE_SIZE, 0.0);
    transform
}

pub fn create_sprite_render(
    some_id: usize,
    sprite_sheet_handle: &SpriteSheetHandle,
) -> SpriteRender {
    SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: some_id,
    }
}

pub fn update_transform(base_components: &BaseEntityComponents, transform: &mut Transform) {
    transform.set_translation_x(base_components.position.x);
    transform.set_translation_y(base_components.position.y);
}

pub fn is_drivable_tile(tile: LayerTile) -> bool {
    // Define drivable tile logic here
    tile.id() != 17 // Assuming 17 is a non-drivable tile id
}
