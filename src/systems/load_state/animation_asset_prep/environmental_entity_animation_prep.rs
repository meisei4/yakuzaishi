use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{
    Commands, Entity, Query, Res, SpriteSheetBundle, TextureAtlas, Timer, TimerMode, Transform,
    With,
};

use crate::components::animation_components::AnimationTimer;
use crate::components::environmental_entity::EnvironmentalEntityComponents;
use crate::ENVIRONMENTAL_ENTITY_ANIMATION_TEXTURE_START_IDX;
use crate::resources::animation_resources::EnvironmentalEntityAnimationResource;

pub fn attach_overlay_animations_to_environmental_entities(
    mut commands: Commands,
    overlay_animation_data: Res<EnvironmentalEntityAnimationResource>,
    query: Query<Entity, With<EnvironmentalEntityComponents>>,
) {
    for entity in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn(SpriteSheetBundle {
                    texture: overlay_animation_data.animation_image_handle.clone(),
                    atlas: TextureAtlas {
                        layout: overlay_animation_data.animation_texture_atlas.clone(),
                        index: ENVIRONMENTAL_ENTITY_ANIMATION_TEXTURE_START_IDX,
                    },
                    transform: Transform::default(), // gets overwritten by the parent??
                    ..Default::default()
                })
                .insert(AnimationTimer(Timer::from_seconds(
                    overlay_animation_data.animation.speed,
                    TimerMode::Repeating,
                )))
                .insert(Name::new("OverlayAnimation2"));
        });
    }
}
