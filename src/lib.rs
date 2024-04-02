pub mod components;
pub mod helpers_hack;
pub mod startup;
pub mod systems;

pub const VEHICLE_SPRITE_SHEET_FILE_PATH: &'static str =
    "assets/sprite_data/vehicle_spritesheet.ron";
pub const VEHICLE_TEXTURE_FILE_PATH: &'static str = "sprite_data/vehicle_spritesheet.png";

pub const VEHICLE_DEFAULT_MAX_SPEED: f32 = 150.0;
pub const VEHICLE_DEFAULT_ACCELERATION: f32 = 80.0;
pub const VEHICLE_DEFAULT_DECELERATION: f32 = 100.0;
pub const VEHICLE_DEFAULT_RATE_OF_ROTATION: f32 = 4.0;

// TODO -------- v fix the assets vs project root folder loading -------------
pub const MAP_FILE_PATH_ASSET: &'static str = "map_data/road_tilemap_1.tmx";
pub const OCEAN_MAP_FILE_PATH_ASSET: &'static str = "map_data/ocean_map.tmx";
pub const MAP_FILE_PATH: &'static str = "assets/map_data/road_tilemap_1.tmx";
pub const OCEAN_MAP_FILE_PATH: &'static str = "assets/map_data/ocean_map.tmx";
// TODO -------- ^ fix the assets vs project root folder loading -------------

pub const FONT_PATH: &'static str = "font_data/saturn.ttf";

pub const TILE_SIZE: f32 = 64.0;

pub const NINTENDO_DS_SCREEN_WIDTH: f32 = 256.0;
pub const NINTENDO_DS_SCREEN_HEIGHT: f32 = 384.0;
