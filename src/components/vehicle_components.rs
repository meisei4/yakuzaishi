use amethyst::{
    core::math::Vector2,
    ecs::prelude::{Component, DenseVecStorage},
};

use crate::components::base_components::BaseEntityComponents;
use crate::TILE_SIZE;

#[derive(Clone)]
pub struct VehicleComponents {
    pub base: BaseEntityComponents,
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub direction: Vector2<f32>,
    pub rotation_speed: f32,
    pub current_tile: Vector2<u32>,
}

impl VehicleComponents {
    pub const DEFAULT_MAX_SPEED: f32 = 150.0;
    pub const DEFAULT_ACCELERATION: f32 = 80.0;
    pub const DEFAULT_DECELERATION: f32 = 100.0;
    pub const DEFAULT_ROTATION_RATE: f32 = 4.0;

    pub fn new(spawn_position_x: f32, spawn_position_y: f32) -> Self {
        // TODO: the "current_tile" attribute is literally only used for logging and even then i think somethings wrong with it.
        let initial_tile = Vector2::new(
            (spawn_position_x / TILE_SIZE) as u32,
            (spawn_position_y / TILE_SIZE) as u32,
        );
        VehicleComponents {
            // TODO: should i remove this pathetic attempt at composition? (remove BaseComponents?)
            base: BaseEntityComponents {
                position: Vector2::new(spawn_position_x, spawn_position_y),
                speed: 0.0,
                current_sprite_index: 36,
            },
            max_speed: Self::DEFAULT_MAX_SPEED,
            acceleration: Self::DEFAULT_ACCELERATION,
            deceleration: Self::DEFAULT_DECELERATION,
            direction: Vector2::new(0.0, 1.0),
            rotation_speed: Self::DEFAULT_ROTATION_RATE,
            current_tile: initial_tile,
        }
    }
}

impl Component for VehicleComponents {
    type Storage = DenseVecStorage<Self>;
}
