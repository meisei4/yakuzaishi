use bevy::math::Vec3;
use bevy::prelude::Component;

// TODO: you need to figure out how to rename this
//  OR!! just provide a new component thing to properly distinguish Controllable entities when
//  it comes to QUERYING
#[derive(Component, Clone)]
pub struct VelocityVectorComponents {
    pub y_axis_displacement: f32,
    pub x_axis_displacement: f32,
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
