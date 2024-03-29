use bevy::prelude::{AssetServer, Commands, Res, Resource};
use serde::Deserialize;

use crate::{
    enums::entity_type::EntityType, MAP_FILE_PATH, resources::{game_map_resource::GameMapResource, vehicle_resource::VehicleResource}, state::{game_map_renderer, vehicle_spawner}, TILESET_FILE_PATH,
    TILESET_TEXTURE_FILE_PATH,
    VEHICLE_SPRITE_SHEET_FILE_PATH,
    VEHICLE_TEXTURE_FILE_PATH,
};

use super::camera_initializer;

#[derive(Resource, Deserialize)]
pub struct Yakuzaishi {
    pub entity_type: EntityType,
}

impl Yakuzaishi {
    pub fn new(entity_type: EntityType) -> Self {
        Self {
            entity_type,
        }
    }

    pub fn init_game_state(&mut self, command_buffer: &mut Commands, asset_server: &Res<AssetServer>) {
        let game_map = GameMapResource::load(asset_server,
                                             MAP_FILE_PATH,
                                             TILESET_FILE_PATH,
                                             TILESET_TEXTURE_FILE_PATH);
        game_map_renderer::render_map(&game_map, command_buffer);

        match self.entity_type {
            EntityType::Vehicle => {
                let vehicle_resource = VehicleResource::load(asset_server,
                                                             VEHICLE_TEXTURE_FILE_PATH,
                                                             VEHICLE_SPRITE_SHEET_FILE_PATH);
                vehicle_spawner::spawn_vehicle(&vehicle_resource, &game_map, command_buffer);
                command_buffer.insert_resource(vehicle_resource); //TODO: this is getting insane, fix it soon
            }
            _ => {}
        }
        camera_initializer::init_camera(command_buffer);
        command_buffer.insert_resource(game_map);
    }
}

impl Default for Yakuzaishi {
    fn default() -> Self {
        Self::new(EntityType::Vehicle)
    }
}




