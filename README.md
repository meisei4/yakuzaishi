# 薬剤師

## Overview

薬剤師 is a game project built with Rust, utilizing the Amethyst game engine (https://book.amethyst.rs/book/stable/)

- **General Purpose**: The project is intended to be a way to learn the Entity Component System (ECS) design pattern as well as learn general game design tooling.
  
- **Tooling**: `tiled`, is being used as the Tiled Map Editor (https://www.mapeditor.org/) and parser (https://docs.rs/tiled/latest/tiled/)

## Project Structure

```
src
 ├─ main.rs:
 │     - initializes resources, builds the world and runs the gameloop
 ├─ game_state.rs:
 │     - Contains the `Yakuzaishi` game state which initializes the game world and manages state transitions.
 ├─ game_map.rs:
 │     - Manages the GameMap resource, wrapping the tiled Map type functionality.
 ├─ vehicle_sprite_sheet.rs:
 │     - Handles the vehicle sprite sheet resource, ensuring that the sprites are loaded and ready for rendering.
 ├─ spawner.rs:
 │     - Includes the logic for spawning game entities like vehicles in the game world.
 └─ camera.rs:
      - Manages the game camera setup.

resources:
      - Holds game assets like sprites (.pngs, .rons), tilesets, tilesheets (.tms, .tmx), and other rust config files
tests:
      - TO-BE: Contains tests for resource loading and other critical game functionalities.
```
## Getting Started

To get started with 薬剤師:

1. Ensure you have Rust and Cargo installed (maybe use vscode or something
2. Clone the repository to your local machine.
3. Navigate to the cloned directory and run `cargo build` to compile the project.
4. Execute `cargo run` to launch the game.

## Contributing

薬剤師 is an open-source project, and we welcome contributions. Whether it's adding new features, fixing bugs, or improving documentation, your input is valued.

Before contributing, please read through the project documentation and adhere to the coding standards established in the project.

## License

薬剤師 is distributed under the MIT License, see LICENSE for more information.
