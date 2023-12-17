use crate::components::base_components::BaseEntityComponents;
use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

pub struct PedestrianComponents {
    pub base: BaseEntityComponents,
    pub direction: WalkingDirection,
}

#[derive(Copy, Clone)]
pub enum WalkingDirection {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

impl PedestrianComponents {
    pub const DEFAULT_SPEED: f32 = 15.0;

    pub fn new(spawn_position_x: f32, spawn_position_y: f32) -> Self {
        PedestrianComponents {
            base: BaseEntityComponents::new(
                Vector2::new(spawn_position_x, spawn_position_y),
                Self::DEFAULT_SPEED,
                0,
            ),
            direction: WalkingDirection::North,
        }
    }
}

impl Component for PedestrianComponents {
    type Storage = DenseVecStorage<Self>;
}
