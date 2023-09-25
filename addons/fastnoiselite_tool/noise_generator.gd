extends Control

signal noise_generated(noise: FastNoiseLite)

@onready var file_dialog: FileDialog = %FileDialog
@onready var message_output: LineEdit = %MessageOutput

@onready var noise_type_input: OptionButton = %NoiseTypeInput
@onready var seed_input: SpinBox = %SeedInput
@onready var frequency_input: SpinBox = %FrequencyInput
@onready var offset_input: Vector3Input = %OffsetInput

@onready var cellular_distance_function_input: OptionButton = %CellularDistanceFunctionInput
@onready var cellular_return_type_input: OptionButton = %CellularReturnTypeInput
@onready var cellular_jitter_input: SpinBox = %CellularJitterInput

@onready var domain_warp_enabled_input: CheckBox = %DomainWarpEnabledInput
@onready var domain_warp_type_input: OptionButton = %DomainWarpTypeInput
@onready var domain_warp_amplitude_input: SpinBox = %DomainWarpAmplitudeInput
@onready var domain_warp_frequency_input: SpinBox = %DomainWarpFrequencyInput

@onready var domain_warp_fractal_gain_input: SpinBox = %DomainWarpFractalGainInput
@onready var domain_warp_fractal_lacunarity_input: SpinBox = %DomainWarpFractalLacunarityInput
@onready var domain_warp_fractal_octaves_input: SpinBox = %DomainWarpFractalOctavesInput
@onready var domain_warp_fractal_type_input: OptionButton = %DomainWarpFractalTypeInput

@onready var fractal_gain_input: SpinBox = %FractalGainInput
@onready var fractal_lacunarity_input: SpinBox = %FractalLacunarityInput
@onready var fractal_octaves_input: SpinBox = %FractalOctavesInput
@onready var fractal_ping_pong_strength_input: SpinBox = %FractalPingPongStrengthInput
@onready var fractal_type_input: OptionButton = %FractalTypeInput
@onready var fractal_weighted_strength_input: SpinBox = %FractalWeightedStrengthInput


@onready var general_inputs: Array[Dictionary] = [
	{"input": noise_type_input, "lock": %NoiseTypeInputLock},
	{"input": seed_input, "lock": %SeedInputLock},
	{"input": frequency_input, "lock": %FrequencyInputLock},
	{"input": offset_input, "lock": %OffsetInputLock}
]
@onready var cellular_inputs: Array[Dictionary] = [
	{"input": cellular_distance_function_input, "lock": %CellularDistanceFunctionInputLock},
	{"input": cellular_return_type_input, "lock": %CellularReturnTypeInputLock},
	{"input": cellular_jitter_input, "lock": %CellularJitterInputLock},
]
@onready var domain_warp_inputs: Array[Dictionary] = [
	{"input": domain_warp_enabled_input, "lock": %DomainWarpEnabledInputLock},
	{"input": domain_warp_type_input, "lock": %DomainWarpTypeInputLock},
	{"input": domain_warp_amplitude_input, "lock": %DomainWarpAmplitudeInputLock},
	{"input": domain_warp_frequency_input, "lock": %DomainWarpFrequencyInputLock},
]
@onready var domain_warp_fractal_inputs: Array[Dictionary] = [
	{"input": domain_warp_fractal_type_input, "lock": %DomainWarpFractalTypeInputLock},
	{"input": domain_warp_fractal_gain_input, "lock": %DomainWarpFractalGainInputLock},
	{"input": domain_warp_fractal_lacunarity_input, "lock": %DomainWarpFractalLacunarityInputLock},
	{"input": domain_warp_fractal_octaves_input, "lock": %DomainWarpFractalOctavesInputLock},
]
@onready var fractal_inputs: Array[Dictionary] = [
	{"input": fractal_type_input, "lock": %FractalTypeInputLock},
	{"input": fractal_gain_input, "lock": %FractalGainInputLock},
	{"input": fractal_lacunarity_input, "lock": %FractalLacunarityInputLock},
	{"input": fractal_octaves_input, "lock": %FractalOctavesInputLock},
	{"input": fractal_ping_pong_strength_input, "lock": %FractalPingPongStrengthInputLock},
	{"input": fractal_weighted_strength_input, "lock": %FractalWeightedStrengthInputLock},
]

