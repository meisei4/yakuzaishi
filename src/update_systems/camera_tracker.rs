use bevy::prelude::{Camera, Query, Transform, With};

use crate::components::flying_entity_components::FlyingEntityComponents;

pub fn camera_tracking_system(
    flying_entity_data_query: Query<&FlyingEntityComponents>,
    mut camera_transforms: Query<&mut Transform, With<Camera>>,
) {
    if let Some(flying_entity) = flying_entity_data_query.iter().next() {
        for mut camera_transform in camera_transforms.iter_mut() {
            camera_transform.translation.x = flying_entity.world_coordinate_position.x;
            camera_transform.translation.y = flying_entity.world_coordinate_position.y;
        }
    }
}
