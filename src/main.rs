mod asset_loader;
mod camera;
mod game_state;
mod map;
mod spawner;
mod vehicle;
mod vehicle_controller;

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
};
use log::info;

use game_state::Yakuzaishi;

// Constants for resource file paths
const DISPLAY_CONFIG: &str = "resources/display.ron";
const BINDINGS_CONFIG: &str = "resources/bindings.ron";

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join(DISPLAY_CONFIG);
    let binding_path = app_root.join(BINDINGS_CONFIG);

    info!("Display config path: {:?}", display_config_path);
    info!("Binding path: {:?}", binding_path);

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;
    info!("Input bundle loaded.");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?;
    info!("Game data bundle created.");

    let resources_dir = app_root.join("resources");
    let mut game = Application::build(resources_dir, Yakuzaishi::default())?.build(game_data)?;
    info!("Game application built.");

    info!("Starting game loop.");
    game.run();
    info!("Game loop ended.");

    Ok(())
}
