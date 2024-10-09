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

const WIDTH: f32 = 1024.0;
const HEIGHT: f32 = 1024.0;
const HALF_WIDTH: f32 = WIDTH / 2.0;
const HALF_HEIGHT: f32 = HEIGHT / 2.0;

const WORLD_TEXTURE_WIDTH: f32 = 1024.0;
const WORLD_TEXTURE_HEIGHT: f32 = 1024.0;


@fragment
fn fragment(out: VertexOutput) -> @location(0) vec4<f32> {
// TODO: this 0.46 constant is still confusing but i dont know how to center the forward camera vector
    let i: f32 = out.position.x * 0.43;
    let j: f32 = out.position.y;


    if (j < HALF_HEIGHT + mode7_material.altitude) {
        return vec4<f32>(1.0, 1.0, 1.0, 1.0); // Return white color
        }
    // TODO figure out normalization of clip and world coordinates im so confused
    let normalized_x = (out.position.x - HALF_WIDTH) / HALF_WIDTH; // Ranges from -1 to 1
    let normalized_y = (out.position.y - HALF_HEIGHT) / HALF_HEIGHT;


    let altitude: f32 = mode7_material.altitude;
    let scale_x: f32 = mode7_material.scaling.x;
    // TODO: FIGURE OUT A BETTER WAY TO MAKE IT CLEAR THAT we are on XZ plane, Y-UP
    let scale_z: f32 = mode7_material.scaling.y;

    // TODO: no idea how these work, the width comes from where, where am i in the world idk
    let x: f32 = HALF_WIDTH - i;
    let y: f32 = j - HALF_HEIGHT - mode7_material.altitude;
    let z: f32 = j - mode7_material.altitude; //TODO: multiply by 2 here to super speed

    let x_projected: f32 = x / y;
    let z_projected: f32 = z / y;

    let cos_theta = cos(mode7_material.rotation);
    let sin_theta = sin(mode7_material.rotation);
    let rotated_x = x_projected * cos_theta - z_projected * sin_theta;
    let rotated_z = x_projected * sin_theta + z_projected * cos_theta;

    let x_world = rotated_x - mode7_material.translation.x;
    let z_world = rotated_z + mode7_material.translation.y;


    let floor_pos_x = modulo(x_world * scale_x, WORLD_TEXTURE_WIDTH) / WORLD_TEXTURE_HEIGHT;
    let floor_pos_z = modulo(z_world * scale_z, WORLD_TEXTURE_HEIGHT) / WORLD_TEXTURE_HEIGHT;

    let floor_pos = vec2<f32>(floor_pos_x, floor_pos_z);

    let floor_col: vec4<f32> = textureSample(floor_texture, floor_sampler, floor_pos);

    // add cloud thing
    let attentuation = min(max(3.5 * (abs(y) / (HALF_HEIGHT + mode7_material.altitude)), 0.0), 1.0);
    let sky_gradient = (1 - attentuation);

    let final_floor_color = vec4(floor_col.x * attentuation + sky_gradient, floor_col.y * attentuation + sky_gradient, floor_col.z * attentuation + sky_gradient, floor_col.w);
    return final_floor_color;
}

fn modulo(a: f32, b: f32) -> f32 {
    var m = a % b;
    if (m < 0.0) {
        m += b;
    }
    return m;
}


