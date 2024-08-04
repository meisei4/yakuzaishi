use bevy::math::Vec3;
use bevy::prelude::Component;

// TODO: you need to figure out how to rename this
//  OR!! just provide a new component thing to properly distinguish Controllable entities when
//  it comes to QUERYING

#[derive(Component, Clone)]
pub struct ControllableEntityComponents {
    pub y_axis_displacement: f32,
    pub x_axis_displacement: f32,
    pub position: Vec3,
    pub prev_position: Vec3,
}

// NOTE: these are based 100% on https://bevy-cheatbook.github.io/cookbook/smooth-movement.html
