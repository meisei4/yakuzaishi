use bevy::prelude::Entity;

use crate::components::overlay_animation::AnimationType;
use crate::components::overlay_animation::AnimationType::Wake;

pub fn determine_animation_type(entity: Entity) -> AnimationType {
    Wake //TODO: add logic to determine what animation type is to occur for both time this is called
}
