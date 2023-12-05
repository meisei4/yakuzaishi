pub mod camera {
    use amethyst::{
        core::transform::Transform,
        ecs::prelude::{World, WorldExt},
        prelude::*,
        renderer::Camera,
    };
    use log::info;

    use crate::{TILE_SIZE, MAP_WIDTH, MAP_HEIGHT};

    pub fn init_camera(world: &mut World) {
        // Calculate the centered position of the camera
        let camera_x = (MAP_WIDTH as f32 * TILE_SIZE) / 2.0;
        let camera_y = (MAP_HEIGHT as f32 * TILE_SIZE) / 2.0;
        let camera_z = 1.0; // Camera's depth in the world

        // Set the camera to view the entire map
        let mut transform: Transform = Transform::default();
        transform.set_translation_xyz(camera_x, camera_y, camera_z);

        // Assuming that the map dimensions are smaller than the window dimensions,
        // adjust the camera's projection to add padding equal to half a tile's size
        let camera_width = MAP_WIDTH as f32 * TILE_SIZE + TILE_SIZE;
        let camera_height = MAP_HEIGHT as f32 * TILE_SIZE + TILE_SIZE;

        world
            .create_entity()
            .with(Camera::standard_2d(camera_width, camera_height))
            .with(transform)
            .build();

        info!("Initialized camera to view the entire map with adjusted boundaries.");
    }
}
