use bevy::math::Vec3;
use bevy::prelude::Component;

#[derive(Component)]
pub struct PlayerEntityComponents {
    pub y_axis_displacement: f32,
    pub x_axis_displacement: f32,
    pub position: Vec3,
    pub prev_position: Vec3,
}
