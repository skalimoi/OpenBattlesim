extends MTerrain

func _ready():
	self.dataDir = "res://data/terrain/"
	self.layersDataDir = "res://data/terrain_layer_data/"
	self.terrain_size = Vector2i(1024, 1024)
	
	var heightmap = Image.load_from_file("res://data/raw/eroded.png")
	
	self.material.set_shader_parameter("m_terrain_heightmap", heightmap)
	self.material.set_shader_parameter("region_size", 64)
	self.start()
	
func _process(delta):
	self.update()
	self.update_physics()
