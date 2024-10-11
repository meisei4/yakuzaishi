#import bevy_sprite::mesh2d_functions::{ get_world_from_local, mesh2d_position_world_to_clip };
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import bevy_sprite::mesh2d_view_bindings::view;
#import bevy_sprite::mesh2d_types::Mesh2d;


struct Mode7Material {
    scaling: vec2<f32>,           // Scaling factors for the projection
    fov: f32,
    frustrum_x_rotation: f32,
    y_axis_rotation: f32,
    translation: vec2<f32>,       // Translation (camera XZ position)
    altitude: f32,                // Camera Y axis "position"
};

@group(2) @binding(0) var<uniform> mode7_material: Mode7Material;
@group(2) @binding(1) var floor_texture: texture_2d<f32>;
@group(2) @binding(2) var floor_sampler: sampler;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    out.position = vec4<f32>(vertex.position.xy, 0.0, 1.0);
    out.uv = vertex.uv;
    out.world_position = vec4<f32>(vertex.position, 1.0);

    return out;
}

const WIDTH: f32 = 1024.0;
const HEIGHT: f32 = 1024.0;
const HALF_WIDTH: f32 = WIDTH / 2.0;
const HALF_HEIGHT: f32 = HEIGHT / 2.0;

const WORLD_TEXTURE_WIDTH: f32 = 1024.0;
const WORLD_TEXTURE_HEIGHT: f32 = 1024.0;

@fragment
fn fragment(out: VertexOutput) -> @location(0) vec4<f32> {
    let i: f32 = out.position.x * mode7_material.fov;
    let j: f32 = out.position.y;

    if (j < HALF_HEIGHT) {
        return vec4<f32>(1.0, 1.0, 1.0, 1.0); // Return white color
    }

    //TODO: i still dont get this (bevy issue?)
    //offset X and Y fragment targets so that (0,0) is at the center of the screen (bevy camera stuff?)
    let centered_x: f32 = HALF_WIDTH - i ;
    let centered_y: f32 = j - HALF_HEIGHT;

    let cam_altitude: f32 = mode7_material.altitude;
    let pitch = mode7_material.frustrum_x_rotation;
    let yaw = mode7_material.y_axis_rotation;

    // Apply pitch rotation over the camera's relative x-axis (looking up/down)
    let cos_pitch = cos(pitch);
    let sin_pitch = sin(pitch);

    let adjusted_y = (centered_y - cam_altitude) * cos_pitch - cam_altitude * sin_pitch;
    let adjusted_z = cam_altitude * cos_pitch + (centered_y - cam_altitude) * sin_pitch;

    let projected_x: f32 = centered_x / adjusted_y;
    let projected_z: f32 = adjusted_z / adjusted_y;

    let cos_yaw = cos(yaw);
    let sin_yaw = sin(yaw);

    let rotated_x = projected_x * cos_yaw - projected_z * sin_yaw;
    let rotated_z = projected_x * sin_yaw + projected_z * cos_yaw;

    let translated_x = rotated_x - mode7_material.translation.x;
    let translated_z = rotated_z + mode7_material.translation.y;

    let scaled_x = translated_x * mode7_material.scaling.x;
    let scaled_z = translated_z * mode7_material.scaling.y;

   // Check if the texture coordinates are within texture bounds
    if (scaled_x < 0.0 || scaled_x > WORLD_TEXTURE_WIDTH || scaled_z < 0.0 || scaled_z > WORLD_TEXTURE_HEIGHT) {
        return vec4<f32>(1.0, 1.0, 1.0, 1.0); //paint outside world white
    }

    let texture_u = scaled_x / WORLD_TEXTURE_WIDTH;
    let texture_v = scaled_z / WORLD_TEXTURE_HEIGHT;

    let texture_coordinates = vec2<f32>(texture_u, texture_v);

    let floor_color: vec4<f32> = textureSample(floor_texture, floor_sampler, texture_coordinates);

    // add cloud feature for faraway distance
    let attenuation = min(max(3.5 * (abs(centered_y) / (HALF_HEIGHT + mode7_material.altitude)), 0.0), 1.0);
    let sky_gradient = 1.0 - attenuation;

    return vec4<f32>(
        floor_color.x * attenuation + sky_gradient,
        floor_color.y * attenuation + sky_gradient,
        floor_color.z * attenuation + sky_gradient,
        floor_color.w
    );
}
//    let near_x1 = x_projected * cos(mode7_material.rotation) * NEAR;
//    let near_z1 = z_projected * sin(mode7_material.rotation) * NEAR;
//
//    let far_x1 = x_projected * cos(mode7_material.rotation) * FAR;
//    let far_z1 = z_projected * sin(mode7_material.rotation) * FAR;
//
//    let near_x2 = x_projected * cos(mode7_material.rotation) * NEAR;
//    let near_z2 = z_projected * sin(mode7_material.rotation) * NEAR;
//
//    let far_x2 = x_projected * cos(mode7_material.rotation) * FAR;
//    let far_z2 = z_projected * sin(mode7_material.rotation) * FAR;
