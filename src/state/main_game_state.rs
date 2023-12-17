use amethyst::{
    assets::Processor,
    ecs::{prelude::WorldExt, DispatcherBuilder},
    input::{InputBundle, InputHandler, StringBindings},
    prelude::*,
    shred::Fetch,
};

use crate::{
    resources::{
        game_map_resource::GameMapResource, pedestrian_resource::PedestrianResource,
        vehicle_resource::VehicleResource,
    },
    state::{camera, game_map_renderer, vehicle_spawner},
    MAP_FILE_PATH, PEDESTRIAN_BINDINGS_CONFIG_FILENAME, PEDESTRIAN_SPRITE_SHEET_FILE_PATH,
    PEDESTRIAN_TEXTURE_FILE_PATH, TILESET_FILE_PATH, TILESET_TEXTURE_FILE_PATH,
    VEHICLE_BINDINGS_CONFIG_FILENAME, VEHICLE_SPRITE_SHEET_FILE_PATH, VEHICLE_TEXTURE_FILE_PATH,
};

pub struct Yakuzaishi {
    pub entity_type: String,
    bindings_config_path: String,
}

impl Yakuzaishi {
    pub fn new(entity_type: &str, bindings_config_path: &str) -> Self {
        Self {
            entity_type: entity_type.to_string(),
            bindings_config_path: bindings_config_path.to_string(),
        }
    }
}

impl Default for Yakuzaishi {
    fn default() -> Self {
        Self {
            entity_type: "".to_string(),
            bindings_config_path: VEHICLE_BINDINGS_CONFIG_FILENAME.to_string(),
        }
    }
}

impl SimpleState for Yakuzaishi {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world: &mut World = data.world;

        // Load the input bindings based on the entity type
        let input_bundle = InputBundle::<StringBindings>::new()
            .with_bindings_from_file(&self.bindings_config_path)
            .expect("Failed to load input bindings");

        // Initialize the dispatcher to include the input processor
        let mut dispatcher = DispatcherBuilder::new()
            bundle(input_bundle)
            .expect("Failed to create dispatcher")
            .build();
        dispatcher.setup(world);

        // Load game resources
        self.load_resources(world);

        // Initialize game state based on entity type
        self.initialize_game_state(world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
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

impl Yakuzaishi {
    fn load_resources(&mut self, world: &mut World) {
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
    }

    fn initialize_game_state(&mut self, world: &mut World) {
        game_map_renderer::render_map(world);
        vehicle_spawner::spawn_vehicle(world);
        camera::init_camera(world);
        log::info!("Game state initialized for {}", self.entity_type);
    }
}
