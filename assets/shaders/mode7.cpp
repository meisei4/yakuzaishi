class mode7_b {

    float camera_x = 1000.0f;
    float camera_y = 1000.0f;
    float camera_dir = 0.1f;
    float near_plane_distance = 0.005f;  // Distance from camera to the near plane
    float far_plane_distance = 0.03f;    // Distance from camera to the far plane
    float fov_spread_left = -3.14159f / 4.0f;   // Left field of view offset (negative)
    float fov_spread_right = 3.14159f / 4.0f;   // Right field of view offset (positive)

protected:
    virtual bool OnUserUpdate(float fElapsedTime) override {
        // Camera position in homogeneous coordinates (4D vector)
        vec4<f32> camera_position = [
            camera_x,
            camera_y,
            0.0f,
            1.0f];

        // Calculate the angles for left and right frustum boundaries
        float camera_spread_left = camera_dir + fov_spread_left;
        float camera_spread_right = camera_dir + fov_spread_right;

        // 4x4 Rotation Matrix for Left FoV Boundary
        mat4x4<f32> leftward_fov = mat4x4<f32>(
            [cos(camera_spread_left), -sin(camera_spread_left), 0.0f, 0.0f],
            [sin(camera_spread_left),  cos(camera_spread_left), 0.0f, 0.0f],
            [0.0f,                     0.0f,                    1.0f, 0.0f],
            [0.0f,                     0.0f,                    0.0f, 1.0f]);

        // 4x4 Rotation Matrix for Right FoV Boundary
        mat4x4<f32> rightward_fov = mat4x4<f32>(
            [cos(camera_spread_right), -sin(camera_spread_right), 0.0f, 0.0f],
            [sin(camera_spread_right),  cos(camera_spread_right), 0.0f, 0.0f],
            [0.0f,                      0.0f,                     1.0f, 0.0f],
            [0.0f,                      0.0f,                     0.0f, 1.0f]);

        // 4D Unit vector along the X-axis
        vec4<f32> x_axis_unit_vector = [
            1.0f,
            0.0f,
            0.0f,
            1.0f];

        // Rotate the direction vector for the left and right frustum boundaries
        vec4<f32> frustum_leftward_vector = leftward_fov * x_axis_unit_vector;
        vec4<f32> frustum_rightward_vector = rightward_fov * x_axis_unit_vector;

        // Calculate frustum corner points by scaling the rotated directions and translating them by the camera's position
        vec4<f32> frustum_bottom_left = camera_position + frustum_leftward_vector * near_plane_distance;
        vec4<f32> frustum_top_left = camera_position + frustum_leftward_vector * far_plane_distance;

        vec4<f32> frustum_bottom_right = camera_position + frustum_rightward_vector * near_plane_distance;
        vec4<f32> frustum_top_right = camera_position + frustum_rightward_vector * far_plane_distance;

        // Precompute matrices for the frustum sides
        mat4x4<f32> frustum_left_side_dimensions = mat4x4<f32>(
            [frustum_bottom_left.x, frustum_top_left.x - frustum_bottom_left.x, 0.0f, 0.0f],
            [frustum_bottom_left.y, frustum_top_left.y - frustum_bottom_left.y, 0.0f, 0.0f],
            [0.0f,                   0.0f,                                      1.0f, 0.0f],
            [0.0f,                   0.0f,                                      0.0f, 1.0f]);

        mat4x4<f32> frustum_right_side_dimensions = mat4x4<f32>(
            [frustum_bottom_right.x, frustum_top_right.x - frustum_bottom_right.x, 0.0f, 0.0f],
            [frustum_bottom_right.y, frustum_top_right.y - frustum_bottom_right.y, 0.0f, 0.0f],
            [0.0f,                    0.0f,                                        1.0f, 0.0f],
            [0.0f,                    0.0f,                                        0.0f, 1.0f]);

        // Rendering Loop: Iterate over each row (y) of the screen's bottom half
        for (int y = 0; y < HALF_SCREEN_HEIGHT; y++) {
            // Calculate depth ratio for the current row based on distance from the near plane
            // TODO: IS THIS THE PERSPECTIVE DIVISION PART????
            float depth_ratio = y / HALF_SCREEN_HEIGHT;

            // Create interpolation vector for depth
            vec4<f32> depth_interpolation_vector = [
                1.0f,
                depth_ratio,
                0.0f,
                1.0f];

            // Calculate start and end points of the projected frustum slice at the current depth using matrix multiplication
            vec4<f32> frustum_left_side = frustum_left_side_dimensions * depth_interpolation_vector;
            vec4<f32> frustum_right_side = frustum_right_side_dimensions * depth_interpolation_vector;

            // Linearly interpolate across the screen width for each pixel (x)
            for (int x = 0; x < SCREEN_WIDTH; x++) {
                // Width ratio to interpolate between the start and end points of the frustum slice
                // TODO: MAYBE THIS THE PERSPECTIVE DIVISION PART????
                float width_ratio = x / SCREEN_WIDTH;

                // Create interpolation vector for width
                vec4<f32> width_vector = [
                    1.0f,
                    width_ratio,
                    0.0f,
                    1.0f];

                // finalize frustum matrix
                mat4x4<f32> frustum = mat4x4<f32>(
                    [frustum_left_side.x, frustum_right_side.x - frustum_left_side.x, 0.0f, 0.0f],
                    [frustum_left_side.y, frustum_right_side.y - frustum_left_side.y, 0.0f, 0.0f],
                    [0.0f,               0.0f,                                        1.0f, 0.0f],
                    [0.0f,               0.0f,                                        0.0f, 1.0f]);

                vec4<f32> frustrum_fragment_coordinates = frustum * width_vector;

                float fragment_x = frustrum_fragment_coordinates.x;
                float fragment_y = frustrum_fragment_coordinates.y;

                // Sample the ground texture and draw to the lower half of the screen
                wchar_t ground_sym = sprGround->SampleGlyph(fragment_x, fragment_y);
                short ground_col = sprGround->SampleColour(fragment_x, fragment_y);
                Draw(x, y + HALF_SCREEN_HEIGHT, ground_sym, ground_col);

                // Sample the sky texture and draw to the upper half of the screen (inverted Y)
                wchar_t sky_sym = sprSky->SampleGlyph(fragment_x, fragment_y);
                short sky_col = sprSky->SampleColour(fragment_x, fragment_y);
                Draw(x, HALF_SCREEN_HEIGHT - y, sky_sym, sky_col);
            }
        }
        return true;
    }
};

