use amethyst::core::Transform;
use amethyst::renderer::Sprite;
use amethyst::renderer::sprite::SpriteSheetHandle;
use amethyst::renderer::SpriteRender;
use tiled::{LayerTile, Tileset};

use crate::components::base_components::BaseEntityComponents;
use crate::TILE_SIZE;

pub fn create_sprites_from_tileset(tileset: &Tileset) -> Vec<Sprite> {
    let image = match tileset.image.as_ref() {
        Some(image) => image,
        None => panic!("Tileset image is missing!"), // Consider a more graceful error handling
    };

    let columns = image.width as u32 / tileset.tile_width;
    let rows = image.height as u32 / tileset.tile_height;

    let mut sprites = Vec::new();
    for y in 0..rows {
        for x in 0..columns {
            let sprite = Sprite::from_pixel_values(
                image.width as u32,
                image.height as u32,
                tileset.tile_width,
                tileset.tile_height,
                (x * tileset.tile_width) as u32,
                (y * tileset.tile_height) as u32,
                [0.0, 0.0], // Offsets
                false,      // Flip horizontally
                true,       // Orientation issue
            );
            sprites.push(sprite);
        }
    }
    sprites
}

pub fn create_transform(x: f32, y: f32) -> Transform {
    let mut transform: Transform = Transform::default();
    transform.set_translation_xyz(x * TILE_SIZE, y * TILE_SIZE, 0.0);
    transform
}

pub fn create_sprite_render(some_id: usize, sprite_sheet_handle: &SpriteSheetHandle) -> SpriteRender {
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
