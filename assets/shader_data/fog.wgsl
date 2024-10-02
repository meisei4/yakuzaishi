#import bevy_ecs_tilemap::vertex_output::MeshVertexOutput
#import bevy_ecs_tilemap::common::process_fragment
#import bevy_ecs_tilemap::common::tilemap_data

struct FogMaterial {
    time: f32,
    density: f32,
    fog_color: vec3<f32>,
    wind_dir: vec2<f32>, // New uniform for wind direction
    _padding: vec2<f32>, // Ensure 16-byte alignment
};

@group(3) @binding(0)
var<uniform> material: FogMaterial;

// Hash function for noise
fn hash(pos: vec2<f32>) -> f32 {
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

    let a = hash(i);
    let b = hash(i + vec2<f32>(1.0, 0.0));
    let c = hash(i + vec2<f32>(0.0, 1.0));
    let d = hash(i + vec2<f32>(1.0, 1.0));

    let u = smoothstep(0.0, 1.0, f.x);
    let v = smoothstep(0.0, 1.0, f.y);

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


fn dynamic_fog(p: vec2<f32>, time: f32, wind_dir: vec2<f32>) -> f32 {
    // Normalize the wind direction for consistent influence
    let wind = normalize(wind_dir);

    // Define scales for various noise perturbations
    let wind_scale = 0.05;       // Spatial scale for wind perturbations
    let turbulence_scale1 = 0.5; // Scale for first turbulence layer
    let turbulence_scale2 = 1.2; // Scale for second turbulence layer
    let main_scale = 0.017;       // Spatial scale for main cloud structures

    // Define time multipliers for faster dynamics
    let time_speed = 2.0; // Increased for more rapid changes

    // Shift all noise layers and patterns by time to ensure the entire fog pattern moves left
    let time_offset = vec2<f32>(-time * 0.1, 0.0); // Control shift speed here

    // Shift the input position by time_offset for the entire fog system
    let p_shifted = p + time_offset;

    // Generate wind-influenced perturbations using fbm
    let wind_perturb_x = fbm(p_shifted * wind_scale + vec2<f32>(time * time_speed, time * time_speed), 3);
    let wind_perturb_y = fbm(p_shifted * wind_scale + vec2<f32>(time * time_speed + 10.0, time * time_speed + 10.0), 3);
    let wind_perturbation = vec2<f32>(wind_perturb_x, wind_perturb_y) * 0.1; // Adjust perturbation intensity as needed

    // Apply wind direction with perturbations to move the cloud pattern
    let p_moved = p_shifted + wind * time * 0.05; // Adjust wind's influence on movement

    // Scale the moved position for main cloud structures
    let p_scaled = p_moved * main_scale;

    // Warp the main noise coordinates with turbulence layers for internal chaos
    let warp_x = fbm(p_scaled * turbulence_scale1 + vec2<f32>(time * time_speed + 1.0, time * time_speed + 1.0), 3);
    let warp_y = fbm(p_scaled * turbulence_scale1 + vec2<f32>(time * time_speed + 2.0, time * time_speed + 2.0), 3);
    let warped_pos = p_scaled + vec2<f32>(warp_x, warp_y) * 0.1; // Adjust warp intensity as needed

    // Generate the main cloud noise with warped coordinates
    let main_noise = fbm(warped_pos, 10); // Higher octaves for detailed cloud structures

    // Additional turbulence to add more chaos within clouds
    let turbulence1 = fbm(p_scaled * turbulence_scale2 + vec2<f32>(time * time_speed + 3.0, time * time_speed + 3.0), 3);
    let turbulence2 = fbm(p_scaled * turbulence_scale2 + vec2<f32>(time * time_speed + 4.0, time * time_speed + 4.0), 2);
    let combined_turbulence = turbulence1 * 0.5 + turbulence2 * 0.25; // Adjust weights for desired effect

    // Combine main noise with turbulence to enhance internal chaos
    let combined = main_noise + combined_turbulence;

    // Refine smoothstep thresholds to maintain cloud volume while allowing internal variations
    let fog_fat = smoothstep(0.5, 0.8, combined);
    let fog_thin = smoothstep(0.0, 0.2, combined);

    let fog_density = fog_fat * 0.5 + fog_thin * 0.5;

    return fog_density;
}



@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    // Process the base color using existing fragment processing
    var base_color = process_fragment(in);

    // Compute World Position
    let tile_pos = vec2<f32>(f32(in.storage_position.x), f32(in.storage_position.y));
    let tile_size_with_spacing = tilemap_data.tile_size + tilemap_data.spacing;

    // Calculate the global tile position
    let global_tile_pos = tilemap_data.chunk_pos * tilemap_data.map_size + tile_pos;

    // Compute the world position by scaling and adding local UV coordinates
    let world_pos = global_tile_pos * tile_size_with_spacing + in.uv.xy * tilemap_data.tile_size;

    // Calculate Dynamic Fog Factor
    let fog_factor = clamp(material.density * dynamic_fog(world_pos, material.time, material.wind_dir), 0.0, 1.0);

    // Mix Base Color with Fog Color Based on Fog Factor
    let final_color = mix(base_color.rgb, material.fog_color, fog_factor);

    return vec4(final_color, base_color.a);
}
