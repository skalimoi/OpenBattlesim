extends Node3D

@onready var camera = $Camera3D
@onready var stream = VoxelStreamMemory.new()
@onready var generator = VoxelGeneratorFlat.new()
@onready var mesher = VoxelMesherTransvoxel.new()
@onready var t : VoxelLodTerrain = VoxelLodTerrain.new()
@onready var mat : ShaderMaterial = preload("res://data/debug/new_shader_material.tres")
#@onready var TerrainGenerator = preload("res://data/debug/generator.gd")

var selected_scenario = []
var selected_vegetation = []
var scenarios : PackedStringArray = []
var vegetation : PackedStringArray = []


func init_chunks():
	generator.channel = VoxelBuffer.CHANNEL_SDF
	t.material = mat
	t.name = "TerrainBox"
	t.view_distance = 65535
	t.voxel_bounds.position = Vector3i(0, 0, 0)
	t.voxel_bounds.size = Vector3i(65535, 8192, 65535)
	t.lod_count = 8
	t.lod_distance = 48
	t.secondary_lod_distance = 48
	t.gi_mode = GeometryInstance3D.GI_MODE_DYNAMIC
	t.generate_collisions = true
	t.threaded_update_enabled = true
	#t.normalmap_enabled = true
	t.position = Vector3(0, 0, 0)
	self.add_child(t)
	Global.node_database[0] = t

func _ready() -> void:
	ThreadPoolSingleton.task_finished.connect(_on_finished_init)
	ThreadPoolSingleton.discard_finished_tasks = false
	init_chunks()
	var io: Object = ImGui.GetIO()
	io.ConfigFlags |= ImGui.ConfigFlags_ViewportsEnable

func _on_finished_init(task_tag) -> void:
	pass

func get_test_scenarios():
	var dir = DirAccess.open("res://data/debug/test_scenes")
	if dir:
		scenarios = dir.get_directories()
		
func load_terrain():
	var terrain_graph : VoxelGeneratorGraph = load("res://data/debug/graph.tres")
	var soil_map : Dictionary = ChunkLoader.get_master_soils(scenarios[selected_scenario[0]], true)
	var values = soil_map.keys()
	values.sort()
	values.reverse()
	var no_data : Image = Image.create_empty(8192, 8192, false, Image.FORMAT_RGB8)
	no_data.fill(Color.BLACK)
	for value : int in values:
		print(value)
		var soil : Image = ChunkLoader.get_soil_data(scenarios[selected_scenario[0]], value, true)
		var pos : int = values.find(value, 0)
		print("pos: %d" % pos)
		var image_node = terrain_graph.get_main_function().find_node_by_name("Soil%d" % pos)
		terrain_graph.get_main_function().set_node_param(image_node, 0, soil)
	if (16 - values.size()) != 0:
		for position in range(values.size(), 16, 1):
			print("empty position: %d" % position)
			var image_node = terrain_graph.get_main_function().find_node_by_name("Soil%d" % position)
			terrain_graph.get_main_function().set_node_param(image_node, 0, no_data)
	
	var image : Image = ChunkLoader.get_height_data(scenarios[selected_scenario[0]], true)
	var data_node = terrain_graph.get_main_function().find_node_by_name("DataNode")
	terrain_graph.get_main_function().set_node_param(data_node, 0, image)
	terrain_graph.compile()
	#var tool : VoxelTool = t.get_voxel_tool()
	t.set_deferred("generator", terrain_graph)
	t.set_deferred("stream", stream)
	mesher.texturing_mode = VoxelMesherTransvoxel.TEXTURES_BLEND_4_OVER_16
	t.set_deferred("mesher", mesher)



func _process(delta: float) -> void:
	if Engine.has_singleton("ImGuiAPI"):
		get_test_scenarios()
		ImGui.Begin("Debugger", [], ImGui.WindowFlags_AlwaysAutoResize)
		var pos: Array = [camera.position.x, camera.position.y, camera.position.z]
		if ImGui.DragInt3("position", pos):
			position = Vector3(pos[0], pos[1], pos[2])
		ImGui.ListBox("Available scenarios", selected_scenario, scenarios, scenarios.size(), 4)
		if ImGui.Button("Load terrain"):
			ThreadPoolSingleton.submit_task_unparameterized(self, "load_terrain")
		var voxeldata = "Click a voxel to query for its index."
		if Global.voxel_pointed != null:
			voxeldata = "Voxel index: %v" % Global.voxel_pointed
		else:
			voxeldata = "No voxel found."
		ImGui.Text(voxeldata)
		ImGui.End()
