use amethyst::{
    ecs::prelude::WorldExt,
    input::{InputHandler, StringBindings},
    prelude::*,
};

use crate::{
    resources::{
        game_map_resource::GameMapResource, key_bindings_resource::KeyBindingsResource,
        pedestrian_resource::PedestrianResource, vehicle_resource::VehicleResource,
    },
    state::{camera, game_map_renderer, vehicle_spawner},
    MAP_FILE_PATH, PEDESTRIAN_BINDINGS_CONFIG_FILENAME, PEDESTRIAN_SPRITE_SHEET_FILE_PATH,
    PEDESTRIAN_TEXTURE_FILE_PATH, TILESET_FILE_PATH, TILESET_TEXTURE_FILE_PATH,
    VEHICLE_BINDINGS_CONFIG_FILENAME, VEHICLE_SPRITE_SHEET_FILE_PATH, VEHICLE_TEXTURE_FILE_PATH,
};

use super::{entity_type::EntityType, pedestrian_spawner};

pub struct Yakuzaishi {
    pub entity_type: EntityType,
}

impl Yakuzaishi {
    pub fn new(entity_type: EntityType) -> Self {
        Self { entity_type }
    }

    fn load_resources(&mut self, world: &mut World) {
        let game_map_resource = GameMapResource::load(
            world,
            MAP_FILE_PATH,
            TILESET_FILE_PATH,
            TILESET_TEXTURE_FILE_PATH,
        );
        world.insert(game_map_resource.unwrap());

        //TODO: straighjt up bad code, figure out how to fix it (do the matching and the keybinding more dynamically????)
        match self.entity_type {
            EntityType::Vehicle => {
                let vehicle_resource = VehicleResource::load(
                    world,
                    VEHICLE_TEXTURE_FILE_PATH,
                    VEHICLE_SPRITE_SHEET_FILE_PATH,
                );
                let key_bindings_resource = KeyBindingsResource::load(
                    EntityType::Vehicle,
                    VEHICLE_BINDINGS_CONFIG_FILENAME,
                );
                world.insert(vehicle_resource.unwrap());
                world.insert(key_bindings_resource.unwrap());
            }
            EntityType::Pedestrian => {
                let pedestrian_resource = PedestrianResource::load(
                    world,
                    PEDESTRIAN_TEXTURE_FILE_PATH,
                    PEDESTRIAN_SPRITE_SHEET_FILE_PATH,
                );
                let key_bindings_resource = KeyBindingsResource::load(
                    EntityType::Pedestrian,
                    PEDESTRIAN_BINDINGS_CONFIG_FILENAME,
                );
                world.insert(key_bindings_resource.unwrap());
                world.insert(pedestrian_resource.unwrap());
            }
        }
    }

    fn initialize_game_state(&mut self, world: &mut World) {
        game_map_renderer::render_map(world);
        //TODO maybe make a kind of generic spawner tht takes in arguments for the entity type
        match self.entity_type {
            EntityType::Vehicle => {
                vehicle_spawner::spawn_vehicle(world);
            }
            EntityType::Pedestrian => {
                pedestrian_spawner::spawn_pedestrian(world);
            }
        }
        camera::init_camera(world);
    }
}

impl Default for Yakuzaishi {
    fn default() -> Self {
        Self::new(EntityType::Vehicle)
    }
}

impl SimpleState for Yakuzaishi {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.load_resources(world);
        self.initialize_game_state(world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Input(_) = event {
            let input_handler = data.world.read_resource::<InputHandler<StringBindings>>();
            if input_handler.action_is_down("quit").unwrap_or(false) {
                return Trans::Quit;
            }
        }

        Trans::None
    }
}
