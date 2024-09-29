use bevy::{reflect::TypePath, render::render_resource::AsBindGroup};
use bevy::math::Vec3;
use bevy_asset::Asset;
use bevy_ecs_tilemap::prelude::MaterialTilemap;
use bevy_render::render_resource::ShaderRef;

#[derive(AsBindGroup, TypePath, Debug, Default, Clone, Asset)]
pub struct FogMaterial {
    #[uniform(0)]
    pub time: f32,

    #[uniform(0)]
    pub density: f32,

    #[uniform(0)]
    pub fog_color: Vec3,

    // Padding to ensure 16-byte alignment (required by WGSL)
    #[uniform(0)]
    pub _padding: Vec3,
}

impl MaterialTilemap for FogMaterial {
    fn fragment_shader() -> ShaderRef {
        "shader_data/fog.wgsl".into()
    }
}
