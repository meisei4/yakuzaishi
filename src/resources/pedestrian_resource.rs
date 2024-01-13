use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    ecs::prelude::WorldExt,
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    Error,
};

pub struct PedestrianResource {
    pub sprite_sheet_handle: Handle<SpriteSheet>,
}

impl PedestrianResource {
    pub fn load(
        world: &mut World,
        pedestrian_texture_file_path: &str,
        sprite_sheet_file_path: &str,
    ) -> Result<Self, Error> {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

        let texture_handle = loader.load(
            pedestrian_texture_file_path,
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

        Ok(PedestrianResource {
            sprite_sheet_handle,
        })
    }
}
