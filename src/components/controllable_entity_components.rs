use bevy::math::Vec3;
use bevy::prelude::Component;

#[derive(Component, Clone)]
pub struct VelocityVectorComponents {
    pub y_axis_speed: f32,
    pub x_axis_strafe_speed: f32,
}

impl VelocityVectorComponents {
    pub fn new() -> Self {
        VelocityVectorComponents {
            y_axis_speed: 0.0,
            x_axis_strafe_speed: 0.0,
        }
    }
}

// NOTE: these are based 100% on https://bevy-cheatbook.github.io/cookbook/smooth-movement.html

#[derive(Component)]
pub struct PositionComponent {
    pub position: Vec3,
}

#[derive(Component)]
pub struct PreviousPositionComponent {
    pub position: Vec3,
}
