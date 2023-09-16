extends MTerrain

func _ready():
	self.dataDir = "res://data/terrain_imported/"
	self.save_generated_normals = true
	self.create_grid()
	self.start()
	
	
func _process(delta):
	self.update()
	self.update_physics()
