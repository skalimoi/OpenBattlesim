extends Node3D

const HTerrain = preload("res://addons/zylann.hterrain/hterrain.gd")
const HTerrainData = preload("res://addons/zylann.hterrain/hterrain_data.gd")

var chunk_dim = 510.0; # two lesser for seamless tiling

# Called when the node enters the scene tree for the first time.
func _ready():
	for x in 16:
		for y in 16:
			var data = HTerrainData.new()
			@warning_ignore("unused_variable")
			data.resize(513)
			data._import_heightmap("res://tile_tex/lvl1/height_%d_%d.png" % [x, y], 0.0, 300.0, true)
			@warning_ignore("unused_variable")
			var heightmap: Image = data.get_image(HTerrainData.CHANNEL_HEIGHT)
			data._import_map(1, "res://tile_tex/lvl1/normal_%d_%d.png" % [x, y])
			data._import_map(3, "res://tile_tex/lvl1/tex_%d_%d.png" % [x, y])
			@warning_ignore("unused_variable")
			var normalmap: Image = data.get_image(HTerrainData.CHANNEL_NORMAL)
			var terrain = HTerrain.new()
			terrain.set_data(data)
			var format_string = "tile_%d_%d"
			terrain.name = format_string % [x, y]
			var modified_region = Rect2(Vector2(), heightmap.get_size())
			terrain.set_shader_type(HTerrain.SHADER_CLASSIC4_LITE)
			terrain.set_chunk_size(128)
			terrain.set_collision_enabled(true)
			data.notify_region_change(modified_region, HTerrainData.CHANNEL_HEIGHT)
			add_child(terrain)
			terrain.global_position = Vector3(x * chunk_dim, 0, y * chunk_dim)
			


# Called every frame. 'delta' is the elapsed time since the previous frame.
@warning_ignore("unused_parameter")
func _process(delta):
	pass
