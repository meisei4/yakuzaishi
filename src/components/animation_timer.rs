use bevy::prelude::{Component, Deref, DerefMut, Timer};

#[derive(Component, Deref, DerefMut, Debug)]
pub struct AnimationTimer(pub Timer);
