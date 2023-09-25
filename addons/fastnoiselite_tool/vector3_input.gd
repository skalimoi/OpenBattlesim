class_name Vector3Input
extends MarginContainer


signal value_changed(value: Vector3)

@onready var x_input: SpinBox = %OffsetXInput
@onready var y_input: SpinBox = %OffsetYInput
@onready var z_input: SpinBox = %OffsetZInput

@export var min_value: int = -10000
@export var max_value: int = 10000

@export var value: Vector3 = Vector3.ZERO
@export var disabled: bool = false


func randomize_values() -> void:
	var rng := RandomNumberGenerator.new()
	rng.randomize()
	x_input.value = rng.randi_range(min_value, max_value)
	y_input.value = rng.randi_range(min_value, max_value)
	z_input.value = rng.randi_range(min_value, max_value)

func set_values(x: int, y: int, z: int) -> void:
	x_input.value = x
	y_input.value = y
	z_input.value = z

func set_vector3(vector: Vector3) -> void:
	x_input.value = vector.x
	y_input.value = vector.y
	z_input.value = vector.z

func disable() -> void:
	x_input.editable = false
	y_input.editable = false
	z_input.editable = false

func enable() -> void:
	x_input.editable = true
	y_input.editable = true
	z_input.editable = true

func _on_offset_x_input_value_changed(new_value: float):
	value.x = new_value
	x_input.value = new_value
	value_changed.emit(value)

func _on_offset_y_input_value_changed(new_value: float):
	value.y = new_value
	y_input.value = new_value
	value_changed.emit(value)

func _on_offset_z_input_value_changed(new_value: float):
	value.z = new_value
	z_input.value = new_value
	value_changed.emit(value)