#import bevy_sprite::mesh2d_functions::{ get_world_from_local, mesh2d_position_local_to_clip, mesh2d_position_local_to_world };
#import bevy_sprite::mesh2d_view_bindings::view;
#import bevy_sprite::mesh2d_types::Mesh2d;


// TODO: add a proper uniform passed in from bevy
const SCREEN_WIDTH: f32 = 480.0;
const SCREEN_HEIGHT: f32 = 320.0;
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

fn depth_interpolate(j: f32, frustum_right_side_dimensions: mat4x4<f32>, frustum_left_side_dimensions: mat4x4<f32>) -> mat4x4<f32>{
    let depth_ratio: f32 = j / HALF_SCREEN_HEIGHT;

    // Create interpolation vector for depth
    let depth_interpolation_vector = vec4<f32>(
        1.0,
        depth_ratio,
        0.0,
        1.0);

    // Calculate start and end points of the projected frustum slice at the current depth using matrix multiplication
    let frustum_left_side_depth = frustum_left_side_dimensions * depth_interpolation_vector;
    let frustum_right_side_depth = frustum_right_side_dimensions * depth_interpolation_vector;

    return mat4x4<f32>(
        vec4<f32>(frustum_left_side_depth.x, frustum_right_side_depth.x - frustum_left_side_depth.x, 0.0, 0.0),
        vec4<f32>(frustum_left_side_depth.y, frustum_right_side_depth.y - frustum_left_side_depth.y, 0.0, 0.0),
        vec4<f32>(0.0,                        0.0,                                                   1.0, 0.0),
        vec4<f32>(0.0,                        0.0,                                                   0.0, 1.0));
}

fn width_interpolate(i: f32, depth_interpolated_frustum: mat4x4<f32>) -> vec4<f32> {
    let width_ratio: f32 = i / SCREEN_WIDTH;
    let width_vector = vec4<f32>(
        1.0,
        width_ratio,
        0.0,
        1.0);

    return depth_interpolated_frustum * width_vector;
}

