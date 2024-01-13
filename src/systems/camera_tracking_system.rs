use amethyst::{
    core::Transform,
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::vehicle_components::VehicleComponents;

pub struct CameraTrackingSystem;

impl<'s> System<'s> for CameraTrackingSystem {
    type SystemData = (
        ReadStorage<'s, VehicleComponents>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (vehicle_components, mut transforms): Self::SystemData) {
        for (vehicle, transform) in (&vehicle_components, &mut transforms).join() {
            transform.set_translation_x(vehicle.base.position.x);
            transform.set_translation_y(vehicle.base.position.y);
        }
    }
}
