use amethyst::{core::Transform, ecs::{Join, ReadStorage, System, WriteStorage}, renderer::Camera};

use crate::components::vehicle_components::VehicleComponents;

pub struct CameraTrackingSystem;

impl<'s> System<'s> for CameraTrackingSystem {
    type SystemData = (
        ReadStorage<'s, VehicleComponents>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
    );

    fn run(&mut self, (vehicles, mut transforms, cameras): Self::SystemData) {
        let vehicle: &VehicleComponents = (&vehicles).join().next().unwrap();
        for (_, camera_transform) in (&cameras, &mut transforms).join() {
            camera_transform.set_translation_x(vehicle.base.position.x);
            camera_transform.set_translation_y(vehicle.base.position.y);
        }
    }
}