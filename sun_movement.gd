extends DirectionalLight3D

var shader
var current_time
var current_sunrise
var current_sunset

func lerp_t(a, b, t):
	return a + (b - a) * t

# Called when the node enters the scene tree for the first time.
func _ready():
	shader = $"../CanvasLayer/ColorRect".material


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	current_time = shader.get_shader_parameter("current_time")
	current_sunrise = shader.get_shader_parameter("sunrise_time")
	current_sunset = shader.get_shader_parameter("sunset_time")
	var declination = shader.get_shader_parameter("declination")
	print("DECLINATION: %d", [declination])
	$".".rotation_degrees.x = (current_time - current_sunrise) * (-360)
	$".".rotation_degrees.y = declination
