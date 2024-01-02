use std::{fs, path::PathBuf};

use yakuzaishi::{
    state::{main_game_state::Yakuzaishi, menu_state::MenuState},
    systems::{
        camera_tracking_system::CameraTrackingSystem,
        vehicle_controller_system::VehicleControllerSystem,
    },
    DISPLAY_CONFIG_FILENAME, MENU_BINDINGS_CONFIG_FILENAME, VEHICLE_BINDINGS_CONFIG_FILENAME,
};

// Windows uncomment:
//use amethyst::renderer::rendy::vulkan::Backend;

// MacOS uncomment:
use amethyst::{error, renderer::rendy::metal::Backend, ui::UiBundle};

use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    Error,
};
use log::info;
use yakuzaishi::systems::collision_system::CollisionSystem;

fn main() -> Result<(), Error> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_path = app_root.join("assets");
    let display_config_path = assets_path.join(DISPLAY_CONFIG_FILENAME);

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(assets_path.join(MENU_BINDINGS_CONFIG_FILENAME))?;
    info!("Menu input bindings loaded successfully.");

    let rendering_bundle = create_rendering_bundle(&display_config_path)?;
    info!("Rendering bundle created successfully.");

    let game_data = build_game_data(input_bundle, rendering_bundle)?;
    info!("Game data bundle created successfully.");

    let mut game = Application::build(assets_path, MenuState::new())?.build(game_data)?;
    info!("Game application built. Starting game loop.");

    game.run();

    info!("Game loop ended successfully.");
    Ok(())
}

//TODO Figure out cross platform build specific stuff (including yaml files)
//#[cfg(feature = "metal")]
//#[cfg(feature = "vulkan")]
fn create_rendering_bundle(
    display_config_path: &PathBuf,
) -> Result<RenderingBundle<DefaultBackend>, amethyst::Error> {
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
    input_bundle: InputBundle<StringBindings>,
    rendering_bundle: RenderingBundle<DefaultBackend>,
) -> Result<GameDataBuilder<'static, 'static>, Error> {
    // ORDER MATTERS BIG TIME HERE
    Ok(GameDataBuilder::default()
        .with_bundle(rendering_bundle)?
        .with_bundle(TransformBundle::new())?
        // .with_bundle(UiBundle::<StringBindings>::new())? TODO: I am r-worded, no idea what is happening with the menu input
        .with_bundle(input_bundle)?
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
