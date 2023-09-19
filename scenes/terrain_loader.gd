extends MTerrain

func _ready():
	self.dataDir = "res://data/terrain_imported/"
	self.save_generated_normals = true
#	$"../ErosionActor".erode_heightmap(10, 1345)
#	$"../TerrainTextureActor".create_texture()
	self.create_grid()
	var heightmap = MRaw16.get_image("res://data/raw/m_terrain_heightmap_eroded.r16", 8193, 8193, -100.0, 1000.0, false)
	for x in 8193:
		for y in 8193:
			var height = heightmap.get_pixel(x, y).r
			self.set_height_by_pixel(x, y, height)
	print("Saving height images.")
	self.save_all_dirty_images()
	
	
func _process(delta):
	pass
