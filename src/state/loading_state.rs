use amethyst::{
    assets::ProgressCounter,
    prelude::*

    ,
};
use amethyst::assets::{AssetStorage, Handle};
use amethyst::renderer::SpriteSheet;

use crate::{MAP_FILE_PATH, PEDESTRIAN_BINDINGS_CONFIG_FILENAME, TILESET_FILE_PATH, TILESET_TEXTURE_FILE_PATH, VEHICLE_BINDINGS_CONFIG_FILENAME, VEHICLE_SPRITE_SHEET_FILE_PATH, VEHICLE_TEXTURE_FILE_PATH};
use crate::enums::entity_type::EntityType;
use crate::resources::game_map_resource::GameMapResource;
use crate::resources::key_bindings_resource::KeyBindingsResource;
use crate::resources::system_active_flag::SystemActive;
use crate::resources::vehicle_resource::VehicleResource;
use crate::state::main_game_state::Yakuzaishi;

pub struct LoadingState {
    progress_counter: ProgressCounter,
    entity_type: EntityType,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl LoadingState {
    pub fn new(entity_type: EntityType) -> Self {
        Self {
            progress_counter: ProgressCounter::new(),
            entity_type,
            sprite_sheet_handle: None, // Initialize as None because it will be loaded later
        }
    }
    fn load_resources(&mut self, world: &mut World) {
        let game_map_resource = GameMapResource::load(
            world,
            MAP_FILE_PATH,
            TILESET_FILE_PATH,
            TILESET_TEXTURE_FILE_PATH,
        );
        world.insert(game_map_resource.unwrap());

        let key_bindings_resource = match self.entity_type {
            EntityType::Vehicle => {
                KeyBindingsResource::load(EntityType::Vehicle, VEHICLE_BINDINGS_CONFIG_FILENAME)
            }
            EntityType::Pedestrian => KeyBindingsResource::load(
                EntityType::Pedestrian,
                PEDESTRIAN_BINDINGS_CONFIG_FILENAME,
            ),
        }.unwrap();

        match self.entity_type {
            EntityType::Vehicle => {
                self.sprite_sheet_handle = Some(VehicleResource::load(
                    world,
                    VEHICLE_TEXTURE_FILE_PATH,
                    VEHICLE_SPRITE_SHEET_FILE_PATH,
                    &mut self.progress_counter,
                ).unwrap());
            }
            _ => {}
        }

        let input_bundle = key_bindings_resource.get_input_bundle(&self.entity_type).unwrap();
        world.insert(input_bundle);
    }
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.insert(SystemActive::new(false));
        self.load_resources(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = &mut data.world;

        if self.progress_counter.is_complete() {
            // Check if the sprite sheet is loaded
            if let Some(sprite_sheet_handle) = &self.sprite_sheet_handle {
                let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
                if let Some(sprite_sheet) = sprite_sheet_storage.get(sprite_sheet_handle) {
                    // Sprite sheet is loaded; now we can generate hitboxes
                    let hitboxes = VehicleResource::generate_hitboxes(sprite_sheet);
                    // Initialize VehicleResource with sprite sheet handle and hitboxes
                    let vehicle_resource = VehicleResource { sprite_sheet_handle: sprite_sheet_handle.clone(), hitboxes };
                    // Insert or update VehicleResource in the world
                    drop(sprite_sheet_storage); // TODO lolol god dammit
                    world.insert(vehicle_resource);

                    // Loading is complete; transition to the main game state
                    return Trans::Switch(Box::new(Yakuzaishi::new(EntityType::Vehicle)));
                }
            }
        }

        Trans::None
    }
}