extends Node3D

@onready var camera = $Camera3D
@onready var stream = VoxelStreamMemory.new()
@onready var generator = VoxelGeneratorFlat.new()
@onready var mesher = VoxelMesherTransvoxel.new()
@onready var t : VoxelLodTerrain = VoxelLodTerrain.new()
#@onready var TerrainGenerator = preload("res://data/debug/generator.gd")

var selected_scenario = []
var scenarios : PackedStringArray = []


func init_chunks():
	generator.channel = VoxelBuffer.CHANNEL_SDF
	t.name = "TerrainBox"
	t.view_distance = 65535
	t.voxel_bounds.position = Vector3i(0, 0, 0)
	t.voxel_bounds.size = Vector3i(65535, 4096, 65535)
	t.lod_count = 8
	t.lod_distance = 48
	t.secondary_lod_distance = 48
	t.generate_collisions = true
	t.threaded_update_enabled = true
	t.normalmap_enabled = true
	t.position = Vector3(0, 0, 0)
	self.call_deferred("add_child", t)

func _ready() -> void:
	ThreadPoolSingleton.task_finished.connect(_on_finished_init)
	ThreadPoolSingleton.discard_finished_tasks = false
	ThreadPoolSingleton.submit_task_unparameterized(self, "init_chunks")
	var io: Object = ImGui.GetIO()
	io.ConfigFlags |= ImGui.ConfigFlags_ViewportsEnable

func _on_finished_init(task_tag) -> void:
	scrnOutput.print("Chunks loaded.")

func get_test_scenarios():
	var dir = DirAccess.open("res://data/debug/test_scenes")
	if dir:
		scenarios = dir.get_directories()
		
func load_terrain():
	var image : Image = ChunkLoader.get_height_data(scenarios[selected_scenario[0]], true)
	var terrain_graph : VoxelGeneratorGraph = load("res://data/debug/graph.tres")
	var data_node = terrain_graph.get_main_function().find_node_by_name("DataNode")
	terrain_graph.get_main_function().set_node_param(data_node, 0, image)
	print(terrain_graph.get_main_function().get_node_param(data_node, 0))
	terrain_graph.compile()
	t.set_deferred("generator", terrain_graph)
	t.set_deferred("stream", stream)
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
		ImGui.End()
