extends MTerrain

func _ready():
	self.dataDir = "res://data/terrain/"
	self.layersDataDir = "res://data/terrain_layer_data/"
	self.terrain_size = Vector2i(1024, 1024)
	self.material.set_shader_parameter("region_size", 64)
	self.start()
	self.create_grid()
	
func _process(delta):
	self.update()
	self.update_physics()
