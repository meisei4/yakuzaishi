use bevy::math::{Vec2, Vec3};
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;
use bevy_asset::{Asset, Handle};
use bevy_render::texture::Image;

#[derive(AsBindGroup, TypePath, Debug, Clone, Asset)]
pub struct Mode7Material {
    #[uniform(0)]
    pub scaling: Vec2,

    #[uniform(0)]
    pub rotation: f32,

    #[uniform(0)]
    pub translation: Vec2,

    #[uniform(0)]
    pub altitude: f32,

    #[uniform(0)]
    pub _padding: Vec3,

    #[texture(1)]
    #[sampler(2)]
    pub map_texture: Handle<Image>,
}

impl Material2d for Mode7Material {
    fn vertex_shader() -> ShaderRef {
        "shaders/mode7.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/mode7.wgsl".into()
    }
}