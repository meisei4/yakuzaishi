#import bevy_sprite::mesh2d_functions::{ get_world_from_local, mesh2d_position_local_to_clip, mesh2d_position_local_to_world };
#import bevy_sprite::mesh2d_view_bindings::view;
#import bevy_sprite::mesh2d_types::Mesh2d;


// TODO: add a proper uniform passed in from bevy
const SCREEN_WIDTH: f32 = 1024.0;
const SCREEN_HEIGHT: f32 = 1024.0;
const HALF_SCREEN_WIDTH: f32 = SCREEN_WIDTH / 2.0;
const HALF_SCREEN_HEIGHT: f32 = SCREEN_HEIGHT / 2.0;

const WORLD_TEXTURE_WIDTH: f32 = 1024.0;
const WORLD_TEXTURE_HEIGHT: f32 = 1024.0;


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

struct VertexOutput {
    // this is `clip position` when the struct is used as a vertex stage output
    // and `frag coord` when used as a fragment stage input
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    #ifdef VERTEX_TANGENTS
    @location(3) world_tangent: vec4<f32>,
    #endif
    #ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
    #endif
}

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

// TODO: DUMB AS SHIT VERTEX SHADER GET OUT
@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(vertex.position.xy, 0.0, 1.0);
    out.uv = vertex.uv;
    out.world_position = vec4<f32>(vertex.position, 1.0);
    return out;
}

@fragment
fn fragment(out: VertexOutput) -> @location(0) vec4<f32> {
    // i, and j represent the fragment position
    let i: f32 = out.position.x;
    let j: f32 = out.position.y;
    if (j < HALF_SCREEN_HEIGHT) {
        return vec4<f32>(1.0, 1.0, 1.0, 1.0); // Return white color
    }

    //offset X and Y fragment targets so that (0,0) is at the center of the screen?

    //TODO: you still barely know what i is... its any arbitrary point on the screen. in the x axis.
    let centered_x: f32 = (i - HALF_SCREEN_WIDTH) / (mode7_material.fov / 2.0);
    let centered_y: f32 = j - HALF_SCREEN_HEIGHT;

    let cam_altitude: f32 = mode7_material.altitude;
    let pitch = mode7_material.frustrum_x_rotation;
    let yaw = mode7_material.y_axis_rotation;

    let fragment: vec4<f32> = vec4<f32>(
        centered_x,
        centered_y - cam_altitude,
        cam_altitude,
        1.0 // HOMOGENOUS
    );

    //Rx(θ) = | 1      0         0  |
    //        | 0   cos(θ)  −sin(θ) |
    //        | 0   sin(θ)   cos(θ) |
    let Rx: mat4x4<f32> = mat4x4<f32>(
        vec4<f32>(1.0,      0.0,       0.0,     0.0),
        vec4<f32>(0.0, cos(pitch), -sin(pitch), 0.0),
        vec4<f32>(0.0, sin(pitch),  cos(pitch), 0.0),
        vec4<f32>(0.0,      0.0,       0.0,     1.0)
    );

    let rotated_fragment_pitch: vec4<f32> = Rx * fragment;

    let projected_x: f32 = rotated_fragment_pitch.x / rotated_fragment_pitch.y;
    let projected_z: f32 = rotated_fragment_pitch.z / rotated_fragment_pitch.y;

    let projected_fragment: vec4<f32> = vec4<f32>(
        projected_x,
        0.0,
        projected_z,
        1.0 // HOMOGENOUS
    );

    //Ry(ϕ) = |  cos(ϕ)   0   sin(ϕ) |
    //        |     0     1     0    |
    //        | −sin(ϕ)   0   cos(ϕ) |
    // WITH HOMOGENOUS
    let Ry: mat4x4<f32> = mat4x4<f32>(
        vec4<f32>(cos(yaw),  0.0, sin(yaw), 0.0),
        vec4<f32>(0.0,       1.0, 0.0,      0.0),
        vec4<f32>(-sin(yaw), 0.0, cos(yaw), 0.0),
        vec4<f32>(0.0,       0.0, 0.0,      1.0)
    );

    let rotated_fragment_yaw: vec4<f32> = Ry * projected_fragment;

    //T = | 1   0   0   0 |
    //    | 0   1   0   0 |
    //    | 0   0   1   0 |
    //    | t_x 0  t_z  1 |
    let t_x = mode7_material.translation.x;
    let t_z = mode7_material.translation.y;

    let translation_matrix: mat4x4<f32> = mat4x4<f32>(
        vec4<f32>(1.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(-t_x, 0.0, t_z, 1.0)
    );

    let translated_fragment: vec4<f32> = translation_matrix * rotated_fragment_yaw;

    //S = | s_x  0    0    0 |
    //    |  0   s_y  0    0 |
    //    |  0    0   s_z  0 |
    //    |  0    0    0   1 |
    let s_x = mode7_material.scaling.x;
    let s_z = mode7_material.scaling.y;

    let scaling_matrix: mat4x4<f32> = mat4x4<f32>(
        vec4<f32>(s_x, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, s_z, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    );

    let scaled_fragment: vec4<f32> = scaling_matrix * translated_fragment;

    let scaled_x: f32 = scaled_fragment.x / WORLD_TEXTURE_WIDTH;
    let scaled_y: f32 = scaled_fragment.y; // Included for completeness
    let scaled_z: f32 = scaled_fragment.z / WORLD_TEXTURE_HEIGHT;

   // Check if the texture X,Z coordinates are within texture bounds
    if (scaled_x < 0.0 || scaled_x > 1.0 || scaled_z < 0.0 || scaled_z > 1.0) {
        return vec4<f32>(1.0, 1.0, 1.0, 1.0); //paint outside world white
    }

    let texture_coordinates = vec2<f32>(scaled_x, scaled_z);

    let color: vec4<f32> = textureSample(floor_texture, floor_sampler, texture_coordinates);

    // add cloud feature for faraway distance
    let attenuation = min(max(3.5 * (abs(centered_y) / (HALF_SCREEN_HEIGHT + mode7_material.altitude)), 0.0), 1.0);
    let sky_gradient = 1.0 - attenuation;

    return vec4<f32>(
        color.x * attenuation + sky_gradient,
        color.y * attenuation + sky_gradient,
        color.z * attenuation + sky_gradient,
        color.w
    );
}
