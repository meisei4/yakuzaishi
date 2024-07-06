use bevy::math::Vec3;
use bevy::prelude::Component;

#[derive(Component)]
pub struct CurrentMovementState {
    pub position: Vec3,
    pub movement: Vec3, // this is the vector of where the object is moving
}

#[derive(Component)]
pub struct PreviousMovementState {
    pub position: Vec3,
}