const noise_types: Array = [
	["Cellular", FastNoiseLite.TYPE_CELLULAR],
	["Perlin", FastNoiseLite.TYPE_PERLIN],
	["Simplex", FastNoiseLite.TYPE_SIMPLEX],
	["Simplex Smooth", FastNoiseLite.TYPE_SIMPLEX_SMOOTH],
	["Value", FastNoiseLite.TYPE_VALUE],
	["Value Cubic", FastNoiseLite.TYPE_VALUE_CUBIC]
]
const fractal_types: Array = [
	["None", FastNoiseLite.FRACTAL_NONE],
	["FBM", FastNoiseLite.FRACTAL_FBM],
	["Ridged", FastNoiseLite.FRACTAL_RIDGED],
	["Ping Pong", FastNoiseLite.FRACTAL_PING_PONG],
]
const cellular_distance_functions: Array = [
	["Euclidean", FastNoiseLite.DISTANCE_EUCLIDEAN],
	["Euclidean Squared", FastNoiseLite.DISTANCE_EUCLIDEAN_SQUARED],
	["Manhattan", FastNoiseLite.DISTANCE_MANHATTAN],
	["Hybrid", FastNoiseLite.DISTANCE_HYBRID],
]
const cellular_return_types: Array = [
	["Cell Value", FastNoiseLite.RETURN_CELL_VALUE],
	["Distance", FastNoiseLite.RETURN_DISTANCE],
	["Distance 2", FastNoiseLite.RETURN_DISTANCE2],
	["Distance 2 Add", FastNoiseLite.RETURN_DISTANCE2_ADD],
	["Distance 2 Sub", FastNoiseLite.RETURN_DISTANCE2_SUB],
	["Distance 2 Mul", FastNoiseLite.RETURN_DISTANCE2_MUL],
	["Distance 2 Div", FastNoiseLite.RETURN_DISTANCE2_DIV]
]
const domain_warp_types: Array = [
	["Simplex", FastNoiseLite.DOMAIN_WARP_SIMPLEX],
	["Reduced", FastNoiseLite.DOMAIN_WARP_SIMPLEX_REDUCED],
	["Basic Grid", FastNoiseLite.DOMAIN_WARP_BASIC_GRID],
]
const domain_warp_fractal_types: Array = [
	["None", FastNoiseLite.DOMAIN_WARP_FRACTAL_NONE],
	["Progressive", FastNoiseLite.DOMAIN_WARP_FRACTAL_PROGRESSIVE],
	["Independent", FastNoiseLite.DOMAIN_WARP_FRACTAL_INDEPENDENT],
]

var noise: FastNoiseLite
var noise_resource: NoiseResource

var is_live: bool = false


func _ready():
	init_ui()
	
	noise = FastNoiseLite.new()
	noise_resource = NoiseResource.new()
	
	load_values_from_resource()
	set_noise_values()

func set_noise_values() -> void:
	noise.noise_type = noise_resource.noise_type
	noise.seed = noise_resource.seed
	noise.frequency = noise_resource.frequency
	noise.offset = noise_resource.offset
	
	noise.cellular_distance_function = noise_resource.cellular_distance_function
	noise.cellular_jitter = noise_resource.cellular_jitter
	noise.cellular_return_type = noise_resource.cellular_return_type
	
	noise.domain_warp_amplitude = noise_resource.domain_warp_amplitude
	noise.domain_warp_enabled = noise_resource.domain_warp_enabled
	noise.domain_warp_fractal_gain = noise_resource.domain_warp_fractal_gain
	noise.domain_warp_fractal_lacunarity = noise_resource.domain_warp_fractal_lacunarity
	noise.domain_warp_fractal_octaves = noise_resource.domain_warp_fractal_octaves
	noise.domain_warp_fractal_type = noise_resource.domain_warp_fractal_type
	noise.domain_warp_frequency = noise_resource.domain_warp_frequency
	noise.domain_warp_type = noise_resource.domain_warp_type
	
	noise.fractal_gain = noise_resource.fractal_gain
	noise.fractal_lacunarity = noise_resource.fractal_lacunarity
	noise.fractal_octaves = noise_resource.fractal_octaves
	noise.fractal_ping_pong_strength = noise_resource.fractal_ping_pong_strength
	noise.fractal_type = noise_resource.fractal_type
	noise.fractal_weighted_strength = noise_resource.fractal_weighted_strength

