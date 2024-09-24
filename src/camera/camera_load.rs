use bevy::prelude::{Camera2dBundle, Commands, OrthographicProjection, Transform};

use crate::{CAMERA_SCALE_MULTIPLIER, CAMERA_Z_LEVEL};

pub fn init_camera(mut command_buffer: Commands) {
    command_buffer.spawn(Camera2dBundle {
        //TODO: the 0.0, 0.0, is ugly here since only the Z value here is needed, and the rest could be initialized in the tracker?
        transform: Transform::from_xyz(0.0, 0.0, CAMERA_Z_LEVEL),
        projection: OrthographicProjection {
            scale: CAMERA_SCALE_MULTIPLIER,
            ..Default::default()
        },
        ..Default::default()
    });
}
