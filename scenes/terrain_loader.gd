extends MTerrain

func _ready():
	
	self.dataDir = "res://data/terrain_imported/"
	self.layersDataDir = "res://data/terrain_layer_data/"
	self.save_generated_normals = true
	$"../ErosionActor".erode_heightmap(10, 1345)
	$"../TerrainTextureActor".create_texture("BWK")
	self.create_grid()
	var heightmap = MRaw16.get_image("res://data/raw/m_terrain_heightmap_eroded.r16", 8193, 8193, 0.0, 1000.0, false)
	var texture = Image.load_from_file("res://data/raw/texture.png")

	for x in 8193:
		for y in 8193:
			var height = heightmap.get_pixel(x, y).r
			var color = texture.get_pixel(x, y)
			self.set_height_by_pixel(x, y, height)
			set_pixel(x, y, color, 0)
	self.save_all_dirty_images()
	self.restart_grid()
	
func _process(delta):
	pass
