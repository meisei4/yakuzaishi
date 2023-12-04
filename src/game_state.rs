use amethyst::{
    assets::Handle,
    ecs::prelude::WorldExt,
    input::{InputHandler, StringBindings},
    prelude::*,
    renderer::SpriteSheet,
    shred::Fetch,
};
use log::info;

use crate::vehicle_sprite_sheet::VehicleSpriteSheet;
use crate::{camera::camera, game_map::GameMap};

pub struct Yakuzaishi {
    vehicle_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl Default for Yakuzaishi {
    fn default() -> Self {
        Yakuzaishi {
            vehicle_sprite_sheet_handle: None,
        }
    }
}

pub const VEHICLE_TEXTURE_FILE_PATH: &str = "sprites/car_sprite.png";
pub const SPRITE_SHEET_FILE_PATH: &str = "sprite_sheet.ron";
pub const MAP_FILE_PATH: &str = "resources/maps/map.tmx";

impl SimpleState for Yakuzaishi {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world: &mut World = data.world;

        let game_map: GameMap = GameMap::new(MAP_FILE_PATH);
        world.insert(game_map);
        info!("inserted game map into world successfully");

        //TODO: why is this an attribute? like what is STATE in ECS and amethyst, this is probably
        //causing the Handle<SpriteSheet> access issue
        //self.vehicle_sprite_sheet_handle = Some(resource_loader::load_vehicle_sprite_sheet(world));

        let vehicle_sprite_sheet: VehicleSpriteSheet =
            VehicleSpriteSheet::new(world, VEHICLE_TEXTURE_FILE_PATH, SPRITE_SHEET_FILE_PATH);
        world.insert(vehicle_sprite_sheet);
        info!("inserted vehicle sprite sheet successfully");
        camera::init_camera(world);
        info!("Camera initialized.");
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        // Handle quitting the game
        if let StateEvent::Input(input_event) = event {
            let input_handler: Fetch<'_, InputHandler<StringBindings>> =
                data.world.read_resource::<InputHandler<StringBindings>>();
            if input_handler.action_is_down("quit").unwrap_or(false) {
                return Trans::Quit;
            }
        }

        Trans::None
    }
}
