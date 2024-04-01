use bevy::prelude::{Camera, Camera2dBundle, Commands, OrthographicProjection, Transform};
use bevy::render::camera::ScalingMode;

use crate::{CAMERA_HEIGHT, CAMERA_WIDTH, MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};

pub fn init_camera(mut command_buffer: Commands) {
    let tile_centering_offset = TILE_SIZE / 2.0;
    let camera_x = (MAP_WIDTH * TILE_SIZE) / 2.0 - tile_centering_offset;
    let camera_y = (MAP_HEIGHT * TILE_SIZE) / 2.0 - tile_centering_offset;
    let camera_z = 2.0;

    command_buffer.spawn(Camera2dBundle {
        transform: Transform::from_xyz(camera_x, camera_y, camera_z),
        camera: Camera {
            order: 1, // to avoid the ambiguity warning:  Camera order ambiguities detected for active cameras with the following priorities: {(0, Some(Window(NormalizedWindowRef(0v1))))}
            ..Default::default()
        },
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Fixed { width: 251.0, height: 251.0 },
            scale: 0.5,
            ..Default::default()
        },
        ..Default::default()
    });

    log::info!("Camera initialized at position ({}, {}) with viewport dimensions ({}, {})", camera_x, camera_y, CAMERA_WIDTH, CAMERA_HEIGHT);
}
