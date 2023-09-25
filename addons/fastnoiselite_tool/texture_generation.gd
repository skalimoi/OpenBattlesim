extends Control


@onready var noise_display: TextureRect = %NoiseDisplay

const IMAGE_SIZE := Vector2(512, 512)
const IMAGE_FORMAT := Image.FORMAT_RGB8


func _on_noise_generator_noise_generated(noise):
	var new_image := Image.create(IMAGE_SIZE.x, IMAGE_SIZE.y, false, IMAGE_FORMAT) 

	for x in range(0, IMAGE_SIZE.x):
		for y in range(0, IMAGE_SIZE.y):
			var noise_level = (noise.get_noise_2d(x, y) + 1) / 2
			new_image.set_pixel(x, y, Color(noise_level, noise_level, noise_level, 1))

	noise_display.texture = ImageTexture.create_from_image(new_image)
