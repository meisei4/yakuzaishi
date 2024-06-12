use bevy::asset::AssetApp;
use bevy::DefaultPlugins;
use bevy::prelude::{App, ImagePlugin, PluginGroup, Startup, Update, Window, WindowPlugin};
use bevy::window::WindowResolution;
use bevy_ecs_tilemap::TilemapPlugin;

use yakuzaishi::{NINTENDO_DS_SCREEN_HEIGHT, NINTENDO_DS_SCREEN_WIDTH};
use yakuzaishi::startup_systems::{initialize_camera, render_map, spawn_flying_entity};
use yakuzaishi::update_systems::{animation, camera_tracker::camera_tracking_system, flying_entity_controller::vehicle_controller_system, process_tiled_maps::TiledMap};
use yakuzaishi::update_systems::process_tiled_maps::process_tiled_maps;
use yakuzaishi::update_systems::tiled_loader::TiledLoader;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
            primary_window: Option::from(Window {
                resolution: WindowResolution::new(
                    NINTENDO_DS_SCREEN_WIDTH,
                    NINTENDO_DS_SCREEN_HEIGHT,
                ),
                resizable: false,
                title: "Yakuzaishi".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(TilemapPlugin)
        .init_asset::<TiledMap>()
        .register_asset_loader(TiledLoader)
        .add_systems(Startup, render_map::render_map)
        .add_systems(Startup, spawn_flying_entity::spawn_vehicle)
        .add_systems(Startup, animation::setup_map_animation_data)
        .add_systems(Startup, initialize_camera::init_camera)
        .add_systems(Startup, process_tiled_maps) // TODO: How is this an Update system?
        //.add_systems(Startup, animation::attach_animations_to_map)
        .add_systems(Update, camera_tracking_system)
        .add_systems(Update, vehicle_controller_system)
        .run();
}
