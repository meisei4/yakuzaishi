use std::path::PathBuf;

use amethyst::{
    core::transform::TransformBundle,
    Error,
    input::StringBindings,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        RenderingBundle,
        types::DefaultBackend,
    },
    utils::application_root_dir,
};
use amethyst::ui::UiBundle;

use yakuzaishi::{
    DISPLAY_CONFIG_FILENAME,
    resources::key_bindings_resource::KeyBindingsResource
    ,
    systems::{
        camera_tracking_system::CameraTrackingSystem,
        vehicle_controller_system::VehicleControllerSystem,
    }, VEHICLE_BINDINGS_CONFIG_FILENAME,
};
use yakuzaishi::enums::entity_type::EntityType;
use yakuzaishi::state::loading_state::LoadingState;
use yakuzaishi::systems::collision_system::CollisionSystem;

fn main() -> Result<(), Error> {
    //log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_path = app_root.join("assets");
    let display_config_path = assets_path.join(DISPLAY_CONFIG_FILENAME);

    let rendering_bundle = create_rendering_bundle(&display_config_path)?;

    //let key_bindings_resource = KeyBindingsResource::load(EntityType::Menu, MENU_BINDINGS_CONFIG_FILENAME)?;
    let key_bindings_resource =
        KeyBindingsResource::load(EntityType::Vehicle, VEHICLE_BINDINGS_CONFIG_FILENAME)?;

    let game_data = build_game_data(key_bindings_resource, rendering_bundle)?;

    //let mut game = Application::build(assets_path, MenuState::new())?.build(game_data)?;
    let loading_state = LoadingState::new(EntityType::Vehicle); // Initialize your loading state with necessary parameters

    let mut game = Application::new(assets_path, loading_state, game_data)?;
    game.run();

    Ok(())
}

//TODO Figure out cross platform build specific stuff (including yaml files)
//#[cfg(feature = "metal")]
//#[cfg(feature = "vulkan")]
fn create_rendering_bundle(
    display_config_path: &PathBuf,
) -> Result<RenderingBundle<DefaultBackend>, Error> {
    Ok(
        RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            .with_plugin(RenderFlat2D::default()),
        //.with_plugin(RenderToWindow::with_metal()))
        //.with_plugin(RenderToWindow::with_vulkan())
    )
}

//TODO write a common type to be implemented for spawning system, resource, components for vehicle and pedestrian etc inheritance
fn build_game_data(
    key_bindings_resource: KeyBindingsResource,
    rendering_bundle: RenderingBundle<DefaultBackend>,
) -> Result<GameDataBuilder<'static, 'static>, Error> {
    // Assuming you have a default input bundle for initial setup
    let default_input_bundle = key_bindings_resource
        //.get_input_bundle(&EntityType::Menu)
        .get_input_bundle(&EntityType::Vehicle)
        .unwrap();

    Ok(GameDataBuilder::default()
        .with_bundle(rendering_bundle)?
        .with_bundle(default_input_bundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        //TODO cant add these systems until Vehicle is chosen and main game state is started (after menu etc)
        .with(VehicleControllerSystem, "vehicle_controller_system", &[])
        .with(
            CameraTrackingSystem,
            "camera_tracking_system",
            &["vehicle_controller_system"],
        )
        .with(
            CollisionSystem,
            "collision_system",
            &["vehicle_controller_system"],
        ))
}
