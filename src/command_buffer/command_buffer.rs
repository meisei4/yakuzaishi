use amethyst::core::Transform;
use amethyst::ecs::{Builder, World, WorldExt};
use amethyst::renderer::SpriteRender;

pub struct EntityCreationCommand {
    pub transform: Transform,
    pub sprite_render: SpriteRender,
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
            world
                .create_entity()
                .with(command.transform)
                .with(command.sprite_render)
                .build();
        }
    }
}
