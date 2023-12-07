use std::f32::consts::{PI, FRAC_PI_2};

use amethyst::{
    core::math::{Vector2, ArrayStorage, U1, U2, Matrix},
    ecs::prelude::{Component, DenseVecStorage},
};
use log::info;


//LEARNING NOTE: Just figured out what components are, they are just the attributes of the entity. 
// suprisingly to me Entities are not neccessarily added as structs, but rather just built when 
/* for example this is how Entities get built. 
entities
        .build_entity()
        //other withs for whatever is needed to define the entity (maybe split up the VehicleComponents to be more specific here)
        .with(VehicleComponents::new(world_x, world_y), vehicle)
        .build();
}
*/
pub struct VehicleComponents {
    pub speed: f32,              // The current speed of the vehicle
    pub max_speed: f32,          // The maximum speed the vehicle can reach
    pub acceleration: f32,       // The rate at which the vehicle increases its speed
    pub deceleration: f32,       // The rate at which the vehicle decreases its speed
    pub position: Vector2<f32>,  // The current position of the vehicle in the game world
    pub direction: Vector2<f32>, // The direction the vehicle is facing (2D vector)
    pub rotation_speed: f32,     // The speed at which the vehicle can rotate or turn
    pub current_sprite_index: usize,     // sprite id from the sprite sheet for rotational rendering
    pub previous_sprite_index: usize,
                                 
    // pub pill_stock: i32,         // The health of the vehicle, if applicable
    // pub gas: i32,
}

impl Component for VehicleComponents {
    type Storage = DenseVecStorage<Self>;
}

impl VehicleComponents {

    pub const DEFAULT_MAX_SPEED: f32 = 30.0; 
    pub const DEFAULT_ACCELERATION: f32 = 45.0; 
    pub const DEFAULT_DECELERATION: f32 = 45.0; 
    pub const DEFAULT_ROTATION_RATE: f32 = 1.0; 

    pub fn new(spawn_position_x: f32, spawn_position_y: f32) -> Self {
        VehicleComponents {
            speed: 0.0,
            max_speed: Self::DEFAULT_MAX_SPEED,
            acceleration: Self::DEFAULT_ACCELERATION,
            deceleration: Self::DEFAULT_DECELERATION,
            position: Vector2::new(spawn_position_x, spawn_position_y),
            direction: Vector2::new(0.0, 1.0),
            rotation_speed: Self::DEFAULT_ROTATION_RATE,
            previous_sprite_index: 0, //??
            current_sprite_index: 1, //whjy the hell does this index at 1...

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
        let new_direction_angle = self.direction_angle() + rotation_amount;
        self.direction = Vector2::new(new_direction_angle.cos(), new_direction_angle.sin());
    }

    pub fn turn_right(&mut self, delta_time: f32) {
        let rotation_amount = self.rotation_speed * delta_time;
        let new_direction_angle = self.direction_angle() - rotation_amount;
        self.direction = Vector2::new(new_direction_angle.cos(), new_direction_angle.sin());
    }

    pub fn direction_angle(&self) -> f32 {
        self.direction.y.atan2(self.direction.x)
    }
    
    pub fn update_sprite_index(&mut self) {
        let angle_radians = self.direction_angle(); // Angle in radians from the positive x-axis
        let total_sprites = 48; // Total number of sprites in the sprite sheet
        let radians_per_sprite = (2.0 * std::f32::consts::PI) / total_sprites as f32;
        let north_sprite_index = 0; // Index of the sprite facing North

        // Calculate the index offset from North considering the angle
        let offset_from_north = ((angle_radians / radians_per_sprite).round() as isize - (std::f32::consts::PI / 2.0 / radians_per_sprite).round() as isize) % total_sprites as isize;
        let mut calculated_sprite_index = (north_sprite_index as isize + offset_from_north) % total_sprites as isize;
        if calculated_sprite_index < 0 {
            calculated_sprite_index += total_sprites as isize;
        }
        let calculated_sprite_index = calculated_sprite_index as usize;

        // Log only if there's a change in sprite index
        if calculated_sprite_index != self.current_sprite_index {
            let orientation_degrees = angle_radians.to_degrees();
            info!("Vehicle orientation: {} degrees", orientation_degrees);
            info!("Offset from North: {}", offset_from_north);
            info!("Previous sprite index: {}, New sprite index: {}", self.current_sprite_index, calculated_sprite_index);
            self.current_sprite_index = calculated_sprite_index; // Update the current sprite index
        }
    }

    pub fn update_position(&mut self, delta_time: f32) {
        let displacement: Matrix<f32, U2, U1, ArrayStorage<f32, U2, U1>> =
            Vector2::new(self.direction.x * self.speed, self.direction.y * self.speed) * delta_time;
        self.position.x += displacement.x;
        self.position.y += displacement.y;
    }

    pub fn adjust_speed(&mut self, forward_movement: f32, delta_time: f32) {
        if forward_movement > 0.0 {
            self.accelerate(delta_time * forward_movement);
        } else if forward_movement < 0.0 {
            self.decelerate(delta_time * -forward_movement);
        }
    }

    pub fn adjust_direction(&mut self, turn_movement: f32, delta_time: f32) {
        if turn_movement > 0.0 {
            self.turn_right(delta_time);
        } else if turn_movement < 0.0 {
            self.turn_left(delta_time);
        }
    }

}