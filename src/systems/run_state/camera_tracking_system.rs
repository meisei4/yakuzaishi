use bevy::math::Vec2;
use bevy::prelude::{Camera, ParamSet, Query, Transform, With};

use crate::components::controllable_entity_components::VelocityVectorComponents;

pub fn camera_tracking_system(
    mut param_set: ParamSet<(
        Query<&Transform, With<VelocityVectorComponents>>,
        Query<&mut Transform, With<Camera>>,
    )>,
) {
    let mut temp_translation = Vec2 { x: 0.0, y: 0.0 };
    for entity_transform in param_set.p0().iter_mut() {
        temp_translation.x = entity_transform.translation.x;
        temp_translation.y = entity_transform.translation.y;
    }
    for mut camera_transform in param_set.p1().iter_mut() {
        camera_transform.translation.x = temp_translation.x;
        camera_transform.translation.y = temp_translation.y;
    }
}
