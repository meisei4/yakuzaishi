use amethyst::core::Transform;
use amethyst::ecs::{Builder, World, WorldExt};
use amethyst::renderer::{Camera, SpriteRender};
use crate::components::vehicle_components::VehicleComponents;

enum EntityComponent {
    Transform(Transform),
    SpriteRender(SpriteRender),
    Camera(Camera),
    VehicleComponent(VehicleComponents),
}

pub struct EntityCreationCommand {
    components: Vec<EntityComponent>,
}

impl EntityCreationCommand {
    pub fn new() -> Self {
        Self { components: Vec::new() }
    }

    // Methods to add each type of component
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.components.push(EntityComponent::Transform(transform));
        self
    }

    pub fn with_sprite_render(mut self, sprite_render: SpriteRender) -> Self {
        self.components.push(EntityComponent::SpriteRender(sprite_render));
        self
    }

    pub fn with_camera(mut self, camera: Camera) -> Self {
        self.components.push(EntityComponent::Camera(camera));
        self
    }

    pub fn with_vehicle_component(mut self, vehicle_component: VehicleComponents) -> Self {
        self.components.push(EntityComponent::VehicleComponent(vehicle_component));
        self
    }
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
            let mut entity_builder = world.create_entity();
            for component in command.components {
                match component {
                    EntityComponent::Transform(transform) => {
                        entity_builder = entity_builder.with(transform);
                    },
                    EntityComponent::SpriteRender(sprite_render) => {
                        entity_builder = entity_builder.with(sprite_render);
                    },
                    EntityComponent::Camera(camera) => {
                        entity_builder = entity_builder.with(camera);
                    },
                    EntityComponent::VehicleComponent(vehicle_component) => {
                        entity_builder = entity_builder.with(vehicle_component);
                    },
                }
            }
            entity_builder.build();
        }
    }
}
