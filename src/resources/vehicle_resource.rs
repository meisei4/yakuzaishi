use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    ecs::prelude::WorldExt,
    Error,
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct VehicleResource {
    pub sprite_sheet_handle: Handle<SpriteSheet>,
}

impl VehicleResource {
    pub fn load(
        world: &mut World,
        vehicle_texture_file_path: &str,
        sprite_sheet_file_path: &str,
    ) -> Result<Self, Error> {
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
            (),
            &sprite_sheet_storage,
        );

        Ok(VehicleResource {
            sprite_sheet_handle,
        })
    }
}
