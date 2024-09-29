use bevy::{reflect::TypePath, render::render_resource::AsBindGroup};
use bevy::math::Vec3;
use bevy_asset::{Asset, Handle};
use bevy_ecs_tilemap::prelude::MaterialTilemap;
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_resource::{
    AsBindGroupError, BindGroupLayout, BindGroupLayoutEntry, ShaderRef, UnpreparedBindGroup,
};
use bevy_render::renderer::RenderDevice;
use bevy_render::texture::{FallbackImage, GpuImage, Image};

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

    #[texture(1)]
    #[sampler(1)]
    pub tile_texture: Handle<Image>,
    // TODO: this next part just means i dont know anything about the grouping and binding macros or whatever
    // /// Tile texture binding (group 1, binding 0).
    // #[texture(1, binding = 0)]
    // pub tile_texture: Handle<Image>,
    //
    // /// Tile sampler binding (group 1, binding 1).
    // #[sampler(1, binding = 1)]
    // pub tile_sampler: Handle<Sampler>,
}

impl MaterialTilemap for FogMaterial {
    fn vertex_shader() -> ShaderRef {
        "shader_data/practice_fog.wgsl".into() // Path to your custom vertex shader
    }

    fn fragment_shader() -> ShaderRef {
        //"shader_data/fog.wgsl".into()
        "shader_data/practice_fog.wgsl".into()
    }
}
