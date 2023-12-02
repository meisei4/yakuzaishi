from PIL import Image, ImageDraw

def create_map_from_text(text_file_path, output_image_path):
    # Read the text file
    with open(text_file_path, 'r') as file:
        lines = file.readlines()

    # Calculate the image size
    tile_size = 64
    image_width = len(lines[0].strip()) * tile_size
    image_height = len(lines) * tile_size

    # Create a new image with a white background
    image = Image.new('RGB', (image_width, image_height), 'white')
    draw = ImageDraw.Draw(image)

    # Process each character and draw corresponding tiles
    for y, line in enumerate(lines):
        for x, char in enumerate(line.strip()):
            upper_left = (x * tile_size, y * tile_size)
            lower_right = (upper_left[0] + tile_size, upper_left[1] + tile_size)

            if char == '═':
                # Draw a grey square
                draw.rectangle([upper_left, lower_right], fill='grey')
            elif char == '█':
                # Draw a green square with a brown center
                draw.rectangle([upper_left, lower_right], fill='green')
                center_square_size = 42
                center_upper_left = (upper_left[0] + (tile_size - center_square_size) // 2,
                                     upper_left[1] + (tile_size - center_square_size) // 2)
                center_lower_right = (center_upper_left[0] + center_square_size,
                                      center_upper_left[1] + center_square_size)
                draw.rectangle([center_upper_left, center_lower_right], fill='brown')

    # Save the image
    image.save(output_image_path)

# Example usage
text_file_path = '/mnt/data/example_map.txt'  # Replace with your actual text file path
output_image_path = '/mnt/data/generated_map.png'
create_map_from_text(text_file_path, output_image_path)
output_image_path
