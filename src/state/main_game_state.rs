use amethyst::{ecs::prelude::WorldExt, GameData, input::{InputHandler, StringBindings}, SimpleState, SimpleTrans, StateData, StateEvent, Trans};
use amethyst::prelude::World;

use crate::{
    resources::{
        game_map_resource::GameMapResource,
        vehicle_resource::VehicleResource,
    },
    state::{game_map_renderer, vehicle_spawner}

    ,
};
use crate::command_buffer::command_buffer::CommandBuffer;
use crate::enums::entity_type::EntityType;
use crate::resources::system_active_flag::SystemActive;

use super::camera_initializer;

pub struct Yakuzaishi {
    pub entity_type: EntityType,
    command_buffer: CommandBuffer,
}

impl Yakuzaishi {
    pub fn new(entity_type: EntityType) -> Self {
        Self {
            entity_type,
            command_buffer: CommandBuffer::new(),
        }
    }

    fn initialize_game_state(&mut self, world: &mut World) {
        let game_map = world.read_resource::<GameMapResource>();
        game_map_renderer::render_map(&game_map, &mut self.command_buffer);
        //drop(game_map);
        // do this because you used to use world mutable borrows after this, and this line is the first time to truly understand ownership concepts
        // TODO: look to this code for understanding immutable borrow
        // TODO TODO: how the fuck did any of this work before hand?
        //  there is mutable, immutable borrows and now lifetime issues, you must truly understand ownership and borrowing rules before you continue to work on this
        match self.entity_type {
            EntityType::Vehicle => {
                // TODO this read is really sketchy since but maybe it works because the hitboxes are not yet needed in the spawn stuff
                let vehicle_resource = world.read_resource::<VehicleResource>();
                // TODO make this use command buffer etc
                vehicle_spawner::spawn_vehicle(&vehicle_resource, &game_map, &mut self.command_buffer);
            }
            _ => {}
        }
        camera_initializer::init_camera(&mut self.command_buffer);
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
        world.insert(SystemActive::new(true));

        self.initialize_game_state(world);
        self.command_buffer.execute(world);
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
