use bevy::prelude::*;
use bevy::prelude::Component;

#[derive(Component)]
pub enum BehaviorPattern {
    FloatInCircle,
    ZigZag,
}

// TODO: This needs to be used when the attach
#[derive(Component)]
pub struct EnvironmentalEntityComponents {
    //pub behavior: BehaviorPattern, TODO not sure if necessary
    pub y_axis_displacement: f32,
    pub x_axis_displacement: f32,
    pub position: Vec3,
    pub prev_position: Vec3,
}
