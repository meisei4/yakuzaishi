use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component)]
pub enum BehaviorPattern {
    FloatInCircle,
    ZigZag,
}

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
