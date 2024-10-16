use bevy::{
    asset::Handle,
    log::info,
    math::{UVec2, Vec2},
    prelude::{
        Assets, Camera2dBundle, Commands, OrthographicProjection, ParamSet, Query, Res, Transform,
        With,
    },
    utils::default,
};
use bevy_render::camera::{Camera, Viewport};

use crate::{
    camera::camera_components::BottomCameraTag,
    CAMERA_SCALE_MULTIPLIER,
    CAMERA_Z_LEVEL,
    environment::moon::MoonTag,
    kinetic_components::PlayerEntityTag, map::tiled_res::{TiledMapAssets, TiledMapSource}, NINTENDO_DS_SCREEN_HEIGHT, NINTENDO_DS_SCREEN_WIDTH,
};

pub fn top_camera(mut commands: Commands, mut query: Query<&Transform, With<MoonTag>>) {
    for moon_transform in query.iter_mut() {
        let camera = Camera2dBundle {
            transform: Transform::from_xyz(
                moon_transform.translation.x,
                moon_transform.translation.y,
                CAMERA_Z_LEVEL,
            ),
            camera: Camera {
                order: 2,
                viewport: Some(Viewport {
                    physical_position: UVec2::ZERO, // top left
                    physical_size: UVec2::new(
                        (NINTENDO_DS_SCREEN_WIDTH * 2.0) as u32,
                        NINTENDO_DS_SCREEN_HEIGHT as u32,
                    ),
                    ..default()
                }),
                ..default()
            },
            projection: OrthographicProjection {
                scale: CAMERA_SCALE_MULTIPLIER * 1.33, // TODO: figure out a better way to scale this par tof the world
                ..default()
            },
            ..default()
        };
        info!(
            "Moon Position: {:?}, Camera Position: {:?}",
            moon_transform.translation, camera.transform.translation
        );
        commands.spawn(camera);
    }
}

pub fn bottom_camera(mut commands: Commands) {
    let normal_transform = Transform::from_xyz(0.0, 0.0, CAMERA_Z_LEVEL);
    init_camera(
        &mut commands,
        UVec2::new(0, NINTENDO_DS_SCREEN_HEIGHT as u32),
        UVec2::new(
            (NINTENDO_DS_SCREEN_WIDTH * 2.0) as u32,
            NINTENDO_DS_SCREEN_HEIGHT as u32,
        ),
        1,
        normal_transform,
    )
}

pub fn init_camera(
    commands: &mut Commands,
    viewport_position: UVec2,
    viewport_size: UVec2,
    camera_order: isize,
    transform: Transform,
) {
    commands
        .spawn(Camera2dBundle {
            transform,
            camera: Camera {
                order: camera_order,
                viewport: Some(Viewport {
                    physical_position: viewport_position,
                    physical_size: viewport_size,
                    ..default()
                }),
                ..default()
            },
            projection: OrthographicProjection {
                scale: CAMERA_SCALE_MULTIPLIER,
                ..default()
            },
            ..default()
        })
        .insert(BottomCameraTag);
}

pub fn track_camera(
    tiled_asset: Res<TiledMapAssets>,
    map_assets: Res<Assets<TiledMapSource>>,
    mut param_set: ParamSet<(
        Query<&Transform, With<PlayerEntityTag>>,
        Query<(&mut Transform, &OrthographicProjection), With<BottomCameraTag>>,
    )>,
) {
    let mut player_position = Vec2::ZERO;

    if let Some(player_transform) = param_set.p0().iter().next() {
        player_position.x = player_transform.translation.x;
        player_position.y = player_transform.translation.y;
    }

    let map_handle: Handle<TiledMapSource> = tiled_asset.tiled_map.clone();
    if let Some(tiled_map) = map_assets.get(&map_handle) {
        // Map dimensions
        let map_width = (tiled_map.rs_tiled_map.width * tiled_map.rs_tiled_map.tile_width) as f32;
        let map_height =
            (tiled_map.rs_tiled_map.height * tiled_map.rs_tiled_map.tile_height) as f32;

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
