use bevy::math::Vec3;
use bevy::reflect::TypePath;
use bevy_asset::{Asset, Handle};
use bevy_ecs_tilemap::prelude::MaterialTilemap;
use bevy_render::render_resource::{AsBindGroup, ShaderRef};
use bevy_render::texture::Image;

#[derive(AsBindGroup, TypePath, Debug, Default, Clone, Asset)]
pub struct FogMaterial {
    #[uniform(0)]
    pub time: f32,

    #[uniform(0)]
    pub density: f32,

    #[uniform(0)]
    pub fog_color: Vec3,

    #[uniform(0)]
    pub wind_dir: Vec3,

    // Padding to ensure 16-byte alignment (required by WGSL)
    #[uniform(0)]
    pub _padding: Vec3,
}

impl MaterialTilemap for FogMaterial {
    // fn vertex_shader() -> ShaderRef {
    //     "shaders/shader.wgsl".into()
    // }
    fn fragment_shader() -> ShaderRef {
        "shaders/fog.wgsl".into()
    }
}
