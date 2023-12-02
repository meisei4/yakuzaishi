use amethyst::{
    assets::Handle, core::timing::Time, ecs::prelude::WorldExt, prelude::*, renderer::SpriteSheet,
    winit::Event,
};

use log::info;

use crate::camera::camera;
use crate::map::GameMap;
use crate::spawner::spawner;
use crate::vehicle::Vehicle;
use crate::{asset_loader::asset_loader, vehicle_controller::VehicleController};

pub struct Yakuzaishi {
    vehicle_sprite_sheet: Option<Handle<SpriteSheet>>,
}

impl Default for Yakuzaishi {
    fn default() -> Self {
        Yakuzaishi {
            vehicle_sprite_sheet: None, // Set the default value as None since on_start loads it later?
        }
    }
}

impl SimpleState for Yakuzaishi {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Initialize the camera
        camera::init_camera(world);
        info!("Camera initialized.");

        // Load the sprite sheet for the vehicle
        self.vehicle_sprite_sheet = Some(asset_loader::load_vehicle_sprite_sheet(world));
        info!("Vehicle sprite sheet loaded.");

        // Load the world map
        asset_loader::load_world_map(world);
        info!("World map loaded.");

        // Temporarily create a scope to limit the lifetime of the immutable borrow
        let spawn_position = {
            let world_map = world.read_resource::<GameMap>();
            spawner::find_spawn_position_from_world_map(&world_map)
            // `world_map` borrow ends here
        };
        info!("Vehicle spawn position found at: {:?}", spawn_position);

        // Spawn the vehicle at the spawn position
        // This is where we need `world` to be mutably borrowed
        spawner::spawn_vehicle(
            world,
            self.vehicle_sprite_sheet.clone().unwrap(),
            spawn_position,
        );
        info!("Vehicle spawned.");
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        state_event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(Event::WindowEvent { event, .. }) = state_event {
            let mut vehicles = data.world.write_storage::<Vehicle>();
            let time = data.world.read_resource::<Time>();
            VehicleController::handle_window_event(event, &mut vehicles, &(*time))
        } else {
            Trans::None
        }
    }
}
