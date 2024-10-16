struct ReflectionMaterial {
    light_position: vec2<f32>,
    light_intensity: f32,
    light_color: vec3<f32>,
};

@group(1) @binding(0) var<uniform> material: ReflectionMaterial;
@group(1) @binding(1) var texture_diffuse: texture_2d<f32>;
@group(1) @binding(2) var texture_diffuse_sampler: sampler;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(texture_diffuse, texture_diffuse_sampler, input.uv);

    // Calculate the distance from the current fragment to the light source
    let light_dir = material.light_position - input.uv;
    let distance = length(light_dir);

    // Simple attenuation based on distance
    let attenuation = material.light_intensity / (distance * distance);

    // Calculate lighting
    let lighting = attenuation * material.light_color;

    // Apply lighting to the texture color
    return vec4<f32>(tex_color.rgb * lighting, tex_color.a);
}
