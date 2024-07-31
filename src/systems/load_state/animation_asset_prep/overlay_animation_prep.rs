use bevy::asset::{Assets, AssetServer};
use bevy::core::Name;
use bevy::hierarchy::BuildChildren;
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Entity, Query, Res, ResMut, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout,
    Transform, With,
};
use bevy::time::{Timer, TimerMode};

use crate::{CONTROLLABLE_ENTITY_ANIMATION_START_IDX, TILE_SIZE, WAKE_ANIMATION};
use crate::components::animation_components::{AnimationComponent, AnimationTimer};
use crate::components::controllable_entity_components::ControllableEntityComponents;
use crate::resources::animation_resources::OverlayAnimationResource;

pub fn insert_overlay_animation_resources_into_gameworld(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let overlay_animation_image = asset_server.load(WAKE_ANIMATION);

    let overlay_animation_texture_atlas = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::splat(TILE_SIZE),
        20,
        1,
        None,
        None,
    ));

    commands.insert_resource(OverlayAnimationResource {
        // TODO: perhaps move the initial values of these kinds of structs to a CONST module (like libs.rs)
        wake_animation: AnimationComponent {
            start_idx: 0,
            end_idx: 19,
            speed: 0.05,
        },
        overlay_animation_image_handle: overlay_animation_image,
        overlay_animation_texture_atlas,
    });
}

pub fn attach_overlay_animations_to_controllable_entities(
    mut commands: Commands,
    overlay_animation_data: Res<OverlayAnimationResource>,
    query: Query<Entity, With<ControllableEntityComponents>>,
) {
    for entity in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn(SpriteSheetBundle {
                    texture: overlay_animation_data
                        .overlay_animation_image_handle
                        .clone(),
                    atlas: TextureAtlas {
                        layout: overlay_animation_data
                            .overlay_animation_texture_atlas
                            .clone(),
                        index: CONTROLLABLE_ENTITY_ANIMATION_START_IDX,
                    },
                    transform: Transform::default(), // gets overwritten by the parent??
                    ..Default::default()
                })
                .insert(AnimationTimer(Timer::from_seconds(
                    overlay_animation_data.wake_animation.speed,
                    TimerMode::Repeating,
                )))
                .insert(Name::new("OverlayAnimation"));
        });
    }
}
