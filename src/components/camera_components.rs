use amethyst::{
    core::Transform,
    ecs::{Component, DenseVecStorage},
    renderer::Camera,
};

pub struct CameraComponents {
    pub camera: Camera,
    pub transform: Transform,
}

impl CameraComponents {
    pub fn new(camera_width: f32, camera_height: f32, x: f32, y: f32, z: f32) -> Self {
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, z);

        CameraComponents {
            camera: Camera::standard_2d(camera_width, camera_height),
            transform,
        }
    }
}

impl Component for CameraComponents {
    type Storage = DenseVecStorage<Self>;
}
