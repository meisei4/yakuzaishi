use bevy::prelude::{Component, Vec2};

use crate::{
    TILE_SIZE, VEHICLE_DEFAULT_ACCELERATION, VEHICLE_DEFAULT_DECELERATION,
    VEHICLE_DEFAULT_MAX_SPEED, VEHICLE_DEFAULT_RATE_OF_ROTATION,
};

#[derive(Component, Clone)]
pub struct RotationalVehicleComponents {
    pub tile_coordinate_position: Vec2,
    pub world_coordinate_position: Vec2,
    pub speed: f32,
    pub current_sprite_index: usize,
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub direction: Vec2,
    pub rotation_speed: f32,
}

impl RotationalVehicleComponents {
    pub fn new(tile_spawn_coordinates: Vec2) -> Self {
        RotationalVehicleComponents {
            tile_coordinate_position: tile_spawn_coordinates,
            world_coordinate_position: Vec2 {
                x: tile_spawn_coordinates.x * TILE_SIZE,
                y: tile_spawn_coordinates.y * TILE_SIZE,
            },
            speed: 0.0,
            current_sprite_index: 36,
            max_speed: VEHICLE_DEFAULT_MAX_SPEED,
            acceleration: VEHICLE_DEFAULT_ACCELERATION,
            deceleration: VEHICLE_DEFAULT_DECELERATION,
            direction: Vec2::new(0.0, 1.0),
            rotation_speed: VEHICLE_DEFAULT_RATE_OF_ROTATION,
        }
    }
}
