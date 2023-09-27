extends Button

func _on_pressed():
	get_tree().change_scene_to_file("res://scenes/generation_scene.tscn")

func _ready():
	self.pressed.connect(_on_pressed)
