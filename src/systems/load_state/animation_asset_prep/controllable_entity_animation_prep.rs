use bevy::prelude::{Commands, Entity, Query, Res, SpriteSheetBundle, TextureAtlas, With};

use crate::components::controllable_entity_components::ControllableEntityComponents;
use crate::CONTROLLABLE_ENTITY_ANIMATION_TEXTURE_START_IDX;
use crate::resources::animation_resources::ControlledAnimationResource;

pub fn attach_controlled_animations_to_controllable_entities(
    mut commands: Commands,
    vehicle_animation_resource: Res<ControlledAnimationResource>,
    query: Query<Entity, With<ControllableEntityComponents>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(SpriteSheetBundle {
            texture: vehicle_animation_resource.image_handle.clone(),
            atlas: TextureAtlas {
                layout: vehicle_animation_resource.texture_atlas.clone(),
                index: CONTROLLABLE_ENTITY_ANIMATION_TEXTURE_START_IDX,
            },
            ..Default::default()
        });
    }
}
