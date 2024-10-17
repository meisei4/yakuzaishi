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
