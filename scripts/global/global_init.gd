extends Node

const Level = Tracer.Level

# Called when the node enters the scene tree for the first time.
func _ready():
	var subscriber = (
		TraceSubscriber
		. new()
		. with_colored_output(true)
		. with_level(true)
		. with_nicer_colors(true)
		. with_timestamp(true)
		. with_filter(Level.Info | Level.Warn | Level.Error | Level.Debug)
	)
	# Initialize the subscriber
	subscriber.init()
	var logs = FileAccess.open("res://log.txt", FileAccess.WRITE)
	var file_logger = (
		TraceSubscriber
		. new()
		. barebones()
		. with_writer(
			TraceSubscriber.writer_from_file(logs)
		)
	)
	file_logger.init()
	
	var version = FileAccess.get_file_as_string("res://VERSION.txt")
	Tracer.info("This is [b][i][color=ff0048]Open[/color][/i][/b][i][b]Battlesim[/b][/i] v.%s." % [version])
