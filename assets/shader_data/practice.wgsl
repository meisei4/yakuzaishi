// Vertex shader
struct VertexInput {
    @location(0) position: vec3<f32>,  // Vertex position
    @location(1) uv: vec2<f32>      // UV coordinates for the texture
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,  // Clip space position
    @location(0) uv: vec2<f32>                   // Pass UV to fragment shader
};

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // We assume the position is in world space, so we transform it to clip space
    output.clip_position = vec4<f32>(input.position, 1.0);

    // Pass UV coordinates to the fragment shader
    output.uv = input.uv;

    return output;
}

// Fragment shader
struct FragmentInput {
    @location(0) uv: vec2<f32>  // UV coordinates from vertex shader
};

struct CustomMaterial {
    time: f32  // Time uniform passed from Bevy's CustomMaterial
};

@group(1) @binding(0)
var<uniform> custom_material: CustomMaterial;

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    // Access the time variable
    let time = custom_material.time;

    // Here you can use UV coordinates and time to create effects
    let uv = input.uv;

    // For now, just returning a simple color (white) for simplicity
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
