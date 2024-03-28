use amethyst::{core::Transform, ecs::{Join, ReadStorage, System, WriteStorage}, renderer::Camera};
use amethyst::ecs::Read;

use crate::components::vehicle_components::VehicleComponents;
use crate::resources::system_active_flag::SystemActive;

pub struct CameraTrackingSystem;

impl<'s> System<'s> for CameraTrackingSystem {
    type SystemData = (
        ReadStorage<'s, VehicleComponents>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, SystemActive>,
    );

    fn run(&mut self, (vehicles, mut transforms, cameras, system_active): Self::SystemData) {
        if !system_active.is_active {
            return;
        }
        if let Some(vehicle) = (&vehicles).join().next() {
            for (_, camera_transform) in (&cameras, &mut transforms).join() {
                camera_transform.set_translation_x(vehicle.base.position.x);
                camera_transform.set_translation_y(vehicle.base.position.y);
            }
        }
    }
}