use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    ecs::prelude::WorldExt,
    Error,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};
use amethyst::assets::ProgressCounter;
use amethyst::core::math::Vector2;
use amethyst::prelude::World;
use amethyst::renderer::Sprite;

use crate::components::hitbox::Hitbox;

pub struct VehicleResource {
    pub sprite_sheet_handle: Handle<SpriteSheet>,
    pub hitboxes: Vec<Hitbox>,
}

impl VehicleResource {
    pub fn load(
        world: &mut World,
        vehicle_texture_file_path: &str,
        sprite_sheet_file_path: &str,
        progress_counter: &mut ProgressCounter,
    ) -> Result<Handle<SpriteSheet>, Error> {
        let asset_loader = world.read_resource::<Loader>();

        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        let texture_handle = asset_loader.load(
            vehicle_texture_file_path,
            ImageFormat::default(),
            (),
            &texture_storage,
        );

        let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        let sprite_sheet_handle = asset_loader.load(
            sprite_sheet_file_path,
            SpriteSheetFormat(texture_handle),
            progress_counter,
            &sprite_sheet_storage,
        );
        Ok(sprite_sheet_handle) //partial vehicle resource
    }

    fn calculate_hitbox_for_sprite(sprite: &Sprite) -> Hitbox {
        // Calculate corners
        let corners = [
            Vector2::new(sprite.tex_coords.left, sprite.tex_coords.top), // Top left
            Vector2::new(sprite.tex_coords.right, sprite.tex_coords.top), // Top right
            Vector2::new(sprite.tex_coords.left, sprite.tex_coords.bottom), // Bottom left
            Vector2::new(sprite.tex_coords.right, sprite.tex_coords.bottom), // Bottom right
        ];

        // Calculate midpoints between corners
        let midpoints = [
            Vector2::new((corners[0].x + corners[1].x) / 2.0, corners[0].y), // Top midpoint
            Vector2::new(corners[1].x, (corners[1].y + corners[3].y) / 2.0), // Right midpoint
            Vector2::new((corners[2].x + corners[3].x) / 2.0, corners[2].y), // Bottom midpoint
            Vector2::new(corners[0].x, (corners[0].y + corners[2].y) / 2.0), // Left midpoint
        ];

        Hitbox { corners, midpoints }
    }

    pub fn generate_hitboxes(sprite_sheet: &SpriteSheet) -> Vec<Hitbox> {
        sprite_sheet.sprites.iter().map(Self::calculate_hitbox_for_sprite).collect()
    }
}

