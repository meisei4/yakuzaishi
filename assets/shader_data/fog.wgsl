@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate the tile's UV coordinates within the tileset
    let tiles_per_row = TilesetSize.x / TileSize.x;
    let tile_x = f32(input.tile_index % u32(tiles_per_row));
    let tile_y = f32(input.tile_index / u32(tiles_per_row));

    let tile_uv = input.uv * TileSize / TilesetSize + vec2<f32>(
        tile_x * TileSize.x / TilesetSize.x,
        tile_y * TileSize.y / TilesetSize.y,
    );

    // Sample the base texture for the specific tile
    let base_color = textureSample(BaseTexture, BaseSampler, tile_uv);

    // **Global Coordinates Calculation**
    // Calculate a global position for continuity across tiles (based on tilemap position)
    let global_position = input.position.xy + vec2<f32>(tile_x * TileSize.x, tile_y * TileSize.y);

    // Apply a simple time-based brightness modulation using global position
    let brightness = 0.5 + 0.5 * sin(global_position.x * 0.1 + Time); // Modify as needed
    let final_color = base_color * vec4<f32>(brightness, brightness, brightness, 1.0);

    return final_color;
}
