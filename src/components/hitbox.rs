use amethyst::core::math::Vector2;

#[derive(Clone, Debug)]
pub struct Hitbox {
    pub corners: [Vector2<f32>; 4],
    pub midpoints: [Vector2<f32>; 4],
}

impl Hitbox {
    pub fn new() -> Self {
        Hitbox {
            corners: [Vector2::zeros(); 4],
            midpoints: [Vector2::zeros(); 4],
        }
    }
}
