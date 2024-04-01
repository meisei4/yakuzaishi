use bevy::prelude::{Camera, Query, Transform, With};

use crate::components::vehicle_components::VehicleComponents;

pub fn camera_tracking_system(
    vehicles: Query<&VehicleComponents>,
    mut camera_transforms: Query<&mut Transform, With<Camera>>,
) {
    if let Some(vehicle) = vehicles.iter().next() {
        for mut camera_transform in camera_transforms.iter_mut() {
            camera_transform.translation.x = vehicle.world_coordinate_position.x;
            camera_transform.translation.y = vehicle.world_coordinate_position.y;
        }
    }
}
