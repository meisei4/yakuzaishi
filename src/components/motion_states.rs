use bevy::math::Vec3;
use bevy::prelude::Component;

#[derive(Component)]
pub struct CurrentMotionState {
    pub position: Vec3,
    pub motion: Vec3, // this is the vector of where the object is moving
}

#[derive(Component)]
pub struct OldMotionState {
    pub position: Vec3,
}