@fragment
fn fragment(out: VertexOutput) -> @location(0) vec4<f32> {
    let camera_x: f32 = 1.0;
    let camera_y: f32 = 1.0;
    let camera_dir: f32 = 0.1;
    let near_plane_distance: f32 = 0.005;  // Distance from camera to the near plane
    let far_plane_distance: f32 = 0.03;    // Distance from camera to the far plane
    let fov_spread_left: f32 = -3.14159 / 4.0;   // Left field of view offset (negative)
    let fov_spread_right: f32 = 3.14159 / 4.0;   // Right field of view offset (positive)

    let camera_position = vec4<f32>(
        camera_x,
        camera_y,
        0.0,
        1.0
    );

    // Calculate the angles for left and right frustum boundaries
    let camera_spread_left: f32 = camera_dir + fov_spread_left;
    let camera_spread_right: f32 = camera_dir + fov_spread_right;

    // 4x4 Rotation Matrix for Left FoV Boundary
    let leftward_fov = mat4x4<f32>(
        vec4<f32>(cos(camera_spread_left), -sin(camera_spread_left), 0.0, 0.0),
        vec4<f32>(0.0,                      1.0,                     0.0, 0.0),
        vec4<f32>(sin(camera_spread_left),  cos(camera_spread_left), 1.0, 0.0),
        vec4<f32>(0.0,                     0.0,                      0.0, 1.0));

    // 4x4 Rotation Matrix for Right FoV Boundary
    let rightward_fov = mat4x4<f32>(
        vec4<f32>(cos(camera_spread_right), -sin(camera_spread_right), 0.0, 0.0),
        vec4<f32>(0.0,                      1.0,                       0.0, 0.0),
        vec4<f32>(sin(camera_spread_right),  cos(camera_spread_right), 1.0, 0.0),
        vec4<f32>(0.0,                      0.0,                       0.0, 1.0));

    // 4D Unit vector along the X-axis
    let x_axis_unit_vector = vec4<f32>(
        1.0,
        0.0,
        0.0,
        1.0);

    // Rotate the direction vector for the left and right frustum boundaries
    let frustum_leftward_vector: vec4<f32> = leftward_fov * x_axis_unit_vector;
    let frustum_rightward_vector: vec4<f32> = rightward_fov * x_axis_unit_vector;

        // Calculate frustum corner points by scaling the rotated directions and translating them by the camera's position
    let frustum_bottom_left: vec4<f32> = camera_position + frustum_leftward_vector * near_plane_distance;
    let frustum_top_left: vec4<f32> = camera_position + frustum_leftward_vector * far_plane_distance;

    let frustum_bottom_right: vec4<f32> = camera_position + frustum_rightward_vector * near_plane_distance;
    let frustum_top_right: vec4<f32> = camera_position + frustum_rightward_vector * far_plane_distance;

        // Precompute matrices for the frustum sides
    let frustum_left_side_dimensions = mat4x4<f32>(
        vec4<f32>(frustum_bottom_left.x, frustum_top_left.x - frustum_bottom_left.x, 0.0, 0.0),
        vec4<f32>(frustum_bottom_left.y, frustum_top_left.y - frustum_bottom_left.y, 0.0, 0.0),
        vec4<f32>(0.0,                   0.0,                                        1.0, 0.0),
        vec4<f32>(0.0,                   0.0,                                        0.0, 1.0));

    let frustum_right_side_dimensions = mat4x4<f32>(
        vec4<f32>(frustum_bottom_right.x, frustum_top_right.x - frustum_bottom_right.x, 0.0, 0.0),
        vec4<f32>(frustum_bottom_right.y, frustum_top_right.y - frustum_bottom_right.y, 0.0, 0.0),
        vec4<f32>(0.0,                    0.0,                                          1.0, 0.0),
        vec4<f32>(0.0,                    0.0,                                          0.0, 1.0));

    let i = out.position.x;
    let j = out.position.y;

    let depth_interpolated_frustum: mat4x4<f32> = depth_interpolate(i, frustum_right_side_dimensions, frustum_left_side_dimensions);
    let frustum_coordinates: vec4<f32> = width_interpolate(j, depth_interpolated_frustum);

    let texture_coordinates = vec2<f32>(frustum_coordinates.x, frustum_coordinates.y);
    return textureSample(floor_texture, floor_sampler, texture_coordinates);
}

