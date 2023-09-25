extends MTerrain

#MBrushDecal
#MPaintPanel
#MTools
#MBrushLayers
#MBrushManager
#MChunkGenerator
#MChunks
#MCollision

func _ready():
	
	self.dataDir = "res://data/terrain_imported/"
	self.layersDataDir = "res://data/terrain_layer_data/"
	self.save_generated_normals = true
#	$"../ErosionActor".erode_heightmap(300, 1345)
#	$"../TerrainTextureActor".create_texture("CFA")
	self.create_grid()
	
	var heightmap = MRaw16.get_image("res://data/raw/m_terrain_heightmap_eroded.r16", 8193, 8193, 0.0, 2000.0, false)
	for x in 8193:
		for y in 8193:
			var height = heightmap.get_pixel(x, y).r
			self.set_height_by_pixel(x, y, height)
	
#	var texture = Image.load_from_file("res://data/raw/texture.png")
#	for x in 8193:
#		for y in 8193:
#			var color = texture.get_pixel(x, y)
#			self.set_pixel(x, y, color, 0)

#	var water_mask = Image.load_from_file("res://data/raw/discharge.png")
#	for x in 8193:
#		for y in 8193:
#			var water = water_mask.get_pixel(x, y)
#			self.set_pixel(x, y, water, 1)

	self.save_all_dirty_images()
	print("Saving images...")
	self.restart_grid()
	
func _process(delta):
	pass
