use amethyst::{core::Transform, ecs::{Join, ReadStorage, System, WriteStorage}, renderer::Camera, SystemDesc};
use amethyst::ecs::{AccessorCow, RunningTime, World};

use crate::components::game_map_tile_components::TileType;
use crate::components::vehicle_components::VehicleComponents;
use crate::resources::game_map_resource::GameMapResource;

#[derive(SystemDesc)]
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
            log::debug!(
                "Vehicle Position: {:?}, Camera Position: {:?}",
                vehicle.base.position,
                camera_transform.translation()
            );
        }
    }
}