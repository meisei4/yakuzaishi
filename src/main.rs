use std::path::PathBuf;

use yakuzaishi::state::main_game_state::Yakuzaishi;
use yakuzaishi::systems::vehicle_spawner_system::VehicleSpawnerSystem;
use yakuzaishi::systems::vehicle_controller_system::VehicleControllerSystem;

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

//TODO figure out where to put these and actually load them (same with constants in main.rs)
const DISPLAY_CONFIG_FILENAME: &str = "display_config.ron";
const BINDINGS_CONFIG_FILENAME: &str = "key_bindings.ron";

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root: PathBuf = application_root_dir()?;
    let assets_path: PathBuf = app_root.join("assets");
    let display_config_path: PathBuf = assets_path.join(DISPLAY_CONFIG_FILENAME);
    let binding_path: PathBuf = assets_path.join(BINDINGS_CONFIG_FILENAME);

    info!("display config path: {:?}", display_config_path);

    info!("key bindings path: {:?}", binding_path);

    let input_bundle: InputBundle<StringBindings> =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    info!("Input bundle/key bindings loaded.");

    let game_data: GameDataBuilder<'_, '_> = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(
            VehicleControllerSystem,
            "vehicle_controller_system",
            &["input_system"],
        )
        // Register your VehicleSpawnerSystem here, ensure it's after TransformBundle to have transforms initialized
        .with(
            VehicleSpawnerSystem,
            "vehicle_spawner_system",
            &["transform_system"],
        ); // Add other systems as needed

    info!("Game data bundle created.");

    let mut game: CoreApplication<'_, GameData<'_, '_>> =
        Application::build(assets_path, Yakuzaishi::default())?.build(game_data)?;

    info!("Game application built.");

    info!("Starting game loop.");

    game.run();

    info!("Game loop ended.");

    Ok(())
}
