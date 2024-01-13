use amethyst::{
    ecs::prelude::{World, WorldExt},
    prelude::*,
};

use crate::components::camera_components::CameraComponents;
use crate::{CAMERA_HEIGHT, CAMERA_WIDTH, MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};

pub fn init_camera(world: &mut World) {
    let tile_centering_offset = TILE_SIZE / 2.0;
    let camera_x: f32 = (MAP_WIDTH * TILE_SIZE) / 2.0 - tile_centering_offset;
    let camera_y: f32 = (MAP_HEIGHT * TILE_SIZE) / 2.0 - tile_centering_offset;
    let camera_z: f32 = 1.0;

    world
        .create_entity()
        .with(CameraComponents::new(
            CAMERA_WIDTH,
            CAMERA_HEIGHT,
            camera_x,
            camera_y,
            camera_z,
        ))
        .build();
}
