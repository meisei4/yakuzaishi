use bevy::asset::Handle;
use bevy::math::Vec2;
use bevy::prelude::{
    Assets, Camera, Camera2dBundle, Commands, OrthographicProjection, ParamSet, Query, Res,
    Transform, With,
};

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
        Query<&mut Transform, With<Camera>>,
    )>,
) {
    let mut temp_translation = Vec2 { x: 0.0, y: 0.0 };

    if let Some(entity_transform) = param_set.p0().iter().next() {
        temp_translation.x = entity_transform.translation.x;
        temp_translation.y = entity_transform.translation.y;
    }

    let map_handle: Handle<TiledMap> = tiled_asset.tiled_map.clone();
    if let Some(tiled_map) = map_assets.get(&map_handle) {
        let map_width = (tiled_map.map.width * tiled_map.map.tile_width) as f32;
        let map_height = (tiled_map.map.height * tiled_map.map.tile_height) as f32;

        for mut camera_transform in param_set.p1().iter_mut() {
            let half_camera_width = camera_transform.scale.x / 2.0;
            let half_camera_height = camera_transform.scale.y / 2.0;

            // Clamp camera position within map boundaries
            camera_transform.translation.x = temp_translation.x.clamp(
                -map_width / 2.0 + half_camera_width,
                map_width / 2.0 - half_camera_width,
            );

            camera_transform.translation.y = temp_translation.y.clamp(
                -map_height / 2.0 + half_camera_height,
                map_height / 2.0 - half_camera_height,
            );
        }
    }
}
