use bevy::prelude::{Camera2dBundle, Commands, Transform};

use crate::{CAMERA_HEIGHT, CAMERA_WIDTH, MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};

pub fn init_camera(mut command_buffer: Commands) {
    let tile_centering_offset = TILE_SIZE / 2.0;
    let camera_x = (MAP_WIDTH * TILE_SIZE) / 2.0 - tile_centering_offset;
    let camera_y = (MAP_HEIGHT * TILE_SIZE) / 2.0 - tile_centering_offset;
    let camera_z = 1.0;

    command_buffer.spawn(Camera2dBundle {
        transform: Transform::from_xyz(camera_x, camera_y, camera_z),
        ..Default::default()
    });

    log::info!("Camera initialized at position ({}, {}) with viewport dimensions ({}, {})", camera_x, camera_y, CAMERA_WIDTH, CAMERA_HEIGHT);
}
