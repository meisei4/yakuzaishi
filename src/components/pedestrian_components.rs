use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

use crate::components::base_components::BaseEntityComponents;

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
    pub fn new(spawn_position_x: f32, spawn_position_y: f32) -> Self {
        PedestrianComponents {
            base: BaseEntityComponents {
                //transform: create_transform(spawn_position_x, spawn_position_x),
                position: Vector2::new(spawn_position_x, spawn_position_y),
                speed: 15.0,
                current_sprite_index: 0,
            },
            direction: WalkingDirection::North,
        }
    }
}

impl Component for PedestrianComponents {
    type Storage = DenseVecStorage<Self>;
}
