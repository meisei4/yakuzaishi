use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::core::math::Vector2;

// Define different types of tiles that make up the game world
#[derive(Clone, Copy)]
pub enum Tile {
    Road,
    Building,
    Checkpoint,
    // ... any other types of tiles needed ...
}

// Define the WorldMap struct
pub struct WorldMap {
    // A 2D vector to represent a grid of tiles
    pub tiles: Vec<Vec<Tile>>,
    pub tile_size: Vector2<f32>, // The size of each tile in the game world
}

impl WorldMap {
    pub fn load_from_file(map_file_path: &str, tile_size: Vector2<f32>) -> Result<Self, Error> {
        // Open the file
        let file = File::open(map_file_path)?;
        let reader = io::BufReader::new(file);

        // Read the file line by line and fill the tiles vector
        let mut tiles = Vec::new();
        for (y, line) in reader.lines().enumerate() {
            let mut row = Vec::new();
            for (x, ch) in line?.chars().enumerate() {
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
        matches!(self.tiles.get(y).and_then(|row| row.get(x)), Some(Tile::Road | Tile::Checkpoint))
    }

    // Checks if a specific tile is a checkpoint
    pub fn is_checkpoint(&self, x: usize, y: usize) -> bool {
        matches!(self.tiles.get(y).and_then(|row| row.get(x)), Some(Tile::Checkpoint))
    }

    // Get the tile size
    pub fn get_tile_size(&self) -> Vector2<f32> {
        self.tile_size
    }

    // ... any additional methods needed for world interactions ...
}

// Implement the Component trait for WorldMap if you need to attach it to an Entity
impl Component for WorldMap {
    type Storage = DenseVecStorage<Self>;
}
