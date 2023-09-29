extends Control

var vegetation_actor = VegetationGen.new()
var erosion_actor = ErosionActor.new()
var texture_actor = TerrainTextureActor.new()
var terrain_gen = MTerrain.new()
var erosion_thread: Thread
@onready var text = self.get_node("RichTextLabel")
@onready var enter_button: Button = self.get_node("Button")
var is_finished: bool

func debug_generate_veg_maps():
	vegetation_actor.generate_yaml_map("PolarZone", 0.2, "data/raw/height_map_veg.png", 300.0, 8, "data/raw/soil_map.png")
	vegetation_actor.example_function("height_map", "ArcticGrass")

func config_terrain_gen():
	terrain_gen.terrain_size = Vector2i(256,256)
	terrain_gen.region_size = 32
	terrain_gen.dataDir = "res://data/terrain_imported/"

func erode_and_texture():
	is_finished = false
#	erosion_actor.erode_heightmap(UiGenData.erosion_cycles, UiGenData.seed)
#	text.call_deferred("set", "text", "Texturing terrain...")
	debug_generate_veg_maps()
	texture_actor.create_texture(UiGenData.biome)
	text.call_deferred("set", "text", "Generating weather data...")
		# grid
	var grid_size = Vector3i(16, 6, 16)
	var box_size = Vector3(512, 256, 512)
	config_terrain_gen()
	terrain_gen.save_generated_normals = true
	self.call_deferred("add_child", terrain_gen)
	terrain_gen.create_grid()
	var heightmap = MRaw16.get_image("res://data/raw/m_terrain_heightmap_eroded.r16", 8193, 8193, 0.0, 1000.0, false)
	for x in 8193:
		for y in 8193:
			var height = heightmap.get_pixel(x, y).r
			terrain_gen.set_height_by_pixel(x, y, height)
	terrain_gen.save_all_dirty_images()
	var counter = 0
	for x in (grid_size.x):
		for y in (grid_size.y):
			for z in (grid_size.z):
				counter += 1
				var area = GridComponent.new()
				area.name = "grid_{0}_{1}_{2}".format([x, y, z])
				area.index = Vector3i(x, y, z)
				var shape = CollisionShape3D.new()
				var box = BoxShape3D.new()
				shape.name = "shape_{0}_{1}_{2}".format([x, y, z])
				box.size = box_size
				shape.set_shape(box)
				self.add_child(area)
				area.add_child(shape)
				area.global_position = Vector3((512 * x) + 256, (256 * y) + 128, (512 * z) + 256)
				if y == 0:
					var pos = area.global_position
					var point_1 = terrain_gen.get_closest_height(area.global_position)
					var point_2 = terrain_gen.get_closest_height(Vector3(area.global_position.x + 256, area.global_position.y, area.global_position.z + 256))
					var point_3 = terrain_gen.get_closest_height(Vector3(area.global_position.x + 256, area.global_position.y, area.global_position.z))
					var point_4 = terrain_gen.get_closest_height(Vector3(area.global_position.x - 256, area.global_position.y, area.global_position.z))
					var point_5 = terrain_gen.get_closest_height(Vector3(area.global_position.x, area.global_position.y, area.global_position.z - 256))
					var point_6 = terrain_gen.get_closest_height(Vector3(area.global_position.x - 256, area.global_position.y, area.global_position.z - 256))
					var mean = (point_1 + point_2 + point_3 + point_4 + point_5 + point_6) / 6
					area.mean_altitude = mean
				else:
					var pos = area.global_position.y
					area.mean_altitude = pos
				area.generate_data(40, UiGenData.biome)
	prepare_rivers()
	prepare_tex()
	
	is_finished = true
	
func _ready():
	self.add_child(erosion_actor)
	erosion_thread = Thread.new()
	erosion_actor.path_to_heightmap = UiGenData.heightmap_path
	erosion_thread.start(erode_and_texture)
	
	
	if is_finished == true:
		erosion_thread.wait_to_finish()
		text.set("text", "Completed.")
		enter_button.set("visible", true)

func prepare_tex():
	var import_w: Window = self.get_node("Window")
	# texture
	import_w.set("file_path", "res://data/raw/texture.png")
	import_w.set("ext", "png")
	import_w.set("save_path", "res://data/terrain_imported/")
	import_w.set("region_size", 1025)
	import_w.set("unifrom_name", "albedo_tex")
	import_w.set("width", 8193)
	import_w.set("height", 8193)
	import_w.set("image_format", 4)
	import_w.set("is_heightmap", false)
	import_w.set("flip_x", false)
	import_w.set("flip_y", false)
	import_w._on_import_pressed()

func prepare_rivers():
	# this is just a copy-paste of the import window from the MTerrain plugin.
	var import_w: Window = self.get_node("Window")
	# water
	import_w.set("file_path", "res://data/raw/discharge.png")
	import_w.set("ext", "png")
	import_w.set("save_path", "res://data/terrain_imported/")
	import_w.set("region_size", 1025)
	import_w.set("unifrom_name", "water_mask")
	import_w.set("width", 8193)
	import_w.set("height", 8193)
#	import_w.set("image_format", 1)
	import_w.set("is_heightmap", false)
	import_w.set("flip_x", false)
	import_w.set("flip_y", false)
	import_w.set("temp_path", "res://tmp/")
	import_w._on_import_pressed()
