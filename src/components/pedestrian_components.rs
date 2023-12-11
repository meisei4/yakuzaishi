use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

pub struct PedestrianComponents {
    pub speed: f32,
    pub position: Vector2<f32>,
    pub direction: WalkingDirection,
    pub current_sprite_index: usize, // Sprite index for current direction (add animation later)
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

impl Component for PedestrianComponents {
    type Storage = DenseVecStorage<Self>;
}

impl PedestrianComponents {
    pub const DEFAULT_SPEED: f32 = 15.0;

    pub fn new(spawn_position_x: f32, spawn_position_y: f32) -> Self {
        PedestrianComponents {
            speed: Self::DEFAULT_SPEED,
            position: Vector2::new(spawn_position_x, spawn_position_y),
            direction: WalkingDirection::North,
            current_sprite_index: 0,
        }
    }
}
