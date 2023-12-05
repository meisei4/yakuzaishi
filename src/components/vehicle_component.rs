use amethyst::{
    core::math::Vector2,
    ecs::prelude::{Component, DenseVecStorage},
};

pub struct Vehicle {
    pub speed: f32,              // The current speed of the vehicle
    pub max_speed: f32,          // The maximum speed the vehicle can reach
    pub acceleration: f32,       // The rate at which the vehicle increases its speed
    pub deceleration: f32,       // The rate at which the vehicle decreases its speed
    pub position: Vector2<f32>,  // The current position of the vehicle in the game world
    pub direction: Vector2<f32>, // The direction the vehicle is facing (2D vector)
    pub rotation_speed: f32,     // The speed at which the vehicle can rotate or turn
                                 
    // pub pill_stock: i32,         // The health of the vehicle, if applicable
    // pub gas: i32,
}

impl Component for Vehicle {
    type Storage = DenseVecStorage<Self>;
}

impl Vehicle {

    pub const DEFAULT_MAX_SPEED: f32 = 100.0; 
    pub const DEFAULT_ACCELERATION: f32 = 5.0; 
    pub const DEFAULT_DECELERATION: f32 = 5.0; 
    pub const DEFAULT_ROTATION_RATE: f32 = 0.1; 

    pub fn new() -> Self {
        Vehicle {
            speed: 0.0,
            max_speed: Self::DEFAULT_MAX_SPEED,
            acceleration: Self::DEFAULT_ACCELERATION,
            deceleration: Self::DEFAULT_DECELERATION,
            position: Vector2::new(0.0, 0.0),
            direction: Vector2::new(1.0, 0.0),
            rotation_speed: Self::DEFAULT_ROTATION_RATE,
        }
    }
    pub fn accelerate(&mut self, delta_time: f32) {
        self.speed += self.acceleration * delta_time;
        if self.speed > self.max_speed {
            self.speed = self.max_speed;
        }
    }

    pub fn decelerate(&mut self, delta_time: f32) {
        self.speed -= self.deceleration * delta_time;
        if self.speed < 0.0 {
            self.speed = 0.0;
        }
    }

    pub fn turn_left(&mut self, delta_time: f32) {
        let rotation_amount = self.rotation_speed * delta_time;
        let new_direction_angle = self.direction_angle() - rotation_amount;
        self.direction = Vector2::new(new_direction_angle.cos(), new_direction_angle.sin());
    }

    pub fn turn_right(&mut self, delta_time: f32) {
        let rotation_amount = self.rotation_speed * delta_time;
        let new_direction_angle = self.direction_angle() + rotation_amount;
        self.direction = Vector2::new(new_direction_angle.cos(), new_direction_angle.sin());
    }

    fn direction_angle(&self) -> f32 {
        self.direction.y.atan2(self.direction.x)
    }

    pub fn update_position(&mut self, delta_time: f32) {
        let displacement =
            Vector2::new(self.direction.x * self.speed, self.direction.y * self.speed) * delta_time;
        self.position.x += displacement.x;
        self.position.y += displacement.y;
    }

}