use amethyst::ecs::{Builder, World, WorldExt};

use crate::command_buffer::command_log::CommandLog;
use crate::command_buffer::entity_creation_command::EntityCreationCommand;
use crate::enums::entity_component::EntityComponent;

pub struct CommandBuffer {
    commands: Vec<EntityCreationCommand>,
}

impl CommandBuffer {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn add_command(&mut self, command: EntityCreationCommand) {
        self.commands.push(command);
    }

    pub fn execute(&mut self, world: &mut World) {
        for command in self.commands.drain(..) {
            command.log_before();

            let mut entity_builder = world.create_entity();
            for component in &command.components {
                match component {
                    EntityComponent::Transform(transform) => {
                        entity_builder = entity_builder.with(transform.clone());
                    },
                    EntityComponent::SpriteRender(sprite_render) => {
                        entity_builder = entity_builder.with(sprite_render.clone());
                    },
                    EntityComponent::Camera(camera) => {
                        entity_builder = entity_builder.with(camera.clone());
                    },
                    EntityComponent::VehicleComponent(vehicle_component) => {
                        entity_builder = entity_builder.with(vehicle_component.clone());
                    },
                }
            }
            entity_builder.build();

            command.log_after();
        }
    }
}
