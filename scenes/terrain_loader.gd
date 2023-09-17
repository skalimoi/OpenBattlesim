extends MTerrain

func _ready():
#	$"../ErosionActor".erode_heightmap(200, 1345)
	$"../TerrainTextureActor".create_texture()
func _process(delta):
	pass
