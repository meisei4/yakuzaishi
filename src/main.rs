use bevy::prelude::{App, ClearColor, Color, PluginGroup, Startup, Update, Window, WindowPlugin};
use bevy::window::WindowResolution;
use bevy::DefaultPlugins;
use bevy_ecs_tilemap::TilemapPlugin;

use yakuzaishi::systems::{
    camera_tracker::camera_tracking_system, vehicle_controller::vehicle_controller_system,
};
use yakuzaishi::helpers_hack;
use yakuzaishi::startup::{initialize_camera, render_map, spawn_vehicle};
use yakuzaishi::{NINTENDO_DS_SCREEN_HEIGHT, NINTENDO_DS_SCREEN_WIDTH};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Option::from(Window {
                resolution: WindowResolution::new(NINTENDO_DS_SCREEN_WIDTH, NINTENDO_DS_SCREEN_HEIGHT),
                resizable: false,
                title: "Yakuzaishi".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(TilemapPlugin)
        .add_plugins(helpers_hack::tiled_hack::TiledMapPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, render_map::render_map)
        .add_systems(Startup, spawn_vehicle::spawn_vehicle)
        .add_systems(Startup, initialize_camera::init_camera)
        .add_systems(Update, camera_tracking_system)
        .add_systems(Update, vehicle_controller_system)
        .run();
}
