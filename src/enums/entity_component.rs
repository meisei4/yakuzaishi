use amethyst::core::Transform;
use amethyst::renderer::{Camera, SpriteRender};

use crate::components::vehicle_components::VehicleComponents;

pub enum EntityComponent {
    Transform(Transform),
    SpriteRender(SpriteRender),
    Camera(Camera),
    VehicleComponent(VehicleComponents),
}