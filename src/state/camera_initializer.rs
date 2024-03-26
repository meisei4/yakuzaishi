use amethyst::core::Transform;
use amethyst::ecs::prelude::WorldExt;
use amethyst::renderer::Camera;

use crate::{CAMERA_HEIGHT, CAMERA_WIDTH, MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};
use crate::command_buffer::command_buffer::{CommandBuffer, EntityCreationCommand};

pub fn init_camera(command_buffer: &mut CommandBuffer) {
    let tile_centering_offset = TILE_SIZE / 2.0;
    let camera_x = (MAP_WIDTH * TILE_SIZE) / 2.0 - tile_centering_offset;
    let camera_y = (MAP_HEIGHT * TILE_SIZE) / 2.0 - tile_centering_offset;
    let camera_z = 1.0;

    let camera = Camera::standard_2d(CAMERA_WIDTH, CAMERA_HEIGHT);

    let mut camera_transform = Transform::default();
    camera_transform.set_translation_xyz(camera_x, camera_y, camera_z);

    command_buffer.add_command(
        EntityCreationCommand::new()
            .with_camera(camera)
            .with_transform(camera_transform)
    );
}
