use std::collections::HashMap;

use bevy::asset::{Assets, AssetServer};
use bevy::math::Vec2;
use bevy::prelude::{Commands, Entity, Query, Res, ResMut, TextureAtlas};
use bevy::time::{Timer, TimerMode};

use crate::components::animation_timer::AnimationTimer;
use crate::components::overlay_animation::{AnimationType, OverlayAnimation, OverlayAnimationData};
use crate::components::overlay_animation::AnimationType::{Splash, Wake};

pub fn setup_overlay_animation_data(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let wake_texture_handle = asset_server.load("textures/wake_spritesheet.png");
    let splash_texture_handle = asset_server.load("textures/splash_spritesheet.png");

    let wake_texture_atlas =
        TextureAtlas::from_grid(wake_texture_handle, Vec2::new(16.0, 16.0), 4, 1);
    let splash_texture_atlas =
        TextureAtlas::from_grid(splash_texture_handle, Vec2::new(16.0, 16.0), 4, 1);

    let mut animations = HashMap::new();

    let wake_animation = OverlayAnimation {
        tile_pos: Default::default(),
        animation_type: Wake,
        start_idx: 0,
        end_idx: 3,
        speed: 0.5,
        z_index: 0.0,
    };

    let splash_animation = OverlayAnimation {
        tile_pos: Default::default(),
        animation_type: Splash,
        start_idx: 4,
        end_idx: 7,
        speed: 0.5,
        z_index: 0.0,
    };

    animations.insert(Wake, wake_animation);
    animations.insert(Splash, splash_animation);

    commands.insert_resource(OverlayAnimationData {
        animations,
        wake_texture: texture_atlases.add(wake_texture_atlas),
        splash_texture: texture_atlases.add(splash_texture_atlas),
    });
}

pub fn attach_overlay_animations(
    mut commands: Commands,
    query: Query<Entity>, // Query for relevant entities
    overlay_animation_data: Res<OverlayAnimationData>,
) {
    for entity in query.iter() {
        // Determine animation type to apply
        let animation_type = determine_animation_type(entity); // Implement logic to get animation ID

        if let Some(animation) = overlay_animation_data.animations.get(&animation_type) {
            commands.entity(entity).insert(OverlayAnimation {
                tile_pos: Default::default(),
                animation_type: Wake,
                z_index: 0.0,
                start_idx: animation.start_idx,
                end_idx: animation.end_idx,
                speed: animation.speed,
            });
            commands
                .entity(entity)
                .insert(AnimationTimer(Timer::from_seconds(
                    0.1,
                    TimerMode::Repeating,
                )));
        }
    }
}

// Dummy function to determine animation ID
fn determine_animation_type(entity: Entity) -> AnimationType {
    // Your logic to determine which animation to apply
    Wake // Example, returns wake animation ID
}
