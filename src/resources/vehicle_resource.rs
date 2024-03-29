use std::fs;

use bevy::{asset::ron,
           prelude::{AssetServer, Handle, Image, Rect, Res, Resource, TextureAtlas, Vec2},
};
use serde::Deserialize;

#[derive(Resource, Deserialize)]
pub struct VehicleResource {
    pub sprite_sheet_handle: Handle<TextureAtlas>,
}

#[derive(Deserialize)]
struct SpriteSheetSpec {
    texture_width: u32,
    texture_height: u32,
    sprites: Vec<SpriteSpec>,
}

#[derive(Deserialize)]
struct SpriteSpec {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    offsets: (f32, f32),
}

impl VehicleResource {
    pub fn load(
        asset_server: &Res<AssetServer>,
        vehicle_texture_file_path: &str,
        sprite_sheet_file_path: &str,
    ) -> VehicleResource {
        let texture_handle: Handle<Image> = asset_server.load(vehicle_texture_file_path);

        let sprite_sheet_spec = Self::load_sprite_sheet_spec_from_file(sprite_sheet_file_path);
        let sprite_sheet_handle = Self::create_texture_atlas(texture_handle, &sprite_sheet_spec);
        VehicleResource { sprite_sheet_handle }
    }

    fn load_sprite_sheet_spec_from_file(file_path: &str) -> SpriteSheetSpec {
        let ron_data = fs::read_to_string(file_path).expect("Failed to read RON file");
        ron::from_str(&ron_data).expect("Failed to deserialize RON data")
    }

    fn create_texture_atlas(
        texture_handle: Handle<Image>,
        sprite_sheet_spec: &SpriteSheetSpec,
    ) -> Handle<TextureAtlas> {
        let mut atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(sprite_sheet_spec.texture_width as f32, sprite_sheet_spec.texture_height as f32));

        for sprite in &sprite_sheet_spec.sprites {
            atlas.add_texture(Rect {
                min: Vec2::new(sprite.x as f32, sprite.y as f32),
                max: Vec2::new((sprite.x + sprite.width) as f32, (sprite.y + sprite.height) as f32),
            });
        }
        atlas
    }
}