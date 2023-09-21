extends Node

func _ready():
	var grid_size = Vector3i(16, 6, 16)
	var box_size = Vector3(512, 256, 512)
	
	for x in (grid_size.x):
		for y in (grid_size.y):
			for z in (grid_size.z):
				var area = Area3D.new()
				area.name = "grid_{0}_{1}_{2}".format([x, y, z])
				var shape = CollisionShape3D.new()
				var box = BoxShape3D.new()
				shape.name = "shape_{0}_{1}_{2}".format([x, y, z])
				box.size = box_size
				shape.set_shape(box)
				self.add_child(area)
				area.add_child(shape)
				area.global_position = Vector3((512 * x) + 256, 256 * y, (512 * z) + 256)
				