func load_values_from_resource() -> void:
	noise_type_input.select(noise_resource.noise_type)
	seed_input.value = noise_resource.seed
	frequency_input.value = noise_resource.frequency
	offset_input.set_vector3(noise_resource.offset)
	
	cellular_distance_function_input.select(noise_resource.cellular_distance_function)
	cellular_jitter_input.value = noise_resource.cellular_jitter
	cellular_return_type_input.select(noise_resource.cellular_return_type)
	
	domain_warp_amplitude_input.value = noise_resource.domain_warp_amplitude
	domain_warp_enabled_input.button_pressed = noise_resource.domain_warp_enabled
	domain_warp_fractal_gain_input.value = noise_resource.domain_warp_fractal_gain
	domain_warp_fractal_lacunarity_input.value = noise_resource.domain_warp_fractal_lacunarity
	domain_warp_fractal_octaves_input.value = noise_resource.domain_warp_fractal_octaves
	domain_warp_fractal_type_input.select(noise_resource.domain_warp_fractal_type)
	domain_warp_frequency_input.value = noise_resource.domain_warp_frequency
	domain_warp_type_input.select(noise_resource.domain_warp_type)
	
	fractal_gain_input.value = noise_resource.fractal_gain
	fractal_lacunarity_input.value = noise_resource.fractal_lacunarity
	fractal_octaves_input.value = noise_resource.fractal_octaves
	fractal_ping_pong_strength_input.value = noise_resource.fractal_ping_pong_strength
	fractal_type_input.select(noise_resource.fractal_type)
	fractal_weighted_strength_input.value = noise_resource.fractal_weighted_strength

func init_ui() -> void:
	add_options_to_input(noise_types, noise_type_input)
	add_options_to_input(cellular_distance_functions, cellular_distance_function_input)
	add_options_to_input(cellular_return_types, cellular_return_type_input)
	add_options_to_input(domain_warp_fractal_types, domain_warp_fractal_type_input)
	add_options_to_input(domain_warp_types, domain_warp_type_input)
	add_options_to_input(fractal_types, fractal_type_input)

func add_options_to_input(options: Array, input: OptionButton) -> void:
	for option in options:
		input.add_item(option[0], option[1])


func _on_generate_button_pressed():
	set_noise_values()
	noise_generated.emit(noise)

func check_if_live() -> void:
	if is_live:
		set_noise_values()
		noise_generated.emit(noise)


func _on_live_toggle_toggled(button_pressed):
	is_live = button_pressed
	check_if_live()


# ---------- Input Signals

func _on_noise_type_input_item_selected(index):
	noise_resource.noise_type = index
	check_if_live()

func _on_seed_input_value_changed(value):
	noise_resource.seed = value
	check_if_live()

func _on_frequency_input_value_changed(value):
	noise_resource.frequency = value
	check_if_live()

func _on_offset_input_value_changed(value):
	noise_resource.offset = value
	check_if_live()

func _on_cellular_distance_function_input_item_selected(index):
	noise_resource.cellular_distance_function = index
	check_if_live()

func _on_cellular_return_type_input_item_selected(index):
	noise_resource.cellular_return_type = index
	check_if_live()

func _on_cellular_jitter_input_value_changed(value):
	noise_resource.cellular_jitter = value
	check_if_live()

func _on_domain_warp_type_input_item_selected(index):
	noise_resource.domain_warp_type = index
	check_if_live()

func _on_domain_warp_enabled_input_toggled(button_pressed):
	noise_resource.domain_warp_enabled = button_pressed
	check_if_live()

func _on_domain_warp_amplitude_input_value_changed(value):
	noise_resource.domain_warp_amplitude = value
	check_if_live()

func _on_domain_warp_frequency_input_value_changed(value):
	noise_resource.domain_warp_frequency = value
	check_if_live()

func _on_domain_warp_fractal_type_input_item_selected(index):
	noise_resource.domain_warp_fractal_type = index
	check_if_live()

func _on_domain_warp_fractal_gain_input_value_changed(value):
	noise_resource.domain_warp_fractal_gain = value
	check_if_live()

func _on_domain_warp_fractal_lacunarity_input_value_changed(value):
	noise_resource.domain_warp_fractal_lacunarity = value
	check_if_live()

func _on_domain_warp_fractal_octaves_input_value_changed(value):
	noise_resource.domain_warp_fractal_octaves = value
	check_if_live()

func _on_fractal_type_input_item_selected(index):
	noise_resource.fractal_type = index
	check_if_live()

func _on_fractal_gain_input_value_changed(value):
	noise_resource.fractal_gain = value
	check_if_live()

func _on_fractal_lacunarity_input_value_changed(value):
	noise_resource.fractal_lacunarity = value
	check_if_live()

func _on_fractal_octaves_input_value_changed(value):
	noise_resource.fractal_octaves = value
	check_if_live()

func _on_fractal_ping_pong_strength_input_value_changed(value):
	noise_resource.fractal_ping_pong_strength = value
	check_if_live()

func _on_fractal_weighted_strength_input_value_changed(value):
	noise_resource.fractal_weighted_strength = value
	check_if_live()


