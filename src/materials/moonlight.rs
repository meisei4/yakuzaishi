use bevy::{
    math::{Vec2, Vec3},
    prelude::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};
use bevy_asset::{Asset, Handle};
use bevy_render::texture::Image;

#[derive(AsBindGroup, TypePath, Debug, Clone, Asset)]
pub struct MoonlightMaterial {
    #[uniform(0)]
    pub light_position: Vec2,
    #[uniform(0)]
    pub light_intensity: f32,
    #[uniform(0)]
    pub light_color: Vec3,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl Material2d for MoonlightMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/reflections.wgsl".into()
    }
}
