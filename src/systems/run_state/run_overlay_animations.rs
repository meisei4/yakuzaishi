use bevy::prelude::{Commands, Query, Res, SpriteSheetBundle, Transform, Vec3};
use bevy_ecs_tilemap::tiles::TilePos;

use crate::components::overlay_animation::{AnimationType, OverlayAnimation, OverlayAnimationData};

pub fn spawn_overlay_animations(
    mut commands: Commands,
    player_query: Query<(&Transform, &TilePos)>,
    animation_data: Res<OverlayAnimationData>,
) {
    for (player_transform, player_tile_pos) in player_query.iter() {
        let animation_type = determine_animation_type(player_tile_pos); // Define logic to determine animation type

        let texture_handle = match animation_type {
            AnimationType::Wake => animation_data.wake_texture.clone(),
            AnimationType::Splash => animation_data.splash_texture.clone(),
        };

        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: Default::default(),
                transform: Transform {
                    translation: Vec3::new(
                        player_transform.translation.x,
                        player_transform.translation.y,
                        1.0,
                    ),
                    ..Default::default()
                },
                texture: Default::default(), //TODO: figure out how to get texture handle here with atlas aswell
                atlas: Default::default(),
                ..Default::default()
            })
            .insert(OverlayAnimation {
                tile_pos: *player_tile_pos,
                animation_type,
                z_index: 1.0,
                start_idx: 0,
                end_idx: 0,
                speed: 0.0,
            });
    }
}

fn determine_animation_type(tile_pos: &TilePos) -> AnimationType {
    // Logic to determine the type of animation based on the tile position or other conditions
    AnimationType::Splash // Example
}
