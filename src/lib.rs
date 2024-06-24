use std::env;
use std::path::PathBuf;

pub mod components;
pub mod systems;
pub mod states;

lazy_static::lazy_static! {
    pub static ref ASSETS_BASE_PATH: PathBuf = env::current_dir().unwrap().join("assets");
}

pub const VEHICLE_SPRITE_SHEET_FILE_PATH: &str =
    "assets/sprite_data/vehicle_spritesheet.ron";
pub const VEHICLE_TEXTURE_FILE_PATH: &str = "sprite_data/vehicle_spritesheet.png";

pub const DEFAULT_SPEED: f32 = 150.0;
pub const DEFAULT_RATE_OF_ROTATION: f32 = 4.0;

pub const VEHICLE_DEFAULT_MAX_SPEED: f32 = 150.0;
pub const VEHICLE_DEFAULT_ACCELERATION: f32 = 80.0;
pub const VEHICLE_DEFAULT_DECELERATION: f32 = 100.0;
pub const VEHICLE_DEFAULT_RATE_OF_ROTATION: f32 = 4.0;

pub const OCEAN_MAP_FILE_PATH: &str = "map_data/small_map.tmx";

pub const TILE_SIZE: f32 = 64.0;

pub const NINTENDO_DS_SCREEN_WIDTH: f32 = 256.0;
pub const NINTENDO_DS_SCREEN_HEIGHT: f32 = 384.0;
