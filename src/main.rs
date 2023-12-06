use std::path::PathBuf;

use yakuzaishi::{
    state::main_game_state::Yakuzaishi,
    systems::{
        vehicle_spawner_system::VehicleSpawnerSystem,
        vehicle_controller_system::VehicleControllerSystem,
        map_rendering_system::MapRenderingSystem, 
        camera_tracking_system::CameraTrackingSystem,
    },
    DISPLAY_CONFIG_FILENAME, 
    BINDINGS_CONFIG_FILENAME,
};

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

fn main() -> Result<(), Error> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_path = app_root.join("assets");
    let display_config_path = assets_path.join(DISPLAY_CONFIG_FILENAME);
    let binding_path = assets_path.join(BINDINGS_CONFIG_FILENAME);

    info!("Display config path: {:?}", display_config_path);
    info!("Key bindings path: {:?}", binding_path);

    let input_bundle = create_input_bundle(&binding_path)?;
    let rendering_bundle = create_rendering_bundle(&display_config_path)?;
    let game_data = build_game_data(input_bundle, rendering_bundle)?;

    info!("Game data bundle created.");

    let mut game = Application::build(assets_path, Yakuzaishi::default())?
        .build(game_data)?;

    info!("Game application built.");
    info!("Starting game loop.");
    game.run();
    info!("Game loop ended.");

    Ok(())
}

fn create_input_bundle(binding_path: &PathBuf) -> Result<InputBundle<StringBindings>, Error> {
    InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path).map_err(Error::from)
}

//TODO Figure out cross platform build specific stuff (including yaml files)
//#[cfg(feature = "metal")]
//#[cfg(feature = "vulkan")] 
fn create_rendering_bundle(
    display_config_path: &PathBuf,
) -> Result<RenderingBundle<DefaultBackend>, amethyst::Error> {
    Ok(RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)?
                .with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default())
        //.with_plugin(RenderToWindow::with_metal()))
        //.with_plugin(RenderToWindow::with_vulkan())
    )
}

fn build_game_data(
    input_bundle: InputBundle<StringBindings>,
    rendering_bundle: RenderingBundle<DefaultBackend>,
) -> Result<GameDataBuilder<'static, 'static>, Error> {
    // ORDER MATTERS BIG TIME HERE
    Ok(
        GameDataBuilder::default()
            .with_bundle(rendering_bundle)?
            .with_bundle(TransformBundle::new())?
            .with_bundle(input_bundle)?
            .with(MapRenderingSystem, "map_rendering_system", &["transform_system"])
            .with(VehicleSpawnerSystem::new(), "vehicle_spawner_system", &["transform_system"])
            .with(VehicleControllerSystem, "vehicle_controller_system", &["input_system"])
            .with(CameraTrackingSystem, "camera_tracking_system", &["vehicle_controller_system"]),
    )
}
