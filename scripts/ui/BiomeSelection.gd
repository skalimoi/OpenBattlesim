extends OptionButton
# TODO. Not needed for now
func get_directories():
	var climates = DirAccess.get_directories_at("res://data/climate_sat_data/")

func _ready():
	pass
