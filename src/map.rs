use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Road,
    Building,
    Checkpoint,
}

pub struct GameMap {
    pub tiles: Vec<Vec<Tile>>,   // A 2D vector to represent a grid of tiles
    pub tile_size: Vector2<f32>, // The size of each tile in the game world
}

impl GameMap {
    pub fn load_from_file(map_file_path: &str, tile_size: Vector2<f32>) -> Result<Self, io::Error> {
        let file = File::open(map_file_path)?;
        let reader = io::BufReader::new(file);

        // Read the file line by line and fill the tiles vector
        let mut tiles = Vec::new();
        for line in reader.lines() {
            let mut row = Vec::new();
            for ch in line?.chars() {
                let tile = match ch {
                    '═' => Tile::Road,
                    '█' => Tile::Building,
                    // Add cases for other characters and tile types as needed
                    _ => continue, // Skip unknown characters or handle them as needed
                };
                row.push(tile);
            }
            tiles.push(row);
        }

        Ok(Self { tiles, tile_size })
    }

    // Checks if a specific tile is drivable
    pub fn is_drivable(&self, x: usize, y: usize) -> bool {
        self.tiles
            .get(y)
            .and_then(|row| row.get(x))
            .map(|&tile| tile == Tile::Road || tile == Tile::Checkpoint)
            .unwrap_or(false)
    }

    // Checks if a specific tile is a checkpoint
    // pub fn is_checkpoint(&self, x: usize, y: usize) -> bool {
    //     self.tiles
    //         .get(y)
    //         .and_then(|row| row.get(x))
    //         .map(|&tile| tile == Tile::Checkpoint)
    //         .unwrap_or(false)
    // }
}

// Implement the Component trait for GameMap if you need to attach it to an Entity
impl Component for GameMap {
    type Storage = DenseVecStorage<Self>;
}
