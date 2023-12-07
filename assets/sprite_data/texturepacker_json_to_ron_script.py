import json

def convert_json_to_ron_v2(json_data):
    sprites = []
    for frame in json_data['frames']:
        frame_data = frame['frame']
        x, y, w, h = frame_data['x'], frame_data['y'], frame_data['w'], frame_data['h']
        sprite = f"        (x: {x}, y: {y}, width: {w}, height: {h}, offsets: Some((0.0, 0.0))),"
        sprites.append(sprite)

    texture_width, texture_height = 100, 100

    ron_string = f"List((\n    texture_width: {texture_width},\n    texture_height: {texture_height},\n    sprites: [\n" + "\n".join(sprites) + "\n    ],\n))"
    return ron_string

with open('/mnt/data/brown_civic.json', 'r') as file:
    json_data = json.load(file)

ron_data_v2 = convert_json_to_ron_v2(json_data)

ron_file_path = '/mnt/data/converted_brown_civic.ron'
with open(ron_file_path, 'w') as file:
    file.write(ron_data_v2)
