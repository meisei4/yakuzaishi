use bevy::prelude::{Component, Vec2};

// TODO there really needs to be a better way than this for shared attributes between components (maybe just write getters and setters...)
#[derive(Component, Clone)]
pub struct BaseEntityComponents {
    pub position: Vec2,
    pub speed: f32,
    pub current_sprite_index: usize,
}
