extends Node

func _ready():
	if $"../MTerrain/MGrass".grass_is_ready and $"../MTerrain".ready:
		var mask = Image.load_from_file("res://data/vegetation_data/height_map/ArcticGrass_equalized.png")
		for x in 8192:
			for y in 8192:
				var is_pixel: bool
				if mask.get_pixel(x, y).get_luminance() != 0:
					is_pixel = true
				else:
					is_pixel = false
				$"../MTerrain/MGrass".set_grass_by_pixel(x, y, is_pixel)
	$"../MTerrain".restart_grid()
