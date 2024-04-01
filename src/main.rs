use bevy::DefaultPlugins;
use bevy::prelude::{App, ClearColor, Color, PluginGroup, Startup, Update, Window, WindowPlugin};
use bevy::window::WindowResolution;
use bevy_ecs_tilemap::TilemapPlugin;

use yakuzaishi::helpers;
use yakuzaishi::startup::{camera_initializer, tiled_map_render_hack, vehicle_spawner};
use yakuzaishi::systems::{camera_tracking_system::camera_tracking_system, vehicle_controller_system::vehicle_controller_system};

fn main() {
    //log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Option::from(Window {
                resolution: WindowResolution::new(500.0, 500.0),
                title: "Yakuzaishi".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(TilemapPlugin)
        .add_plugins(helpers::tiled_hack::TiledMapPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, tiled_map_render_hack::render_map)
        .add_systems(Startup, vehicle_spawner::spawn_vehicle)
        .add_systems(Startup, camera_initializer::init_camera)
        .add_systems(Update, camera_tracking_system)
        .add_systems(Update, vehicle_controller_system)
        .run();
}


