use bevy::prelude::{Component, Vec2};

use crate::{DEFAULT_RATE_OF_ROTATION, TILE_SIZE};

#[derive(Component, Clone)]
pub struct FlyingEntityComponents {
    pub tile_coordinate_position: Vec2,
    pub world_coordinate_position: Vec2,
    pub y_axis_speed: f32,
    pub x_axis_strafe_speed: f32,
    pub current_sprite_index: usize,
    pub direction: Vec2,
    pub rotation_speed: f32,
}

impl FlyingEntityComponents {
    pub fn new(tile_spawn_coordinates: Vec2) -> Self {
        FlyingEntityComponents {
            tile_coordinate_position: tile_spawn_coordinates,
            world_coordinate_position: Vec2 {
                x: tile_spawn_coordinates.x * TILE_SIZE,
                y: tile_spawn_coordinates.y * TILE_SIZE,
            },
            y_axis_speed: 0.0,
            x_axis_strafe_speed: 70.0,
            current_sprite_index: 36,
            direction: Vec2::new(0.0, 1.0),
            rotation_speed: DEFAULT_RATE_OF_ROTATION,
        }
    }
}
