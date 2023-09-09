extends Node
var rd := RenderingServer.create_local_rendering_device()
var shader_file := load("res://boids/boid_compute.glsl")
var shader_spirv: RDShaderSPIRV = shader_file.get_spirv()
var shader := rd.shader_create_from_spirv(shader_spirv)
var boids_vel : PackedVector3Array
var boids_pos : PackedVector3Array
var boid_number = 100
var pipeline : RID
var vel_output : Array

var material_physics = PhysicsMaterial.new()

var mat = Material.new()

# Called when the node enters the scene tree for the first time.
func _ready():
	pipeline = rd.compute_pipeline_create(shader)
	
	# boid creation
	for i in boid_number:
		var shape = CollisionShape3D.new()
		var sph = CSGSphere3D.new()
		shape.shape = SphereShape3D.new()
		shape.shape.set_radius(10.0)
		sph.set_material(mat)
		sph.set_radius(10)
		var rb = RigidBody3D.new()
		$"../BoidBase".add_child(rb)
		# await rb.tree_entered
		rb.physics_material_override = material_physics
		rb.gravity_scale = 0
		rb.add_child(shape)
		# await shape.tree_entered
		rb.add_child(sph)
		# await sph.tree_entered
		var pos = Vector3(randf_range(10.0, 1500.0), randf_range(10.0, 1500.0), randf_range(10.0, 1500.0))
		
		print("CREATING BOID NUMBER ", [i])
		rb.global_position = pos
		
		
		boids_pos.append(rb.global_position)
		boids_vel.append(rb.linear_velocity)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	var vel_byte_array = boids_vel.to_byte_array()
	var pos_byte_array = boids_pos.to_byte_array()
	var vel_storage_buffer = rd.storage_buffer_create(vel_byte_array.size(), vel_byte_array)
	var pos_storage_buffer = rd.storage_buffer_create(pos_byte_array.size(), pos_byte_array)
	var num_storage_buffer = rd.storage_buffer_create([boid_number].size(), [boid_number])
	
	var u_vel = RDUniform.new()
	u_vel.uniform_type = RenderingDevice.UNIFORM_TYPE_STORAGE_BUFFER
	u_vel.add_id(vel_storage_buffer)
	u_vel.binding = 0
	
	var u_pos = RDUniform.new()
	u_pos.uniform_type = RenderingDevice.UNIFORM_TYPE_STORAGE_BUFFER
	u_pos.add_id(pos_storage_buffer)
	u_pos.binding = 1
	
	var u_num = RDUniform.new()
	u_num.uniform_type = RenderingDevice.UNIFORM_TYPE_STORAGE_BUFFER
	u_num.add_id(num_storage_buffer)
	u_num.binding = 2
	
	var bindings = [u_vel, u_pos, u_num]
	
	var uniform_set = rd.uniform_set_create(bindings, shader, 0)

	var compute_list = rd.compute_list_begin()
	rd.compute_list_bind_compute_pipeline(compute_list, pipeline)
	rd.compute_list_bind_uniform_set(compute_list, uniform_set, 0)
	rd.compute_list_dispatch(compute_list, 10, 1, 1)
	rd.compute_list_end()
	rd.submit()
	rd.sync()
	
	var vel_data = rd.buffer_get_data(vel_storage_buffer).to_float32_array()
	var i = 0
	for boid in $"../BoidBase".get_children():
		# check if indexes okay
		boid.linear_velocity = (Vector3(vel_data[i * 3],vel_data[i * 3 + 1], vel_data[i * 3 + 2]))
		i += 1
#	for value in vel_data:
#		print(value)
