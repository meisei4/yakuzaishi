use amethyst::ecs::{Component, DenseVecStorage};

pub struct CameraComponents {
    pub lens_width: f32,
    pub lens_height: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Component for CameraComponents {
    type Storage = DenseVecStorage<Self>;
}
