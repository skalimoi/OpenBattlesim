extends DirectionalLight3D

var shader
var current_time
var current_sunrise
var current_sunset

# Called when the node enters the scene tree for the first time.
func _ready():
	shader = $"../CanvasLayer/ColorRect".material


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	current_time = shader.get_shader_parameter("current_time")
	current_sunrise = shader.get_shader_parameter("sunrise_time")
	current_sunset = shader.get_shader_parameter("sunset_time")
	var declination = shader.get_shader_parameter("declination")
	var latitude = shader.get_shader_parameter("latitude")
	var hour = (current_time * 24)
	var solar_hour_angle = 15 * (hour - 12)
	print("HOUR ANGLE: %d", solar_hour_angle)
	
	var altitude_angle_sin = (cos(deg_to_rad(solar_hour_angle)) * cos(deg_to_rad(declination)) * cos(deg_to_rad(latitude))) + (sin(deg_to_rad(declination)) * sin(deg_to_rad(latitude)))
	var altitude_angle = rad_to_deg(asin(altitude_angle_sin))
	print("ALTITUDE ANGLE: %d", [altitude_angle])
	
	$".".rotation_degrees.x = (current_time - current_sunrise) * -550
	$".".rotation_degrees.y = altitude_angle
	
	print("HOUR: %d", [hour])
