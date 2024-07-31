use bevy::prelude::{Handle, Resource, TextureAtlasLayout};

#[derive(Resource)]
pub struct VehicleTextureAtlasHandle {
    pub handle: Handle<TextureAtlasLayout>,
}
