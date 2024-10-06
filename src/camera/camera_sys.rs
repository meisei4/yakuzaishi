use bevy::asset::Handle;
use bevy::math::Vec2;
use bevy::prelude::{
    Assets, Camera2dBundle, Commands, OrthographicProjection, ParamSet, Query, Res, Transform, With,
};
use bevy_render::camera::Camera;

use crate::{CAMERA_SCALE_MULTIPLIER, CAMERA_Z_LEVEL};
use crate::kinetic_components::PlayerEntityTag;
use crate::map::tiled_res::{TiledMap, TiledMapAssets};

pub fn init_camera(mut command_buffer: Commands) {
    command_buffer.spawn(Camera2dBundle {
        //TODO: the 0.0, 0.0, is ugly here since only the Z value here is needed, and the rest could be initialized in the tracking system?
        transform: Transform::from_xyz(0.0, 0.0, CAMERA_Z_LEVEL),
        projection: OrthographicProjection {
            scale: CAMERA_SCALE_MULTIPLIER,
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn track_camera(
    tiled_asset: Res<TiledMapAssets>,
    map_assets: Res<Assets<TiledMap>>,
    mut param_set: ParamSet<(
        Query<&Transform, With<PlayerEntityTag>>,
        Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    )>,
) {
    let mut player_position = Vec2::ZERO;

    if let Some(player_transform) = param_set.p0().iter().next() {
        player_position.x = player_transform.translation.x;
        player_position.y = player_transform.translation.y;
    }

    let map_handle: Handle<TiledMap> = tiled_asset.tiled_map.clone();
    if let Some(tiled_map) = map_assets.get(&map_handle) {
        // Map dimensions
        let map_width = (tiled_map.map.width * tiled_map.map.tile_width) as f32;
        let map_height = (tiled_map.map.height * tiled_map.map.tile_height) as f32;

        // Map boundaries
        let map_min_x = 0.0;
        let map_max_x = map_width;
        let map_min_y = 0.0;
        let map_max_y = map_height;

        for (mut camera_transform, orthographic_projection) in param_set.p1().iter_mut() {
            // Calculate the camera's half-width and half-height using the updated area
            let camera_width = orthographic_projection.area.width();
            let camera_height = orthographic_projection.area.height();
            let half_camera_width = camera_width / 2.0;
            let half_camera_height = camera_height / 2.0;

            // Adjust for half-tile offset
            let tile_size = 64.0; // Replace with your TILE_SIZE constant

            // Calculate clamping boundaries
            let min_x = map_min_x + half_camera_width - tile_size / 2.0;
            let max_x = map_max_x - half_camera_width - tile_size / 2.0;
            let min_y = map_min_y + half_camera_height - tile_size / 2.0;
            let max_y = map_max_y - half_camera_height - tile_size / 2.0;

            // Clamp the camera's position
            camera_transform.translation.x = player_position.x.clamp(min_x, max_x);
            camera_transform.translation.y = player_position.y.clamp(min_y, max_y);
        }
    }
}
