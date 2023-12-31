use amethyst::{
    ecs::prelude::WorldExt,
    input::{InputBundle, InputHandler, StringBindings},
    prelude::*,
};

use crate::{
    resources::{
        game_map_resource::GameMapResource, key_bindings_resource::KeyBindingsResource,
        pedestrian_resource::PedestrianResource, vehicle_resource::VehicleResource,
    },
    state::{camera, game_map_renderer, vehicle_spawner},
    systems::{
        camera_tracking_system::CameraTrackingSystem,
        vehicle_controller_system::VehicleControllerSystem,
    },
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

        let key_bindings_resource = match self.entity_type {
            EntityType::Vehicle => {
                KeyBindingsResource::load(EntityType::Vehicle, VEHICLE_BINDINGS_CONFIG_FILENAME)
            }
            EntityType::Pedestrian => KeyBindingsResource::load(
                EntityType::Pedestrian,
                PEDESTRIAN_BINDINGS_CONFIG_FILENAME,
            ),
            EntityType::Menu => return, // If it's a menu, no need to load key bindings
        }
        .unwrap();

        // Insert resources specific to the entity type
        match self.entity_type {
            EntityType::Vehicle => {
                let vehicle_resource = VehicleResource::load(
                    world,
                    VEHICLE_TEXTURE_FILE_PATH,
                    VEHICLE_SPRITE_SHEET_FILE_PATH,
                );
                world.insert(vehicle_resource.unwrap());
            }
            EntityType::Pedestrian => {
                let pedestrian_resource = PedestrianResource::load(
                    world,
                    PEDESTRIAN_TEXTURE_FILE_PATH,
                    PEDESTRIAN_SPRITE_SHEET_FILE_PATH,
                );
                world.insert(pedestrian_resource.unwrap());
            }
            EntityType::Menu => { /* Menu specific resources */ }
        }

        // Insert the key bindings input bundle
        let input_bundle = key_bindings_resource
            .get_input_bundle(&self.entity_type)
            .unwrap();
        world.insert(input_bundle);
    }

    fn initialize_game_state(&mut self, world: &mut World) {
        game_map_renderer::render_map(world);
        //TODO maybe make a kind of generic spawner tht takes in arguments for the entity type
        match self.entity_type {
            EntityType::Vehicle => {
                vehicle_spawner::spawn_vehicle(world);
                //world.add_system(VehicleControllerSystem);
                //world.add_system(CameraTrackingSystem);
                //world.add_system(CollisionSystem);
            }
            EntityType::Pedestrian => {
                pedestrian_spawner::spawn_pedestrian(world);
            }
            EntityType::Menu => {
                //TODO do nothing until can like
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
