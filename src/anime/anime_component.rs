use bevy::prelude::{Component, Deref, DerefMut, Timer};

#[derive(Component, Clone, Copy)]
pub struct AnimationComponent {
    pub start_idx: u32,
    pub end_idx: u32,
    pub speed: f32,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
