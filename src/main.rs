use std::path::PathBuf;

use yakuzaishi::{
    state::main_game_state::Yakuzaishi,
    systems::{
        vehicle_spawner_system::VehicleSpawnerSystem, 
        vehicle_controller_system::VehicleControllerSystem, 
        map_rendering_system::MapRenderingSystem
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
};
use log::info;

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
        //REGISTER SYSTEMS
        .with(
            MapRenderingSystem,
            "map_rendering_system",
            &["transform_system"],
        )
        .with(
            VehicleControllerSystem,
            "vehicle_controller_system",
            &["input_system"],
        )
        .with(
            VehicleSpawnerSystem::new(),
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
