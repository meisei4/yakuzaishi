use std::env;
use std::path::PathBuf;

pub mod components;
pub mod events;
pub mod resources;
pub mod states;
pub mod systems;

lazy_static::lazy_static! {
    pub static ref ASSETS_BASE_PATH: PathBuf = env::current_dir().unwrap().join("assets");
}

pub const DEFAULT_SPEED: f32 = 90.0;
pub const VEHICLE_DEFAULT_MAX_SPEED: f32 = 150.0;
pub const VEHICLE_DEFAULT_ACCELERATION: f32 = 80.0;
pub const VEHICLE_DEFAULT_DECELERATION: f32 = 100.0;

pub const CAMERA_SCALE_MULTIPLIER: f32 = 0.5;

pub const CAMERA_Z_LEVEL: f32 = 3.0;
pub const OVERLAY_ANIMATIONS_Z_LEVEL: f32 = 2.0; //TODO: currently overlay Transform inherits from Parent Entity it is attached to, use later
pub const CONTROLLABLE_ENTITY_Z_LEVEL: f32 = 1.0;

pub const DEFAULT_SPAWN_TILE_X: f32 = 0.0;
pub const DEFAULT_SPAWN_TILE_Y: f32 = 0.0;

pub const TILE_SIZE: f32 = 64.0;
pub const NINTENDO_DS_SCREEN_WIDTH: f32 = 256.0;
pub const NINTENDO_DS_SCREEN_HEIGHT: f32 = 384.0;

pub const VEHICLE_TEXTURE_FILE_PATH: &str = "sprite_data/iruka.png";

pub const CONTROLLABLE_ENTITY_ANIMATION_TEXTURE_START_IDX: usize = 0;
pub const CONTROLLABLE_ENTITY_ANIMATION_TEXTURE_END_IDX: usize = 0;

pub const WAKE_ANIMATION_FILE_PATH: &str = "sprite_data/random_test_animations.png";
pub const WAKE_ANIMATION_TILE_COLUMN_LENGTH: usize = 20;
pub const WAKE_ANIMATION_TILE_ROW_LENGTH: usize = 1;

pub const OVERLAY_ANIMATION_TEXTURE_START_IDX: u32 = 0;
pub const OVERLAY_ANIMATION_TEXTURE_END_IDX: u32 = 19;

pub const OCEAN_MAP_FILE_PATH: &str = "map_data/small_map.tmx";
pub const TILE_ANIMATION_TEXTURE_START_IDX: usize = 40;
pub const TILE_ANIMATION_TEXTURE_END_IDX: usize = 59;
