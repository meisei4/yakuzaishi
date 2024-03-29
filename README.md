# 薬剤師

## Update: everything below is bogus info i am now converting everything to bevy ;_;

## Overview

薬剤師 is a game project built with Rust, utilizing the Amethyst game engine (https://book.amethyst.rs/book/stable/)

- **General Purpose**: The project is intended to be a way to learn the Entity Component System (ECS) design pattern as
  well as learn general game design tooling.

- **Tooling**: `tiled`, is being used as the Tiled Map Editor (https://www.mapeditor.org/) and
  parser (https://docs.rs/tiled/latest/tiled/)

## example outline

```
src
 ├─ main.rs:
 │     - code entry point: initializes the gameworld and starts gameloop (RUST ROVER has RUN CONFIG HERE)
 ├─ command_fuffer
 │     └─ command_buffer.rs:
 │           - all the world mutability issues i was having gets solved here, so that i just pass all the commands to a single execute function at the start
 ├─ components
 │     └─ vehicle_components.rs:
 │           - Defines the Vehicle component for entities in the ECS (Entity-Component-System) architecture.
 ├─ resources
 │     └─ game_map_resource.rs:
 │           - Manages the game map resource, encapsulating the tiled Map type functionality (also conversion to tiles in amethyst)
 ├─ state
 │     └─ main_game_state.rs:
 │           - Contains the `on_start` function to init the game world and insert Resources (later will manage other state related functionaliy)
 ├─ systems
 │     └─ vehicle_controller_system.rs:
 │           - Handles the logic for Vehicle movement and control within the game world
assets
 ├─ map_data
 │     - Contains map-related data files (png, tmx, tsx) used for generating the game world.
 ├─ sprite_data
 │     - Stores sprite sheets and individual sprite data for entities in the game.
 ├─ display_config.ron:
 │     - Configuration file defining display settings such as resolution and full-screen mode.
 └─ key_bindings:
       - Configuration files specifying the key bindings for player input and controls.
```

## Getting Started

To get started with 薬剤師:

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install)
   and [Cargo](https://doc.rust-lang.org/cargo/commands/cargo-install.html) installed (
   recommend [Rust Rover](https://www.jetbrains.com/help/rust/installation-guide.html) for debugging)
2. Clone the repository to your local machine.
3. Navigate to the cloned directory and run `cargo build` to compile the project.
4. Execute `cargo run` to launch the game.

## Debug stuff

1. Use Rust Rover and just go to main.rs and run the main loop in debug mode (right-click the play button)
