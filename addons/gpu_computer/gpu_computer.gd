class_name GPUComputer

extends Node

signal compute_list_submitted
signal compute_list_synced

## Place your shader file (*.glsl) here. As of Godot 4.1 the file must be created in an external text editor and then added to the "res://" directory.
@export var shader_file: RDShaderFile

var submitted := false

var shader_uniforms := Dictionary()
var shader_buffers := Dictionary()
var uniform_sets := Dictionary()
var shader: RID
var pipeline: RID
var compute_list
var rd := RenderingServer.create_local_rendering_device()

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

func _load_shader() -> void:
	var shader_spirv: RDShaderSPIRV = shader_file.get_spirv()
	shader = rd.shader_create_from_spirv(shader_spirv)

func _add_buffer(_set: int,
			binding: int,
			data: PackedByteArray,
			usage := RenderingDevice.STORAGE_BUFFER_USAGE_DISPATCH_INDIRECT,
		) -> void:
	var set_and_binding := Vector2i(_set, binding)
	shader_buffers[set_and_binding] = rd.storage_buffer_create(data.size(), data, usage)

func _add_uniform(_uniform_type: RenderingDevice.UniformType, _set: int, _binding: int, buffer: RID) -> void:
	var new_uniform := RDUniform.new()
	new_uniform.uniform_type = _uniform_type
	new_uniform.binding = _binding
	new_uniform.add_id(buffer)
	if not shader_uniforms.has(_set):
		shader_uniforms[_set] = [new_uniform]
	else:
		shader_uniforms[_set].append(new_uniform)

func _add_uniform_set(_set: int) -> void:
	var new_uniform_set := rd.uniform_set_create(shader_uniforms[_set], shader, _set)
	uniform_sets[_set] = new_uniform_set

func _make_pipeline(workgroup_size: Vector3i, new_pipeline: bool = false) -> void:
	if new_pipeline:
		for set_and_binding in shader_buffers.keys():
			_add_uniform(RenderingDevice.UNIFORM_TYPE_STORAGE_BUFFER,
					set_and_binding.x,
					set_and_binding.y,
					shader_buffers[set_and_binding],)
		for _set in shader_uniforms.keys():
			_add_uniform_set(_set)
		pipeline = rd.compute_pipeline_create(shader)
	compute_list = rd.compute_list_begin()
	rd.compute_list_bind_compute_pipeline(compute_list, pipeline)
	for _set in uniform_sets.keys():
		rd.compute_list_bind_uniform_set(compute_list, uniform_sets[_set], _set)
	rd.compute_list_dispatch(compute_list, workgroup_size.x, workgroup_size.y, workgroup_size.z)
	rd.compute_list_end()

func _submit() -> void:
	rd.submit()
	submitted = true
	emit_signal("compute_list_submitted")

func _sync() -> void:
	rd.sync()
	submitted = false
	emit_signal("compute_list_synced")

func output(_set: int, binding: int, offset_bytes: int = 0, size_bytes: int = 0, free_RID: bool = false) -> PackedByteArray:
	var set_and_binding := Vector2i(_set, binding)
	var requested_buffer: RID = shader_buffers[set_and_binding]
	var requested_output = rd.buffer_get_data(requested_buffer, offset_bytes, size_bytes)
	if free_RID:
		rd.free_rid(requested_buffer)
	return requested_output

func _update_buffer(new_byte_array: PackedByteArray,
			_set: int,
			binding: int,
			offset: int = 0,
			post_barrier: RenderingDevice.BarrierMask = RenderingDevice.BARRIER_MASK_ALL_BARRIERS,
		) -> void:
	var set_and_binding := Vector2i(_set, binding)
	var requested_buffer: RID = shader_buffers[set_and_binding]
	rd.buffer_update(requested_buffer, offset, new_byte_array.size(), new_byte_array, post_barrier)

func _free_rid(_set: int, binding: int) -> void:
	var set_and_binding := Vector2i(_set, binding)
	var requested_buffer: RID = shader_buffers[set_and_binding]
	rd.free_rid(requested_buffer)

func _exit_tree():
	for set_and_binding in shader_buffers.keys():
		rd.free_rid(shader_buffers[set_and_binding])
