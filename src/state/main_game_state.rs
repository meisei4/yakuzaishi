use amethyst::{
    ecs::prelude::WorldExt,
    input::{InputHandler, StringBindings},
    prelude::*,
    shred::Fetch,
};

use crate::{
    camera::camera,
    resources::{
        game_map_resource::GameMapResource, pedestrian_resource::PedestrianResource,
        vehicle_resource::VehicleResource,
    },
    MAP_FILE_PATH, PEDESTRIAN_SPRITE_SHEET_FILE_PATH, PEDESTRIAN_TEXTURE_FILE_PATH,
    TILESET_FILE_PATH, TILESET_TEXTURE_FILE_PATH, VEHICLE_SPRITE_SHEET_FILE_PATH,
    VEHICLE_TEXTURE_FILE_PATH,
};

pub struct Yakuzaishi {}

impl Default for Yakuzaishi {
    fn default() -> Self {
        Self {}
    }
}

impl SimpleState for Yakuzaishi {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world: &mut World = data.world;

        let game_map: GameMapResource = GameMapResource::new(
            world,
            MAP_FILE_PATH,
            TILESET_FILE_PATH,
            TILESET_TEXTURE_FILE_PATH,
        );
        world.insert(game_map);

        log::info!("inserted game map into world successfully");

        let vehicle_sprite_sheet: VehicleResource = VehicleResource::new(
            world,
            VEHICLE_TEXTURE_FILE_PATH,
            VEHICLE_SPRITE_SHEET_FILE_PATH,
        );
        world.insert(vehicle_sprite_sheet);

        let pedestrian_sprite_sheet: PedestrianResource = PedestrianResource::new(
            world,
            PEDESTRIAN_TEXTURE_FILE_PATH,
            PEDESTRIAN_SPRITE_SHEET_FILE_PATH,
        );
        world.insert(pedestrian_sprite_sheet);

        log::info!("inserted vehicle sprite sheet successfully");

        camera::init_camera(world);

        log::info!("Camera initialized.");
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        // Handle quitting the game
        if let StateEvent::Input(_input_event) = event {
            let input_handler: Fetch<'_, InputHandler<StringBindings>> =
                data.world.read_resource::<InputHandler<StringBindings>>();
            if input_handler.action_is_down("quit").unwrap_or(false) {
                return Trans::Quit;
            }
        }

        Trans::None
    }
}