# ---------- Button Signals

func _on_reset_values_pressed():
	noise_resource = NoiseResource.new()
	_on_all_unlock_pressed()
	load_values_from_resource()
	check_if_live()

func _on_save_resource_pressed() -> void:
	file_dialog.file_mode = FileDialog.FILE_MODE_SAVE_FILE
	file_dialog.visible = true

func _on_load_resource_pressed():
	file_dialog.file_mode = FileDialog.FILE_MODE_OPEN_FILE
	file_dialog.visible = true

func _on_file_dialog_file_selected(path):
	if file_dialog.file_mode == FileDialog.FILE_MODE_OPEN_FILE:
		load_noise_resource_file(path)
	elif file_dialog.file_mode == FileDialog.FILE_MODE_SAVE_FILE:
		save_noise_resource_file(path)

func load_noise_resource_file(path: String) -> void:
	var loaded_noise_resource = ResourceLoader.load(path)
	if loaded_noise_resource is NoiseResource:
		noise_resource = loaded_noise_resource
		load_values_from_resource()
		message_output.text = "Load Success"
	else:
		message_output.text = "Error: Wrong Resource Type"

func save_noise_resource_file(path: String) -> void:
		var error = ResourceSaver.save(noise_resource, path)
		message_output.text = get_error_message(error)

func get_error_message(error: Error) -> String:
	if error == OK:
		return "Saved Successfully"
	else:
		return "Error: Save Failed"


func toggle_input_group(input_group: Array[Dictionary], is_unlocked: bool) -> void:
	for input_set in input_group:
		input_set.lock.button_pressed = is_unlocked

func randomize_input_group(input_group: Array[Dictionary]) -> void:
	var rng = RandomNumberGenerator.new()
	rng.randomize()
	
	for input_set in input_group:
		if input_set.lock.button_pressed:
			var input_control = input_set.input
			if input_set.input is SpinBox:
				input_control.value = rng.randf_range(input_control.min_value, input_control.max_value)
			elif input_set.input is OptionButton:
				input_control.select(rng.randi_range(0, input_control.item_count-1))
			elif input_set.input is CheckBox:
				input_control.button_pressed = randi() % 2 == 0
			elif input_set.input is Vector3Input:
				input_control.randomize_values()


func _on_general_reroll_pressed():
	randomize_input_group(general_inputs)

func _on_general_unlock_pressed():
	toggle_input_group(general_inputs, true)

func _on_general_lock_pressed():
	toggle_input_group(general_inputs, false)


func _on_cellular_reroll_pressed():
	randomize_input_group(cellular_inputs)

func _on_cellular_unlock_pressed():
	toggle_input_group(cellular_inputs, true)

func _on_cellular_lock_pressed():
	toggle_input_group(cellular_inputs, false)


func _on_domain_warp_reroll_pressed():
	randomize_input_group(domain_warp_inputs)

func _on_domain_warp_unlock_pressed():
	toggle_input_group(domain_warp_inputs, true)

func _on_domain_warp_lock_pressed():
	toggle_input_group(domain_warp_inputs, false)


func _on_domain_warp_fractal_reroll_pressed():
	randomize_input_group(domain_warp_fractal_inputs)

func _on_domain_warp_fractal_unlock_pressed():
	toggle_input_group(domain_warp_fractal_inputs, true)

func _on_domain_warp_fractal_lock_pressed():
	toggle_input_group(domain_warp_fractal_inputs, false)


func _on_fractal_reroll_pressed():
	randomize_input_group(fractal_inputs)

func _on_fractal_unlock_pressed():
	toggle_input_group(fractal_inputs, true)

func _on_fractal_lock_pressed():
	toggle_input_group(fractal_inputs, false)


func _on_all_reroll_pressed():
	randomize_input_group(general_inputs)
	randomize_input_group(cellular_inputs)
	randomize_input_group(domain_warp_inputs)
	randomize_input_group(domain_warp_fractal_inputs)
	randomize_input_group(fractal_inputs)

func _on_all_unlock_pressed():
	toggle_input_group(general_inputs, true)
	toggle_input_group(cellular_inputs, true)
	toggle_input_group(domain_warp_inputs, true)
	toggle_input_group(domain_warp_fractal_inputs, true)
	toggle_input_group(fractal_inputs, true)

func _on_all_lock_pressed():
	toggle_input_group(general_inputs, false)
	toggle_input_group(cellular_inputs, false)
	toggle_input_group(domain_warp_inputs, false)
	toggle_input_group(domain_warp_fractal_inputs, false)
	toggle_input_group(fractal_inputs, false)
