use amethyst::{
    core::math::Vector2,
    ecs::prelude::{Component, DenseVecStorage},
};

use crate::components::base_components::BaseEntityComponents;
use crate::util::create_transform;

pub struct VehicleComponents {
    pub base: BaseEntityComponents,
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub direction: Vector2<f32>,
    pub rotation_speed: f32,
    // pub pill_stock: i32,
    // pub gas: i32,
}

impl VehicleComponents {
    pub const DEFAULT_MAX_SPEED: f32 = 150.0;
    pub const DEFAULT_ACCELERATION: f32 = 80.0;
    pub const DEFAULT_DECELERATION: f32 = 100.0;
    pub const DEFAULT_ROTATION_RATE: f32 = 4.0;

    pub fn new(spawn_position_x: f32, spawn_position_y: f32) -> Self {
        VehicleComponents {
            base: BaseEntityComponents {
                transform: create_transform(spawn_position_x, spawn_position_y),
                position: Vector2::new(spawn_position_x, spawn_position_y),
                speed: 0.0,
                current_sprite_index: 36,
            },
            max_speed: Self::DEFAULT_MAX_SPEED,
            acceleration: Self::DEFAULT_ACCELERATION,
            deceleration: Self::DEFAULT_DECELERATION,
            direction: Vector2::new(0.0, 1.0),
            rotation_speed: Self::DEFAULT_ROTATION_RATE,
            //TODO have to set the indices to something, even though they get overwritten immediately
        }
    }
}

impl Component for VehicleComponents {
    type Storage = DenseVecStorage<Self>;
}
