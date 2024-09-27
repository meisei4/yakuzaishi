use bevy::asset::Assets;
use bevy::core::Name;
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Res, ResMut, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout, Timer, TimerMode,
    Transform,
};

use crate::{
    ENVIRONMENT_ENTITY_ANIMATION_SPEED, ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_COLUMN_LENGTH,
    ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_END_IDX, ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_ROW_LENGTH,
    ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_START_IDX, ENVIRONMENT_ENTITY_SPAWN_X,
    ENVIRONMENT_ENTITY_SPAWN_Y, ENVIRONMENT_ENTITY_Z_LEVEL, TILE_SIZE,
};
use crate::anime::anime_components::{AnimationComponent, AnimationTimer};
use crate::anime::anime_res::EnvironmentEntityAnimationAssets;
use crate::bundles::EnvironmentEntityBundle;
use crate::kinetic_components::{EnvironmentEntityTag, KineticEntityComponents};

pub fn spawn_environment_entity(
    mut commands: Commands,
    environment_entity_assets: Res<EnvironmentEntityAnimationAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let environment_texture_atlas_layout =
        texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            Vec2::splat(TILE_SIZE),
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

    let sprite_sheet_bundle = SpriteSheetBundle {
        texture: environment_entity_assets.animation_image_handle.clone(),
        atlas: TextureAtlas {
            layout: environment_texture_atlas_layout,
            index: ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_START_IDX as usize,
        },
        transform,
        ..Default::default()
    };

    let animation_component = AnimationComponent {
        start_idx: ENVIRONMENT_ENTITY_ANIMATION_TEXTURE_START_IDX,
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
            animation_component,
            animation_timer: AnimationTimer(Timer::from_seconds(
                animation_component.speed,
                TimerMode::Repeating,
            )),
        })
        .insert(EnvironmentEntityTag);
}
