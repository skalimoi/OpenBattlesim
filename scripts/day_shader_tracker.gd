extends ColorRect

const HOURS_PER_DAY := 24.0
const MINUTES_PER_HOUR := 60.0

var eot_values : Dictionary
var daytime_shader : ShaderMaterial
var play_day_shader : bool
var playback_value : float
var playback_duration : float
var current_time : float
@export var day: float
var accumulated_elapsed_time: float

@onready var eot_node = $"../../CustomEOT"

# Called when the node enters the scene tree for the first time.
func _ready():
	daytime_shader = $".".material
	current_time = 0
	day = 0
	accumulated_elapsed_time = 0

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	var day_duration = 60
	
	if accumulated_elapsed_time < day_duration:
		accumulated_elapsed_time += (delta)
		if accumulated_elapsed_time > day_duration:
			accumulated_elapsed_time = 0
			day += 1
	
	var day_progress = (accumulated_elapsed_time / day_duration)
	var viewport = $"../../Camera3D".get_viewport()
	if viewport.size_changed:
		$".".set_size(viewport.get_visible_rect().size, false)
	eot_values = eot_node.get_full_info_about_day(day)
	if daytime_shader:
		daytime_shader.set_shader_parameter("sunrise_time", eot_values["sunrise_pct"])
		daytime_shader.set_shader_parameter("sunset_time", eot_values["sunset_pct"])
		daytime_shader.set_shader_parameter("twilight_duration", eot_values["twilight_duration_pct"])
		daytime_shader.set_shader_parameter("current_time", day_progress)
		daytime_shader.set_shader_parameter("declination", eot_values["declination"])
		daytime_shader.set_shader_parameter("latitude", eot_values["latitude"])
#		print(daytime_shader.get_shader_parameter("current_time"))
	
	
