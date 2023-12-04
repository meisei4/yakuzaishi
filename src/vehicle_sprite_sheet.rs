use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    ecs::prelude::WorldExt,
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    shred::Fetch,
};
use log::info;

pub struct VehicleSpriteSheet(pub Option<Handle<SpriteSheet>>);

impl VehicleSpriteSheet {
    pub fn new(
        world: &mut World,
        vehicle_texture_file_path: &str,
        sprite_sheet_file_path: &str,
    ) -> Self {
        let loader: Fetch<'_, Loader> = world.read_resource::<Loader>();
        let texture_storage: Fetch<'_, AssetStorage<Texture>> =
            world.read_resource::<AssetStorage<Texture>>();
        let sprite_sheet_storage: Fetch<'_, AssetStorage<SpriteSheet>> =
            world.read_resource::<AssetStorage<SpriteSheet>>();

        // Load the texture and sprite sheet as you did before
        let texture_handle = loader.load(
            vehicle_texture_file_path,
            ImageFormat::default(),
            (),
            &texture_storage,
        );

        let sprite_sheet_handle = loader.load(
            sprite_sheet_file_path,
            SpriteSheetFormat(texture_handle),
            (),
            &sprite_sheet_storage,
        );

        // Store the handle in the new resource
        VehicleSpriteSheet(Some(sprite_sheet_handle))
    }
}
