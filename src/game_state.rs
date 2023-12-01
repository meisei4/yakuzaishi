use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::{timing::Time, transform::Transform},
    ecs::{Join, Read, ReadStorage, WriteStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    input::{InputHandler, StringBindings},
};

pub struct Yakuzaishi {
    pub vehicle_sprite_sheet: Option<Handle<SpriteSheet>>, // Store the sprite sheet handle
}

impl SimpleState for Yakuzaishi {
    
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Initialize the camera
        init_camera(world);

        // Load the sprite sheet for the vehicle
        self.vehicle_sprite_sheet = Some(load_vehicle_sprite_sheet(world));

        // Create the vehicle entity with the sprite and necessary components
        spawn_vehicle(world, self.vehicle_sprite_sheet.clone().unwrap());


        let tile_size = Vector2::new(64.0, 64.0); // Example tile size, adjust as needed
        let map_file_path = "resources/maps/map1.txt"; // Path to your map layout file
        
        // Load the map and insert it into the world as a resource
        match world::WorldMap::load_from_file(map_file_path, tile_size) {
            Ok(world_map) => {
                world.insert(world_map);
            },
            Err(e) => {
                eprintln!("Error loading map: {}", e);
                std::process::exit(1);
            }
        }
        let vehicle = Vehicle {
            speed: 0.0,
            max_speed: 100.0,
            acceleration: 1.0,
            deceleration: 1.0,
            position: Vector3::new(0.0, 0.0, 0.0), // Starting position, adjust as needed
            direction: Vector2::new(0.0, 1.0), // Starting direction, adjust as needed
            rotation_speed: 0.1,
            pill_stock: 100,
            gas: 100,
        };

        // Add the vehicle to the world
        data.world
            .create_entity()
            .with(vehicle)
            // You may also need to add other components like Transform, Renderable, etc.
            .build();
    }

    fn init_camera(world: &mut World) {
        let mut transform = Transform::default();
        transform.set_translation_xyz(250.0, 250.0, 1.0); // Center the camera (adjust as needed)
    
        world
            .create_entity()
            .with(Camera::standard_2d(500.0, 500.0)) // Set your desired camera dimensions
            .with(transform)
            .build();
    }
    
    // Loads the sprite sheet for the vehicle
    fn load_vehicle_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    
        let texture_handle = loader.load(
            "resources/sprites/car_sprite.png", // Replace with the path to your new PNG file
            ImageFormat::default(),
            (),
            &texture_storage,
        );
    
        loader.load(
            "resources/sprite_sheet.ron", // Replace with the path to your updated .ron file
            SpriteSheetFormat(texture_handle),
            (),
            &sprite_sheet_store,
        )
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key_code) = input.virtual_keycode {
                        match key_code {
                            VirtualKeyCode::Up => {
                                // Call vehicle.accelerate()
                            }
                            VirtualKeyCode::Down => {
                                // Call vehicle.decelerate()
                            }
                            VirtualKeyCode::Left => {
                                // Call vehicle.turn_left()
                            }
                            VirtualKeyCode::Right => {
                                // Call vehicle.turn_right()
                            }
                            VirtualKeyCode::Escape => {
                                return Trans::Quit;
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let delta_time = data.world.read_resource::<Time>().delta_seconds();
        for (mut vehicle,) in (&mut vehicles,).join() {
            vehicle.update_position(delta_seconds);
        }
        
        Trans::None
    }
}
