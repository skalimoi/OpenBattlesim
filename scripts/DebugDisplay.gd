extends CanvasLayer

@onready var data_node = $ColorRect.material
var time_raw: float
var hour
var minutes
var day
var sunrise
var sunset


func _process(delta):
	time_raw = data_node.get_shader_parameter("current_time") * 24
	sunrise = int(data_node.get_shader_parameter("sunrise_time") * 24)
	sunset = int(data_node.get_shader_parameter("sunset_time") * 24)
	day = $ColorRect.day
	hour = int(time_raw)
	minutes = (time_raw - hour) * 60
	$FPS.text = "[right][font=data/fonts/ProggyVector-Regular.ttf][font_size=15]FPS: %d" % Engine.get_frames_per_second()
	$TimeDisplay.text = "[right][font=data/fonts/ProggyVector-Regular.ttf][font_size=15]Time: %d:%d" % [hour, minutes]
	$Day.text = "[right][font=data/fonts/ProggyVector-Regular.ttf][font_size=15]Day: %d" % day
	$SunriseTime.text = "[right][font=data/fonts/ProggyVector-Regular.ttf][font_size=15]Sunrise: %d" % sunrise
	$SunsetTime.text = "[right][font=data/fonts/ProggyVector-Regular.ttf][font_size=15]Sunset: %d" % sunset
