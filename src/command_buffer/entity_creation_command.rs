use amethyst::core::Transform;
use amethyst::renderer::{Camera, SpriteRender};

use crate::components::vehicle_components::VehicleComponents;
use crate::enums::entity_component::EntityComponent;

pub struct EntityCreationCommand {
    pub components: Vec<EntityComponent>,
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
