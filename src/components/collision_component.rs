use amethyst::{
    core::math::Vector2,
    ecs::prelude::{Component, DenseVecStorage},
};

pub struct CollisionComponent {
    pub size: Vector2<f32>, // Size of the bounding box, adjust as needed
                            // Add additional fields as needed
}

impl Component for CollisionComponent {
    type Storage = DenseVecStorage<Self>;
}
