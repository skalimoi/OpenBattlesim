extends MTerrain

func _ready():
	self.dataDir = "res://data/terrain_imported/"
	$"../ErosionActor".erode_heightmap(5, 1345)
#	$"../TerrainTextureActor".create_texture()
	self.create_grid()
	var file = FileAccess.open("res://data/raw/m_terrain_heightmap_eroded.r16", FileAccess.READ)
	var content = file.get_file_as_bytes("res://data/raw/m_terrain_heightmap_eroded.r16")
	var place: int = 0
	for x in 8193:
		for y in 8193:
			var height = content[place]
			self.set_height_by_pixel(x, y, height)
			place += 1
	print("Saving height images.")
	self.save_all_dirty_images()
	self.create_grid()
	
func _process(delta):
	pass
