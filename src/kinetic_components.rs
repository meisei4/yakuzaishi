use bevy::math::Vec3;
use bevy::prelude::Component;

#[derive(Component)]
pub struct KineticEntityComponents {
    pub y_axis_displacement: f32,
    pub x_axis_displacement: f32,
    pub position: Vec3,
    pub prev_position: Vec3,
}

#[derive(Component)]
pub struct PlayerEntityTag;

#[derive(Component)]
pub struct EnvironmentEntityTag;
