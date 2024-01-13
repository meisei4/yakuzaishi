use amethyst::core::Transform;
use amethyst::renderer::Camera;
use amethyst::{
    ecs::prelude::{World, WorldExt},
    prelude::*,
};

use crate::{CAMERA_HEIGHT, CAMERA_WIDTH, MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};

pub fn init_camera(world: &mut World) {
    let tile_centering_offset = TILE_SIZE / 2.0;
    let camera_x = (MAP_WIDTH * TILE_SIZE) / 2.0 - tile_centering_offset;
    let camera_y = (MAP_HEIGHT * TILE_SIZE) / 2.0 - tile_centering_offset;
    let camera_z = 1.0;

    let mut transform = Transform::default();
    transform.set_translation_xyz(camera_x, camera_y, camera_z);

    world
        .create_entity()
        // .with(CameraComponents {
        //     lens_width: CAMERA_WIDTH,
        //     lens_height: CAMERA_HEIGHT,
        //     x: camera_x,
        //     y: camera_y,
        //     z: camera_z,
        // })
        .with(Camera::standard_2d(CAMERA_WIDTH, CAMERA_HEIGHT))
        .with(transform)
        .build();
}
