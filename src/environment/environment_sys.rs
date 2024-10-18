use bevy::{
    core::Name,
    math::UVec2,
    prelude::{
        Commands, Res, ResMut, TextureAtlas, TextureAtlasLayout, Timer, TimerMode, Transform,
    },
    sprite::SpriteBundle,
};
use bevy_asset::Assets;

use crate::{
    anime::{
        anime_components::{AnimationComponent, AnimationTimer},
        anime_res::EnvironmentEntityAnimationAssets,
    },
    bundles::EnvironmentEntityBundle,
    kinetic_components::{EnvironmentEntityTag, KineticEntityComponents},
    ENVIRONMENT_ENTITY_ANIMATION_SPEED, ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_COLUMN_LENGTH,
    ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_END_IDX, ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_ROW_LENGTH,
    ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_START_IDX, ENVIRONMENT_ENTITY_SPAWN_X,
    ENVIRONMENT_ENTITY_SPAWN_Y, ENVIRONMENT_ENTITY_Z_LEVEL, TILE_SIZE,
};

pub fn spawn_environment_entity(
    mut commands: Commands,
    environment_entity_assets: Res<EnvironmentEntityAnimationAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let environment_texture_atlas_layout =
        texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(TILE_SIZE as u32),
            ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_COLUMN_LENGTH,
            ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_ROW_LENGTH,
            None,
            None,
        ));

    let transform = Transform::from_xyz(
        ENVIRONMENT_ENTITY_SPAWN_X * TILE_SIZE,
        ENVIRONMENT_ENTITY_SPAWN_Y * TILE_SIZE,
        ENVIRONMENT_ENTITY_Z_LEVEL,
    );

    let sprite_sheet_bundle = SpriteBundle {
        texture: environment_entity_assets.animation_image_handle.clone(),
        transform,
        ..Default::default()
    };

    let texture_atlas = TextureAtlas {
        layout: environment_texture_atlas_layout,
        index: ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_START_IDX,
    };

    let animation_component = AnimationComponent {
        start_idx: ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_START_IDX as u32,
        end_idx: ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_END_IDX,
        speed: ENVIRONMENT_ENTITY_ANIMATION_SPEED,
    };

    let environment_entity_kinetics = KineticEntityComponents {
        y_axis_displacement: 0.0,
        x_axis_displacement: 0.0,
        position: transform.translation,
        prev_position: transform.translation,
    };
    commands
        .spawn(EnvironmentEntityBundle {
            name: Name::new("Environmental_Entity"),
            kinetics: environment_entity_kinetics,
            sprite_sheet: sprite_sheet_bundle,
            texture_atlas,
            animation_component,
            animation_timer: AnimationTimer(Timer::from_seconds(
                animation_component.speed,
                TimerMode::Repeating,
            )),
        })
        .insert(EnvironmentEntityTag);
}
