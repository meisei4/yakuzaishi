pub mod camera {
    use amethyst::{
        core::transform::Transform,
        ecs::prelude::{World, WorldExt},
        prelude::*,
        renderer::Camera,
    };
    use log::info;

    pub const CAMERA_POSITION_X: f32 = 250.0;
    pub const CAMERA_POSITION_Y: f32 = 250.0;
    pub const CAMERA_POSITION_Z: f32 = 1.0;
    pub const CAMERA_WIDTH: f32 = 500.0;
    pub const CAMERA_HEIGHT: f32 = 500.0;

    pub fn init_camera(world: &mut World) {
        let mut transform: Transform = Transform::default();
        transform.set_translation_xyz(CAMERA_POSITION_X, CAMERA_POSITION_Y, CAMERA_POSITION_Z); // Adjust camera position as needed
    
        world
            .create_entity()
            .with(Camera::standard_2d(CAMERA_WIDTH, CAMERA_HEIGHT))
            .with(transform)
            .build();
        info!("Initialized camera with position ({}, {}, {})", CAMERA_POSITION_X, CAMERA_POSITION_Y, CAMERA_POSITION_Z);
    }
}