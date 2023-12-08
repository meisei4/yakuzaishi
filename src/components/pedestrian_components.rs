use amethyst::{
    core::math::{ArrayStorage, Matrix, Vector2, U1, U2},
    ecs::{Component, DenseVecStorage},
};

pub struct PedestrianComponents {
    pub speed: f32,                  // Fixed walking speed
    pub position: Vector2<f32>,      // Current position
    pub direction: WalkingDirection, // Current walking direction
    pub current_sprite_index: usize, // Sprite index for current direction
}

// Enum for 8 discrete walking directions
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
    // Initialize with position and default direction (e.g., North)
    // ...

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
            WalkingDirection::North => 0,
            WalkingDirection::Northeast => 1,
            WalkingDirection::East => 2,
            WalkingDirection::Southeast => 3,
            WalkingDirection::South => 4,
            WalkingDirection::Southwest => 5,
            WalkingDirection::West => 6,
            WalkingDirection::Northwest => 7,
        };
    }
}
