use bevy::math::Vec2;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;
use bevy_asset::{Asset, Handle};
use bevy_ecs_tilemap::prelude::MaterialTilemap;
use bevy_render::texture::Image;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SimpleTilemapMaterial {
    #[uniform(0)]
    pub time: f32,
    #[uniform(1)]
    pub tile_size: Vec2,
    #[uniform(2)]
    pub tileset_size: Vec2,
    #[texture(3)]
    #[sampler(4)]
    pub base_texture: Handle<Image>,
}

impl MaterialTilemap for SimpleTilemapMaterial {
    fn fragment_shader() -> ShaderRef {
        "shader_data/fog.wgsl".into()
    }
}
