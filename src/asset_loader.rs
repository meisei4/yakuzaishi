pub mod asset_loader {
    use amethyst::{
        assets::{AssetStorage, Handle, Loader},
        core::math::Vector2,
        ecs::prelude::WorldExt,
        prelude::*,
        renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    };
    use log::{error, info};

    use crate::{map::GameMap, spawner::spawner::TILE_SIZE};

    // Constants related to assets
    pub const VEHICLE_SPRITE_FILE_PATH: &str = "resources/sprites/car_sprite.png";
    pub const SPRITE_SHEET_FILE_PATH: &str = "resources/sprite_sheet.ron";
    pub const MAP_FILE_PATH: &str = "resources/maps/generated_map.png";

    pub fn load_world_map(world: &mut World) {
        let map_file_path = MAP_FILE_PATH;
        let tile_size = Vector2::new(TILE_SIZE, TILE_SIZE); // Adjust the tile size as needed

        match GameMap::load_from_file(map_file_path, tile_size) {
            Ok(world_map) => {
                world.insert(world_map); // Insert the map into the world as a resource
                info!("Loaded world map successfully");
            }
            Err(e) => {
                error!("Error loading map: {}", e);
            }
        }
    }

    pub fn load_vehicle_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
        let vehicle_sprite_file_path = VEHICLE_SPRITE_FILE_PATH;
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                vehicle_sprite_file_path, // Path to the sprite
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };

        // Create a sprite sheet with a single sprite for the vehicle
        let sprite_sheet_handle = {
            let sprite_sheet_file_path = SPRITE_SHEET_FILE_PATH;
            let loader = world.read_resource::<Loader>();
            let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                sprite_sheet_file_path,
                SpriteSheetFormat(texture_handle),
                (),
                &sprite_sheet_storage,
            )
        };

        info!("Loaded vehicle sprite sheet successfully");

        sprite_sheet_handle
    }
}
