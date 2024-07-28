use bevy::asset::{Assets, AssetServer};
use bevy::math::Vec2;
use bevy::prelude::{Commands, Entity, Query, Res, ResMut, TextureAtlasLayout};
use bevy::time::{Timer, TimerMode};

use crate::{SPLASH_ANIMATION, WAKE_ANIMATION};
use crate::components::animation_timer::AnimationTimer;
use crate::components::overlay_animation::{OverlayAnimation, OverlayAnimationData};
use crate::components::overlay_animation::AnimationType::{Splash, Wake};
use crate::systems::util::determine_animation_type;

pub fn setup_overlay_animation_data(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let wake_texture_handle = asset_server.load(WAKE_ANIMATION);
    let splash_texture_handle = asset_server.load(SPLASH_ANIMATION);

    let wake_texture_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::splat(16.0),
        4,
        1,
        None,
        None,
    ));
    let splash_texture_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::splat(16.0),
        4,
        1,
        None,
        None,
    ));

    commands.insert_resource(OverlayAnimationData {
        wake_animation: OverlayAnimation {
            start_idx: 0,
            end_idx: 3,
            speed: 0.5,
            z_index: 0.0,
        },
        splash_animation: OverlayAnimation {
            start_idx: 4,
            end_idx: 7,
            speed: 0.5,
            z_index: 0.0,
        },
        wake_texture_handle,
        splash_texture_handle,
        wake_texture_layout,
        splash_texture_layout,
    });
}

pub fn attach_overlay_animations(
    mut commands: Commands,
    query: Query<Entity>,
    overlay_animation_data: Res<OverlayAnimationData>,
) {
    for entity in query.iter() {
        let animation = match determine_animation_type(entity) {
            Wake => overlay_animation_data.wake_animation,
            Splash => overlay_animation_data.splash_animation,
        };

        commands.entity(entity).insert(animation);
        commands
            .entity(entity)
            .insert(AnimationTimer(Timer::from_seconds(
                animation.speed,
                TimerMode::Repeating,
            )));
    }
}
