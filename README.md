# 薬剤師

## Overview

薬剤師 is a game project built with Rust, utilizing the Amethyst game engine (https://book.amethyst.rs/book/stable/)

- **General Purpose**: The project is intended to be a way to learn the Entity Component System (ECS) design pattern as well as learn general game design tooling.
  
- **Tooling**: `tiled`, is being used as the Tiled Map Editor (https://www.mapeditor.org/) and parser (https://docs.rs/tiled/latest/tiled/)

## Project Structure

```
src
 ├─ main.rs:
 │     - code entry point: initializes the gameworld and starts gameloop
 ├─ lib.rs:
 │     - library with shared logic project and re-exported items for the binary
 ├─ components
 │     ├─ mod.rs:
 │     │     - the module declaration file for components (provides accessibility from other modules)
 │     └─ vehicle_component.rs:
 │           - Defines the Vehicle component for entities in the ECS (Entity-Component-System) architecture.
 ├─ resources
 │     ├─ mod.rs:
 │     ├─ game_map_resource.rs:
 │     │     - Manages the GameMap resource, encapsulating the tiled Map type functionality
 │     └─ vehicle_sprite_sheet.rs:
 │           - Manages the VehicleSpriteSheet resource, loading and preparing sprites for rendering
 ├─ state
 │     ├─ mod.rs:
 │     └─ main_game_state.rs:
 │           - Contains the `on_start` function to init the game world and insert Resources (later will manage other state related functionaliy)
 ├─ systems
 │     ├─ mod.rs:
 │     ├─ vehicle_controller_system.rs:
 │     │     - Handles the logic for Vehicle movement and control within the game world
 │     └─ vehicle_spawner_system.rs:
 │           - Includes the logic for dynamically spawning Vehicle entities in the game
 └─ camera.rs:
       - Manages the game camera, handling the viewport and projection within the game world

assets
 ├─ map_data
 │     - Contains map-related data files (png, tmx, tsx) used for generating the game world.
 ├─ sprite_data
 │     - Stores sprite sheets and individual sprite data for entities in the game.
 ├─ display_config.ron:
 │     - Configuration file defining display settings such as resolution and full-screen mode.
 └─ key_bindings.ron:
       - Configuration file specifying the key bindings for player input and controls.
```
## Getting Started

To get started with 薬剤師:

1. Ensure you have Rust and Cargo installed (maybe use vscode or something)
2. Clone the repository to your local machine.
3. Navigate to the cloned directory and run `cargo build` to compile the project.
4. Execute `cargo run` to launch the game.

## Contributing

薬剤師 is an open-source project, and we welcome contributions. Whether it's adding new features, fixing bugs, or improving documentation, your input is valued.

Before contributing, please read through the project documentation and adhere to the coding standards established in the project.

## License

薬剤師 is distributed under the MIT License, see LICENSE for more information.
