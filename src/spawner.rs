pub mod spawner {
    use amethyst::{
        assets::Handle,
        core::{
            math::{Vector2, Vector3},
            Transform,
        },
        ecs::prelude::{World, WorldExt},
        prelude::*,
        renderer::{SpriteRender, SpriteSheet},
    };
    use log::info;

    use crate::{map::GameMap, vehicle::Vehicle};

    // Constants related to spawning
    pub const TILE_SIZE: f32 = 64.0;
    pub const VEHICLE_SIZE: f32 = 32.0;

    pub fn find_spawn_position_from_world_map(game_map: &GameMap) -> Vector2<f32> {
        for (y, row) in game_map.tiles.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                // Ignoring the tile itself
                if game_map.is_drivable(x, y) {
                    let spawn_position = Vector2::new(
                        x as f32 * game_map.tile_size.x,
                        y as f32 * game_map.tile_size.y,
                    );
                    info!(
                        "Found drivable tile at ({}, {}), spawn position: {:?}",
                        x, y, spawn_position
                    );
                    return spawn_position;
                }
            }
        }

        info!("No drivable tile found. Defaulting to spawn position (0.0, 0.0)");
        Vector2::new(0.0, 0.0)
    }

    pub fn spawn_vehicle(
        world: &mut World,
        sprite_sheet_handle: Handle<SpriteSheet>,
        spawn_position: Vector2<f32>,
    ) {
        let mut transform = Transform::default();
        transform.set_translation_xyz(spawn_position.x, spawn_position.y, 0.0);

        let scale = TILE_SIZE / VEHICLE_SIZE;
        transform.set_scale(Vector3::new(scale, scale, 1.0));

        world
            .create_entity()
            .with(SpriteRender {
                sprite_sheet: sprite_sheet_handle,
                sprite_number: 0,
            })
            .with(transform)
            .with(Vehicle::new())
            .build();

        info!("Vehicle spawned at position: {:?}", spawn_position);
    }
}
