pub mod util;
pub mod systems;
pub mod state;
pub mod resources;
pub mod components;
pub mod camera;

pub const SPRITE_SHEET_FILE_PATH: &str = "sprite_data/brown_civic_sprite_sheet.ron";
pub const VEHICLE_TEXTURE_FILE_PATH: &str = "sprite_data/brown_civic_sprites.png";

pub const MAP_FILE_PATH: &str = "assets/map_data/road_tilemap_1.tmx";
pub const TILESET_FILE_PATH: &str = "assets/map_data/road_tileset.tsx";
pub const TILESET_TEXTURE_FILE_PATH: &str = "map_data/tileset_texture.png";

pub const DISPLAY_CONFIG_FILENAME: &str = "display_config.ron";
pub const BINDINGS_CONFIG_FILENAME: &str = "key_bindings.ron";

pub const TILE_SIZE: f32 = 64.0;
pub const MAP_HEIGHT: f32 = 10.0; //unit is tiles
pub const MAP_WIDTH: f32 = 10.0;