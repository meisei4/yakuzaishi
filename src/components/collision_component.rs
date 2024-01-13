use amethyst::{
    core::math::Vector2,
    ecs::prelude::{Component, DenseVecStorage},
};

pub struct CollisionComponent {
    pub size: Vector2<f32>,
}

impl Component for CollisionComponent {
    type Storage = DenseVecStorage<Self>;
}
