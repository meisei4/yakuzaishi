use bevy::prelude::Component;

// TODO: SERIOUSLY FIGURE OUT HOW TO ORGANIZE THESE KINDS OF OBJECTS BETTER

#[derive(Component)]
pub struct AnimatedTile {
    pub start_idx: u32,
    pub end_idx: u32,
    pub speed: f32,
}
