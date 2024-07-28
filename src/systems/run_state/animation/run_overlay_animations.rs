use bevy::prelude::{
    Commands, default, Query, Res, SpriteBundle, TextureAtlas, Timer, TimerMode, Transform, Vec3,
};
use bevy_ecs_tilemap::tiles::TilePos;

use crate::components::animation_timer::AnimationTimer;
use crate::components::overlay_animation::AnimationType::{Splash, Wake};
use crate::components::overlay_animation::OverlayAnimationData;

pub fn spawn_overlay_animations(
    mut commands: Commands,
    player_query: Query<(&Transform, &TilePos)>,
    overlay_animation_data: Res<OverlayAnimationData>,
) {
    for (player_transform, player_tile_pos) in player_query.iter() {
        //TODO: do something about the arguments to util.determine_animations_type thing
        let animation_type = Wake; // determine_animation_type(player_tile_pos);

        let (texture_handle, texture_layout_handle, animation) = match animation_type {
            Wake => (
                overlay_animation_data.wake_texture_handle.clone(),
                overlay_animation_data.wake_texture_layout.clone(),
                overlay_animation_data.wake_animation,
            ),
            Splash => (
                overlay_animation_data.splash_texture_handle.clone(),
                overlay_animation_data.splash_texture_layout.clone(),
                overlay_animation_data.splash_animation,
            ),
        };

        commands.spawn((
            SpriteBundle {
                texture: texture_handle,
                transform: Transform::from_xyz(
                    player_transform.translation.x,
                    player_transform.translation.y,
                    animation.z_index,
                )
                .with_scale(Vec3::splat(6.0)),
                ..default()
            },
            TextureAtlas {
                layout: texture_layout_handle,
                index: animation.start_idx as usize,
            },
            AnimationTimer(Timer::from_seconds(animation.speed, TimerMode::Repeating)),
            animation,
        ));
    }
}
