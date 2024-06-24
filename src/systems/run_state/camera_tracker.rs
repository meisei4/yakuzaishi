use bevy::prelude::{Camera, Query, Transform, With};

use crate::components::flying_entity_components::FlyingEntityComponents;

pub fn camera_tracking_system(
    //entity_data_query: Query<&RotationalVehicleComponents>,
    entity_data_query: Query<&FlyingEntityComponents>,
    mut camera_transforms: Query<&mut Transform, With<Camera>>,
) {
    if let Some(entity) = entity_data_query.iter().next() {
        for mut camera_transform in camera_transforms.iter_mut() {
            camera_transform.translation.x = entity.world_coordinate_position.x;
            camera_transform.translation.y = entity.world_coordinate_position.y;
        }
    }
}
