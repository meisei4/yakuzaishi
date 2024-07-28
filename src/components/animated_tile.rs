use bevy::prelude::Component;

#[derive(Component)]
pub struct AnimatedTile {
    pub start_idx: u32,
    pub end_idx: u32,
    pub speed: f32,
}
