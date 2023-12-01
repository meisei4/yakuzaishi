use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::core::math::{Vector2, Vector3};

pub struct Vehicle {
    pub speed: f32,           // The current speed of the vehicle
    pub max_speed: f32,       // The maximum speed the vehicle can reach
    pub acceleration: f32,    // The rate at which the vehicle increases its speed
    pub deceleration: f32,    // The rate at which the vehicle decreases its speed
    pub position: Vector3<f32>, // The current position of the vehicle in the game world
    pub direction: Vector2<f32>, // The direction the vehicle is facing (2D vector)
    pub rotation_speed: f32,  // The speed at which the vehicle can rotate or turn
    pub pill_stock: i32,          // The health of the vehicle, if applicable
    pub gas: i32,

}

impl Component for Vehicle {
    type Storage = DenseVecStorage<Self>;
}

impl Vehicle {
    // Accelerates the vehicle, increasing its speed
    pub fn accelerate(&mut self, delta_time: f32) {
        self.speed += self.acceleration * delta_time;
        if self.speed > self.max_speed {
            self.speed = self.max_speed;
        }
    }

    // Decelerates the vehicle, decreasing its speed
    pub fn decelerate(&mut self, delta_time: f32) {
        self.speed -= self.deceleration * delta_time;
        if self.speed < 0.0 {
            self.speed = 0.0;
        }
    }

    // Turns the vehicle to the left
    pub fn turn_left(&mut self, delta_time: f32) {
        let rotation_amount = self.rotation_speed * delta_time;
        let new_direction_angle = self.direction_angle() - rotation_amount;
        self.direction = Vector2::new(new_direction_angle.cos(), new_direction_angle.sin());
    }

    // Turns the vehicle to the right
    pub fn turn_right(&mut self, delta_time: f32) {
        let rotation_amount = self.rotation_speed * delta_time;
        let new_direction_angle = self.direction_angle() + rotation_amount;
        self.direction = Vector2::new(new_direction_angle.cos(), new_direction_angle.sin());
    }

    // Helper function to get the current angle of direction
    fn direction_angle(&self) -> f32 {
        self.direction.y.atan2(self.direction.x)
    }

    // Updates the position of the vehicle based on its speed and direction
    pub fn update_position(&mut self, delta_time: f32) {
        let displacement = Vector2::new(self.direction.x * self.speed, self.direction.y * self.speed) * delta_time;
        self.position.x += displacement.x;
        self.position.y += displacement.y;
    }
}
