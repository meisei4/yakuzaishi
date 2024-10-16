use bevy::{
    core::Name,
    prelude::{Bundle, SpriteBundle},
    sprite::TextureAtlas,
};

use crate::{
    anime::anime_components::{AnimationComponent, AnimationTimer},
    kinetic_components::KineticEntityComponents,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub kinetics: KineticEntityComponents,
    pub sprite_sheet: SpriteBundle,
    pub texture_atlas: TextureAtlas,
}

#[derive(Bundle)]
pub struct EnvironmentEntityBundle {
    pub name: Name,
    pub kinetics: KineticEntityComponents,
    pub sprite_sheet: SpriteBundle,
    pub texture_atlas: TextureAtlas,
    pub animation_component: AnimationComponent,
    pub animation_timer: AnimationTimer,
}
