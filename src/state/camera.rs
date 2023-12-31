use amethyst::{
    core::transform::Transform,
    ecs::prelude::{World, WorldExt},
    prelude::*,
    renderer::Camera,
};

use crate::{CAMERA_HEIGHT, CAMERA_WIDTH, MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};

pub fn init_camera(world: &mut World) {
    let tile_centering_offset = TILE_SIZE / 2.0;
    let camera_x: f32 = (MAP_WIDTH * TILE_SIZE) / 2.0 - tile_centering_offset; // actually gets to the middle of the tile with the offset
    let camera_y: f32 = (MAP_HEIGHT * TILE_SIZE) / 2.0 - tile_centering_offset;
    let camera_z: f32 = 1.0;

    let mut transform: Transform = Transform::default();
    transform.set_translation_xyz(camera_x, camera_y, camera_z);

    world
        .create_entity()
        .with(Camera::standard_2d(CAMERA_WIDTH, CAMERA_HEIGHT))
        .with(transform)
        .build();
}
