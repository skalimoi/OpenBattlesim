extends Node

@export var heightmap_path: String
@export var erosion_cycles: int
@export var seed: int
@export var biome: String
@export var colors: String
@export var current_cycle: int

func _ready():
	heightmap_path = "placeholder"
	erosion_cycles = 0
	seed = 0
	current_cycle = 0
	biome = "placeholder"
	colors = "placeholder"
