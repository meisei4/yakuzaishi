use amethyst::{
    assets::Handle,
    renderer::{Sprite, SpriteSheet, Texture},
};
use log::info;
use tiled::Tileset;

pub fn convert_tileset_to_sprite_sheet(tileset: &Tileset, texture_handle: &Handle<Texture>) -> SpriteSheet {
    let mut sprites: Vec<Sprite> = Vec::new();

    // Extract tileset image details with error handling
    let image = tileset.image.as_ref().unwrap();
    let tile_width = tileset.tile_width as f32;
    let tile_height = tileset.tile_height as f32;
    let image_width = image.width as f32;
    let image_height = image.height as f32;

    // Calculate the number of columns and rows in the tileset
    let columns = (image_width / tile_width).floor() as u32;
    let rows = (image_height / tile_height).floor() as u32;

    info!("Creating sprites for each tile in the tileset. Columns: {}, Rows: {}", columns, rows);

    for y in 0..rows {
        for x in 0..columns {
            let sprite = Sprite::from_pixel_values(
                image_width as u32,
                image_height as u32,
                tile_width as u32,
                tile_height as u32,
                (x * tile_width as u32) as u32,
                (y * tile_height as u32) as u32,
                [0.0, 0.0], // Offsets
                false,      // Flip horizontally
                true,      // TODO lol this needs to be true because i think something about tiled messes up orientation
            );
            sprites.push(sprite);
            info!("Sprite created at column {}, row {}", x, y);
        }
    }

    info!("All sprites created successfully. Total: {}", sprites.len());

    SpriteSheet {
        texture: texture_handle.clone(),
        sprites,
    }
}
