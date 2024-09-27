use bevy::core::Name;
use bevy::prelude::{Bundle, SpriteSheetBundle};

use crate::anime::anime_components::{AnimationComponent, AnimationTimer};
use crate::kinetic_components::KineticEntityComponents;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub kinetics: KineticEntityComponents,
    pub sprite_sheet: SpriteSheetBundle,
}

#[derive(Bundle)]
pub struct EnvironmentEntityBundle {
    pub name: Name,
    pub kinetics: KineticEntityComponents,
    pub sprite_sheet: SpriteSheetBundle,
    pub animation_component: AnimationComponent,
    pub animation_timer: AnimationTimer,
}
