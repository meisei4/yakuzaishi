use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::prelude::{Query, Res, Time, Transform};

use crate::components::animation_components::AnimationTimer;
use crate::components::behavior_patterns::{FloatInCircle, ZigZag};
use crate::resources::animation_resources::EnvironmentalEntityAnimationResource;
use crate::resources::tiled_resources::TiledMap;

pub fn animate_env_entity_animations(
    time: Res<Time>,
    overlay_animation_data: Res<EnvironmentalEntityAnimationResource>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (mut timer, mut overlay_atlas) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            //TODO: Is there some sort of implied Copy and or Clone happening here? I dont know why when
            // AnimationComponent doesnt derive copy or clone this next line doesnt work
            let animation = overlay_animation_data.animation;

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

pub fn environmental_entities_behavior_pattern(
    time: Res<Time>,
    tiled_map_assets: Res<Assets<TiledMap>>,
    map_query: Query<&Handle<TiledMap>>,
    mut query: Query<(
        &mut Transform,
        Option<&mut FloatInCircle>,
        Option<&mut ZigZag>,
    )>,
) {
    if let Some(map_handle) = map_query.iter().next() {
        if let Some(tiled_map) = tiled_map_assets.get(map_handle) {
            let map_width = (tiled_map.map.width * tiled_map.map.tile_width) as f32;
            let map_height = (tiled_map.map.height * tiled_map.map.tile_height) as f32;

            for (mut transform, float_circle, zigzag) in query.iter_mut() {
                if let Some(mut fc) = float_circle {
                    fc.angle += fc.speed * time.delta_seconds();
                    transform.translation.x += fc.radius * (fc.angle * PI / 180.0).cos();
                    transform.translation.y += fc.radius * (fc.angle * PI / 180.0).sin();
                } else if let Some(mut zz) = zigzag {
                    transform.translation.x += zz.direction.x * zz.speed * time.delta_seconds();
                    transform.translation.y += zz.direction.y * zz.speed * time.delta_seconds();

                    // Change direction when hitting screen edge (assuming map bounds)
                    if transform.translation.x < 0.0 || transform.translation.x > map_width {
                        zz.direction.x *= -1.0;
                    }
                    if transform.translation.y < 0.0 || transform.translation.y > map_height {
                        zz.direction.y *= -1.0;
                    }
                }
            }
        }
    }
}
