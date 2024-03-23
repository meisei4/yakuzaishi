use amethyst::core::Transform;
use amethyst::ecs::{Builder, World, WorldExt};
use amethyst::renderer::SpriteRender;

use crate::components::vehicle_components::VehicleComponents;

pub struct EntityCreationCommand {
    pub transform: Transform,
    pub sprite_render: SpriteRender,
    pub vehicle_components: Option<VehicleComponents>,
}

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
            let mut entity_builder = world.create_entity()
                .with(command.transform)
                .with(command.sprite_render);

            // TODO ugly conditional, maybe do some composition stuff
            if let Some(vehicle_components) = command.vehicle_components {
                entity_builder = entity_builder.with(vehicle_components);
            }
            entity_builder.build();
        }
    }
}
