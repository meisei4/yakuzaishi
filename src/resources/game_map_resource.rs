use std::path::Path;
use tiled::Loader;

pub struct GameMap {
    pub map: tiled::Map,
}

impl GameMap {

    pub fn new(map_file_path: &str) -> Self {
        let mut loader: Loader = Loader::new();
        let map: tiled::Map = loader
            .load_tmx_map(Path::new(map_file_path))
            .expect("Failed to load map");
        GameMap { map: map }
    }

}