extends Node3D

@onready var camera = $Camera3D
@onready var stream = VoxelStreamMemory.new()
@onready var generator = VoxelGeneratorFlat.new()
@onready var mesher = VoxelMesherTransvoxel.new()
@onready var terrain_node = Node.new()
@onready var terrain_generator = VoxelGeneratorScript.new()

var selected_scenario = []


func init_chunks():
	generator.channel = VoxelBuffer.CHANNEL_SDF
	terrain_node.name = "TerrainBase"
	self.call_deferred("add_child", terrain_node)
	for z in 8:
		for x in 8:
			var t: VoxelLodTerrain = VoxelLodTerrain.new()
			t.view_distance = 65535
			t.voxel_bounds.position = Vector3i(0, 0, 0)
			t.voxel_bounds.size = Vector3i(8192, 4096, 8192)
			t.lod_count = 8
			t.lod_distance = 48
			t.secondary_lod_distance = 48
			t.generate_collisions = true
			t.threaded_update_enabled = true
			t.stream = stream
			t.generator = generator
			t.mesher = mesher
			t.position = Vector3(8192 * x, 0, 8192 * z)
			terrain_node.call_deferred("add_child", t)

func _ready() -> void:
	ThreadPoolSingleton.task_finished.connect(_on_finished_init)
	ThreadPoolSingleton.discard_finished_tasks = false
	ThreadPoolSingleton.submit_task_unparameterized(self, "init_chunks")
	var io: Object = ImGui.GetIO()
	io.ConfigFlags |= ImGui.ConfigFlags_ViewportsEnable

func _on_finished_init(task_tag) -> void:
	scrnOutput.print("Chunks loaded.")

func get_test_scenarios() -> PackedStringArray:
	var dir = DirAccess.open("res://data/debug/test_scenes")
	if dir:
		return dir.get_directories()
	else:
		return []
		
func load_terrain():
	for child in self.terrain_node.get_children(false):
		child.generator = terrain_generator
		# TODO set script for generator

func _process(delta: float) -> void:
	if Engine.has_singleton("ImGuiAPI"):
		var scenarios = get_test_scenarios()
		ImGui.Begin("Debugger", [], ImGui.WindowFlags_AlwaysAutoResize)
		var pos: Array = [camera.position.x, camera.position.y, camera.position.z]
		if ImGui.DragInt3("position", pos):
			position = Vector3(pos[0], pos[1], pos[2])
		ImGui.ListBox("Available scenarios", selected_scenario, scenarios, scenarios.size(), 4)
		if ImGui.Button("Load terrain"):
			
		ImGui.End()
