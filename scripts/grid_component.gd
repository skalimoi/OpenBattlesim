extends GridComponent

var humidity_arr: PackedFloat32Array
var wind_arr: PackedVector3Array
var td_arr: PackedFloat32Array
var pressure_arr: PackedFloat32Array
var temperature_arr: PackedFloat32Array

func _ready():
	humidity_arr = self.load_float_data(0)
	wind_arr = self.load_vector3_data(0)
	td_arr = self.load_float_data(1)
	pressure_arr = self.load_float_data(2)
	temperature_arr = self.load_float_data(3)
	self.monitoring = true
	self.get_overlapping_bodies()
	connect("body_entered", _on_GridComponent_body_entered)

func _process(delta):
	self.set_data(Simulation.total_hour, humidity_arr, wind_arr, td_arr, pressure_arr, temperature_arr)

func _on_GridComponent_body_entered(body: RigidBody3D):
	print("Body intercepted!")
	var wind: Vector3 = (self.wind_p * 100)
	body.apply_central_force(wind)
