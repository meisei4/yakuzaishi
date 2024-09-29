#import bevy_ecs_tilemap::common::process_fragment
#import bevy_ecs_tilemap::vertex_output::MeshVertexOutput

struct FogMaterial {
    time: f32,
    density: f32,
    fog_color: vec3<f32>,
    _padding: vec3<f32>,
};

@group(3) @binding(0)
var<uniform> material: FogMaterial;

fn hash(pos: vec2<f32>) -> f32 {
    // A common hash function using sine and dot product
    return fract(sin(dot(pos, vec2(12.9898, 78.233))) * 43758.5453);
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
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    var base_color = process_fragment(in);

    let pixel_coordinates = in.position.xy;

    // Calculate dynamic fog factor
    let fog_factor = clamp(material.density * dynamic_fog(pixel_coordinates, material.time), 0.0, 1.0);

    // Mix base color with fog color based on fog factor
    let final_color = mix(base_color.rgb, material.fog_color, fog_factor);

    return vec4(final_color, base_color.a);
}