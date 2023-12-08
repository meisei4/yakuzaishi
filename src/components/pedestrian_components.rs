use amethyst::{
    core::math::{ArrayStorage, Matrix, Vector2, U1, U2},
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

    pub fn update_position(&mut self, delta_time: f32) {
        let movement: Matrix<f32, U2, U1, ArrayStorage<f32, U2, U1>> = match self.direction {
            WalkingDirection::North => Vector2::new(0.0, 1.0),
            WalkingDirection::Northeast => Vector2::new(1.0, 1.0),
            WalkingDirection::East => Vector2::new(1.0, 0.0),
            WalkingDirection::Southeast => Vector2::new(1.0, -1.0),
            WalkingDirection::South => Vector2::new(0.0, -1.0),
            WalkingDirection::Southwest => Vector2::new(-1.0, -1.0),
            WalkingDirection::West => Vector2::new(-1.0, 0.0),
            WalkingDirection::Northwest => Vector2::new(-1.0, 1.0),
        };
        self.position += movement.normalize() * self.speed * delta_time;
    }

    // Update the sprite index based on the direction
    pub fn update_sprite_index(&mut self) {
        // Assuming a sprite sheet where each direction has a corresponding sprite
        self.current_sprite_index = match self.direction {
            WalkingDirection::North => 2,
            WalkingDirection::Northeast => 2,
            WalkingDirection::East => 1,
            WalkingDirection::Southeast => 1,
            WalkingDirection::South => 0,
            WalkingDirection::Southwest => 0,
            WalkingDirection::West => 3,
            WalkingDirection::Northwest => 3,
        };
    }
}
