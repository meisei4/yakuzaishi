use bevy::prelude::*;

use yakuzaishi::{
    enums::entity_type::EntityType, resources::key_bindings_resource::KeyBindingsResource,
    VEHICLE_BINDINGS_CONFIG_FILENAME,
};
use yakuzaishi::state::loading_state::load_assets;

fn main() {
    //log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let key_bindings_resource =
        KeyBindingsResource::load(EntityType::Vehicle, VEHICLE_BINDINGS_CONFIG_FILENAME)?;

    App::new()
        .add_plugins(WindowPlugin {
            primary_window: Some(Window {
                resolution: (600.0, 600.0).into(),
                title: "Yakuzaishi".to_string(),
                ..default()
            }),
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (load_assets))// TODO just pass the functions in the state bullshit from amethyst
        //TODO 1: ^^ focus onn establishing the resource loading/LoadingState process

        .add_systems(Update, (vehicle_controller_system))
        .add_systems(Update, (camera_tracking_system))
        .add_systems(Update, (collision_system))
        .run();
}