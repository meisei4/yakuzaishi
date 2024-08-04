use bevy::prelude::*;
use bevy::prelude::Component;

#[derive(Component)]
pub enum BehaviorPattern {
    FloatInCircle,
    ZigZag,
}

#[derive(Component)]
pub struct EnvironmentEntityComponents {
    pub y_axis_displacement: f32,
    pub x_axis_displacement: f32,
    pub position: Vec3,
    pub prev_position: Vec3,
}
