#import bevy_sprite::mesh2d_functions::{ get_world_from_local, mesh2d_position_world_to_clip };
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import bevy_sprite::mesh2d_view_bindings::view;
#import bevy_sprite::mesh2d_types::Mesh2d;


struct Mode7Material {
    scaling: vec2<f32>,     // Scaling factors for the projection
    rotation: f32,          // Rotation around the viewer's Y axis
    translation: vec2<f32>, // Translation (camera position)
    altitude: f32,
};

@group(2) @binding(0) var<uniform> mode7_material: Mode7Material;
@group(2) @binding(1) var floor_texture: texture_2d<f32>;
@group(2) @binding(2) var floor_sampler: sampler;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};


// Keep the vertex shader simple as above
@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    out.position = vec4<f32>(vertex.position.xy, 0.0, 1.0); // JUST USE 1.0 for w?
    out.uv = vertex.uv;
    out.world_position = vec4<f32>(vertex.position, 1.0); // JUST USE 1.0 for w?

    return out;
}

const SCREEN_WIDTH: f32 = 1024.0;
const SCREEN_HEIGHT: f32 = 1024.0;
const HALF_WIDTH: f32 = SCREEN_WIDTH / 2.0;
const HALF_HEIGHT: f32 = SCREEN_HEIGHT / 2.0;

const WORLD_TEXTURE_WIDTH: f32 = 1024.0;
const WORLD_TEXTURE_HEIGHT: f32 = 1024.0;

// Constants for near and far planes, and field of view
const NEAR: f32 = 0.1; // Near plane distance
const FAR: f32 = 100.0; // Far plane distance
const FoV: f32 = 90.0; // Field of view in degrees
const FoVHalf: f32 = radians(FoV * 0.5); // Convert to radians and halve for calculations

fn fragment(out: VertexOutput) -> @location(0) vec4<f32> {
    let i: f32 = out.position.x;
    let j: f32 = out.position.y;

    // Skip rendering the sky for now; focus on the ground
    if (j < HALF_HEIGHT + mode7_material.altitude) {
        return vec4<f32>(1.0, 1.0, 1.0, 1.0); // Return white color for the sky
    }

    // Compute sample depth (non-linear scaling factor)
    var fSampleDepth: f32 = (j - (HALF_HEIGHT + mode7_material.altitude)) / (HALF_HEIGHT);
    fSampleDepth = max(fSampleDepth, 0.001); // Avoid division by zero

    // Compute the inverse of sample depth for non-linear scaling
    let scale: f32 = 1.0 / fSampleDepth;

    // Calculate the field of view scaling factors
    let aspect_ratio: f32 = SCREEN_WIDTH / SCREEN_HEIGHT;
    let fFoVScaleX: f32 = tan(FoVHalf) * aspect_ratio;
    let fFoVScaleY: f32 = tan(FoVHalf);

    // Map screen coordinates to normalized device coordinates (NDC)
    let ndc_x: f32 = (2.0 * i) / SCREEN_WIDTH - 1.0;
    let ndc_y: f32 = 1.0 - (2.0 * j) / SCREEN_HEIGHT; // Flip Y-axis for screen space

    // Apply non-linear scaling to NDC coordinates
    let x_camera: f32 = ndc_x * fFoVScaleX * scale;
    let y_camera: f32 = ndc_y * fFoVScaleY * scale;

    // Since we're rendering a flat ground, y_camera can represent the forward distance
    let world_x: f32 = mode7_material.translation.x + x_camera * mode7_material.scaling.x;
    let world_z: f32 = mode7_material.translation.y + fSampleDepth * FAR;

    // Apply rotation
    let cos_angle: f32 = cos(mode7_material.rotation);
    let sin_angle: f32 = sin(mode7_material.rotation);
    let rotated_x: f32 = world_x * cos_angle - world_z * sin_angle;
    let rotated_z: f32 = world_x * sin_angle + world_z * cos_angle;

    // Compute texture coordinates
    let floor_pos_x: f32 = rotated_x;
    let floor_pos_z: f32 = rotated_z;

    // Normalize texture coordinates
    let tex_u: f32 = fract(floor_pos_x / WORLD_TEXTURE_WIDTH);
    let tex_v: f32 = fract(floor_pos_z / WORLD_TEXTURE_HEIGHT);

    // Sample the floor texture
    let floor_col: vec4<f32> = textureSample(floor_texture, floor_sampler, vec2<f32>(tex_u, tex_v));

    // Optional: Add atmospheric attenuation for distance
    let attenuation: f32 = clamp(3.5 * fSampleDepth, 0.0, 1.0);
    let sky_gradient: f32 = 1.0 - attenuation;

    return vec4<f32>(
        floor_col.r * attenuation + sky_gradient,
        floor_col.g * attenuation + sky_gradient,
        floor_col.b * attenuation + sky_gradient,
        floor_col.a
    );
}

