use bevy::{
    asset::Assets,
    core::Name,
    hierarchy::BuildChildren,
    math::UVec2,
    prelude::{
        Commands, Entity, Query, Res, ResMut, TextureAtlas, TextureAtlasLayout, Time, Timer,
        TimerMode, With,
    },
    sprite::SpriteBundle,
};

use crate::{
    anime::{
        anime_components::{AnimationComponent, AnimationTimer, OverlayAnimationTag},
        anime_res::OverlayAnimationAssets,
    },
    kinetic_components::PlayerEntityTag,
    TILE_SIZE, WAKE_ANIMATION_SPEED, WAKE_ANIMATION_TEXTURE_COLUMN_LENGTH,
    WAKE_ANIMATION_TEXTURE_END_IDX, WAKE_ANIMATION_TEXTURE_ROW_LENGTH,
    WAKE_ANIMATION_TEXTURE_START_IDX,
};

pub fn attach_overlay_animation_to_player_entity(
    mut commands: Commands,
    overlay_animation_assets: Res<OverlayAnimationAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    query: Query<Entity, With<PlayerEntityTag>>,
) {
    for entity in query.iter() {
        let overlay_animation_texture_atlas_layout =
            texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                UVec2::splat(TILE_SIZE as u32),
                WAKE_ANIMATION_TEXTURE_COLUMN_LENGTH,
                WAKE_ANIMATION_TEXTURE_ROW_LENGTH,
                None,
                None,
            ));

        let animation_component = AnimationComponent {
            start_idx: WAKE_ANIMATION_TEXTURE_START_IDX,
            end_idx: WAKE_ANIMATION_TEXTURE_END_IDX,
            speed: WAKE_ANIMATION_SPEED,
        };

        commands.entity(entity).with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    texture: overlay_animation_assets.animation_image_handle.clone(),
                    transform: Default::default(), // gets overwritten by the parent??
                    ..Default::default()
                })
                .insert(TextureAtlas {
                    layout: overlay_animation_texture_atlas_layout,
                    index: WAKE_ANIMATION_TEXTURE_START_IDX as usize,
                })
                .insert(animation_component)
                .insert(AnimationTimer(Timer::from_seconds(
                    animation_component.speed,
                    TimerMode::Repeating,
                )))
                .insert(Name::new("PlayerEntityOverlayAnimation"))
                .insert(OverlayAnimationTag);
        });
    }
}

pub fn animate_overlay_animations(
    time: Res<Time>,
    mut query: Query<
        (&mut AnimationTimer, &AnimationComponent, &mut TextureAtlas),
        With<OverlayAnimationTag>,
    >,
) {
    for (mut timer, animation, mut overlay_atlas) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            let current_index = overlay_atlas.index;

            let next_index = if current_index == animation.end_idx as usize {
                animation.start_idx as usize
            } else {
                current_index + 1
            };

            overlay_atlas.index = next_index;
        }
    }
}
