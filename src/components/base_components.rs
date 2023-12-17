use amethyst::core::math::Vector2;


// TODO there really needs to be a better way than this for shared attributes between components (maybe just write getters and setters...)
pub struct BaseEntityComponents {
    pub position: Vector2<f32>,
    pub speed: f32,
    pub current_sprite_index: usize,
}

impl BaseEntityComponents {
    pub fn new(position: Vector2<f32>, speed: f32, current_sprite_index: usize) -> Self {
        BaseEntityComponents {
            position,
            speed,
            current_sprite_index,
        }
    }
}
