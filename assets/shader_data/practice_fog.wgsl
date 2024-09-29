// practice_fog.wgsl

// Vertex Shader

struct VertexInput {
    @location(0) position: vec3<f32>,  // Vertex position
    @location(1) uv: vec2<f32>,        // UV coordinates for the texture
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,  // Clip space position
    @location(0) uv: vec2<f32>,                  // Pass UV to fragment shader
};

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // Transform the position to clip space
    output.clip_position = vec4<f32>(input.position, 1.0);

    // Pass UV coordinates to the fragment shader
    output.uv = input.uv;

    return output;
}


// Fragment Shader

struct FragmentInput {
    @location(0) uv: vec2<f32>,  // UV coordinates from vertex shader
};

// Define your custom material with necessary uniforms
struct FogMaterial {
    time: f32,                  // Time uniform for animation
    density: f32,               // Fog density
    fog_color: vec3<f32>,       // Fog color
    _padding: vec3<f32>,        // Padding for uniform alignment
};

@group(0) @binding(0)
var<uniform> custom_material: FogMaterial;

// Bindings for tile texture and sampler
@group(1) @binding(0)
var tile_texture: texture_2d<f32>;

@group(1) @binding(1)
var tile_sampler: sampler;

// Hash function for noise generation
fn hash(pos: vec2<f32>) -> f32 {
    return fract(sin(dot(pos, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

// Smoothstep function for smoother interpolation
fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    return t * t * (3.0 - 2.0 * t);
}

// Linear interpolation
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a + t * (b - a);
}

// 2D Value Noise function
fn value_noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = p - i;

    // Four corners
    let a = hash(i);
    let b = hash(i + vec2<f32>(1.0, 0.0));
    let c = hash(i + vec2<f32>(0.0, 1.0));
    let d = hash(i + vec2<f32>(1.0, 1.0));

    // Smooth interpolation
    let u = smoothstep(0.0, 1.0, f.x);
    let v = smoothstep(0.0, 1.0, f.y);

    // Interpolate between the four corners
    let res = lerp(lerp(a, b, u), lerp(c, d, u), v);
    return res;
}

// Fractal Brownian Motion (fBM) for multiple noise layers
fn fbm(p: vec2<f32>, octaves: i32) -> f32 {
    var frequency: f32 = 1.0;
    var amplitude: f32 = 0.5;
    var noise_value: f32 = 0.0;

    for (var i: i32 = 0; i < octaves; i = i + 1) {
        noise_value = noise_value + amplitude * value_noise(p * frequency);
        frequency = frequency * 2.0;
        amplitude = amplitude * 0.5;
    }

    return noise_value;
}

// Function to create dynamic, globular fog using fBM
fn dynamic_fog(p: vec2<f32>, time: f32) -> f32 {
    // Offset to animate the fog movement
    let motion = vec2<f32>(time * 0.5, time * 2.0);
    let p_moved = p + motion;

    // Apply fBM with 4 octaves for complexity
    let n = fbm(p_moved * 0.1, 4);

    // Enhance contrast to create distinct fog clouds
    let fog_density = smoothstep(0.4, 0.6, n);

    return fog_density;
}

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    // Sample the tile's texture color
    let tile_color = textureSample(tile_texture, tile_sampler, input.uv).rgb;

    // Calculate dynamic fog factor
    let fog_factor = clamp(custom_material.density * dynamic_fog(input.uv, custom_material.time), 0.0, 1.0);

    // Mix tile color with fog color based on fog factor
    let final_color = mix(tile_color, custom_material.fog_color, fog_factor);

    // Return the final color with full opacity
    return vec4<f32>(final_color, 1.0);
}
