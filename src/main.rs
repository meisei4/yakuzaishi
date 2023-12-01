mod vehicle;
mod game_state;
mod world;
mod input_system;

use amethyst::{
    prelude::*,
    utils::application_root_dir,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    core::transform::TransformBundle,
    ecs::prelude::*,
    input::{InputBundle, StringBindings},
};

use game_state::Yakuzaishi;
use input_system::InputSystem; // Import the InputSystem

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron"); // Make sure this path is correct

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

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
        .with_bundle(input_bundle)? // Add the input bundle
        .with(InputSystem, "input_system", &["input_system"]); // Register the InputSystem

    let mut game = Application::new(app_root, Yakuzaishi::default(), game_data)?;

    game.run();

    Ok(())
}