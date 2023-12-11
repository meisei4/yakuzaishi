use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    ecs::prelude::WorldExt,
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    shred::Fetch,
};

pub struct VehicleResource {
    pub sprite_sheet_handle: Handle<SpriteSheet>,
}

impl VehicleResource {
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

        let texture_handle: Handle<Texture> = loader.load(
            vehicle_texture_file_path,
            ImageFormat::default(),
            (),
            &texture_storage,
        );

        let sprite_sheet_handle: Handle<SpriteSheet> = loader.load(
            sprite_sheet_file_path,
            SpriteSheetFormat(texture_handle),
            (),
            &sprite_sheet_storage,
        );

        VehicleResource {
            sprite_sheet_handle,
        }
    }
}
