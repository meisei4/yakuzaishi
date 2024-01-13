use amethyst::ecs::{ReadExpect, System, WriteExpect};

use crate::components::camera_components::CameraComponents;
use crate::components::vehicle_components::VehicleComponents;

pub struct CameraTrackingSystem;

impl<'s> System<'s> for CameraTrackingSystem {
    type SystemData = (
        ReadExpect<'s, VehicleComponents>, // Assuming a single instance in the world
        WriteExpect<'s, CameraComponents>, // Assuming a single instance in the world
    );

    fn run(&mut self, (vehicle_components, mut camera_components): Self::SystemData) {
        // Directly access the components without iteration
        camera_components
            .transform
            .set_translation_x(vehicle_components.base.position.x);
        camera_components
            .transform
            .set_translation_y(vehicle_components.base.position.y);
    }
}
