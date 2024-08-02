use bevy::prelude::{Component, Vec2};

#[derive(Component, Clone)]
pub struct ControllableEntityComponents {
    pub y_axis_speed: f32,
    pub x_axis_strafe_speed: f32,
    pub current_sprite_index: usize,
    pub direction: Vec2,
}

impl ControllableEntityComponents {
    pub fn new() -> Self {
        ControllableEntityComponents {
            y_axis_speed: 0.0,
            x_axis_strafe_speed: 70.0,
            current_sprite_index: 36,
            direction: Vec2::new(0.0, 1.0),
        }
    }
}
