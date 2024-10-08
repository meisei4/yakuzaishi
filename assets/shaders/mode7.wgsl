#import bevy_sprite::mesh2d_functions::{ get_world_from_local, mesh2d_position_world_to_clip };
#import bevy_sprite::mesh2d_vertex_output::VertexOutput;
#import bevy_sprite::mesh2d_view_bindings::view;
#import bevy_sprite::mesh2d_types::Mesh2d;


struct Mode7Material {
    scaling: vec2<f32>,     // Scaling factors for the projection
    rotation: f32,          // Rotation around the viewer's Y axis
    translation: vec2<f32>, // Translation (camera position)
};

@group(2) @binding(0) var<uniform> mode7_material: Mode7Material;
@group(2) @binding(1) var map_texture: texture_2d<f32>;
@group(2) @binding(2) var map_sampler: sampler;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let world_from_local = get_world_from_local(vertex.instance_index);

    // Simple scaling factors
    let sx = mode7_material.scaling.x;
    let sy = mode7_material.scaling.y;

    // Apply rotation
    let cos_r = cos(mode7_material.rotation);
    let sin_r = sin(mode7_material.rotation);

    // Construct transformation matrix with scaling and rotation
    let transform_matrix = mat4x4<f32>(
        vec4<f32>(sx * cos_r, sx * sin_r, 0.0, 0.0),
        vec4<f32>(-sy * sin_r, sy * cos_r, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    );

    // Apply the transformation matrix
    var transformed_pos = transform_matrix * vec4<f32>(vertex.position, 1.0);

    // Transform to world space and then clip space
    let world_position = world_from_local * transformed_pos;
    out.position = mesh2d_position_world_to_clip(world_position);

    // Pass through UVs unchanged
    out.uv = vertex.uv; // No perspective scaling yet
    out.world_position = world_position;

    return out;
}

@fragment
fn fragment(out: VertexOutput) -> @location(0) vec4<f32> {

    // TODO this debugs a gradient to look at
    return vec4<f32>(
        out.world_position.x * 0.01,  // Scale down to fit in color range
        out.world_position.y * 0.01,
        0.0,                          // Leave z as 0 for simplicity
        1.0
    );
    // TODO: this really doesnt seem to work how i want it. It only seems to make the gradient,
    // and then paste the map texture as-is on top,
    // I think this is because the uv coordinates arent connected to the actual image correctly?
    //return textureSample(map_texture, map_sampler, out.uv);
}
