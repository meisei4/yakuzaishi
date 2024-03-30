use bevy::prelude::{App, AssetServer, ClearColor, Color, Commands, Res, ResMut, Startup, Update, Window, WindowPlugin};

use yakuzaishi::{startup::startup_system::Yakuzaishi, systems::{camera_tracking_system::camera_tracking_system, vehicle_controller_system::vehicle_controller_system}};

fn main() {
    //log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    App::new()
        .add_plugins(WindowPlugin {
            primary_window: Some(Window {
                resolution: (600.0, 600.0).into(),
                title: "Yakuzaishi".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Yakuzaishi::default())
        .add_systems(Startup, (init_game_state_system))
        .add_systems(Update, (camera_tracking_system))
        .add_systems(Update, (vehicle_controller_system))
        .run();
}

fn init_game_state_system(
    command_buffer: &mut Commands,
    asset_server: Res<AssetServer>,
    mut yakuzaishi: ResMut<Yakuzaishi>,
) {
    //TODO: Initialize the asset texture_atlas_layouts


    yakuzaishi.init_game_state(command_buffer, asset_server, texture_atlas_layouts);
}