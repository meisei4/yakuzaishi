use bevy::prelude::*;

#[derive(Component)]
pub struct FloatInCircle {
    pub radius: f32,
    pub speed: f32,
    pub angle: f32,
}

#[derive(Component)]
pub struct ZigZag {
    pub speed: f32,
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Bounce {
    pub speed: Vec2,
    pub bounds: Vec2,
}
