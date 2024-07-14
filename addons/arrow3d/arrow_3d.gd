@icon("res://addons/arrow3d/arrow_3d.svg")
@tool
class_name Arrow3D
extends Marker3D

@export var arrow_size = 1.0 : set = set_size
@export var arrow_thickness = 0.05 : set = set_thickness
@export var arrow_material = StandardMaterial3D.new()
@export var opposite_direction = false : set = flip_direction
@export var is_centered = false : set = center_arrow

@export var head_segments = 6 : set = set_head_segments
@export var stick_segments = 6 : set = set_stick_segments

var arrow_head = MeshInstance3D.new()
var arrow_head_mesh = CylinderMesh.new()
var arrow_stick = MeshInstance3D.new()
var arrow_stick_mesh = CylinderMesh.new()
var arrow_rig = Node3D.new()

func _ready():
	arrow_head_mesh.radial_segments = head_segments
	arrow_stick_mesh.radial_segments = stick_segments

	gizmo_extents = 0
	arrow_stick.mesh = arrow_stick_mesh
	arrow_stick.material_override = arrow_material
	
	# Init arrow head
	arrow_head.mesh = arrow_head_mesh
	arrow_head.material_override = arrow_material
	
	# Add to scene
	arrow_rig.add_child(arrow_stick)
	arrow_rig.add_child(arrow_head)
	add_child(arrow_rig)
	
	resize_arrow()
	center_arrow(is_centered)

func flip_direction(value):
	opposite_direction = value
	resize_arrow()

func set_size(size):
	arrow_size = size
	resize_arrow()

func set_thickness(thickness):
	arrow_thickness = thickness
	resize_arrow()

func set_head_segments(segments):
	head_segments = segments
	arrow_head_mesh.radial_segments = head_segments

func set_stick_segments(segments):
	stick_segments = segments
	arrow_stick_mesh.radial_segments = stick_segments

func resize_arrow():
	arrow_stick_mesh.bottom_radius = arrow_thickness
	arrow_stick_mesh.top_radius = arrow_thickness
	arrow_stick_mesh.height = (arrow_size / 3) * 2
	arrow_head_mesh.top_radius = 0.0
	arrow_head_mesh.bottom_radius = arrow_thickness * 2
	arrow_head_mesh.height = arrow_size / 3
	arrow_head.rotation_degrees.x = 90 if not opposite_direction else -90
	arrow_stick.rotation_degrees.x = 90 if not opposite_direction else -90

	if is_centered:
		if opposite_direction:
			arrow_head.position.z = -arrow_size / 2
			arrow_stick.position.z = 0
		else:
			arrow_head.position.z = arrow_size / 2
			arrow_stick.position.z = 0
	else:
		if opposite_direction:
			arrow_head.position.z = -(arrow_stick_mesh.height + arrow_head_mesh.height / 2)
			arrow_stick.position.z = -arrow_stick_mesh.height / 2
		else:
			arrow_head.position.z = arrow_stick_mesh.height + arrow_head_mesh.height / 2
			arrow_stick.position.z = arrow_stick_mesh.height / 2

func center_arrow(centered):
	is_centered = centered
	resize_arrow()
