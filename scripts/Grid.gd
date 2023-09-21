extends Node

func _ready():
	var grid_size = Vector3i(16, 6, 16)
	var box_size = Vector3(512, 256, 512)
	
	for x in (grid_size.x):
		for y in (grid_size.y):
			for z in (grid_size.z):
				var area = GridComponent.new()
				area.name = "grid_{0}_{1}_{2}".format([x, y, z])
				area.index = Vector3i(x, y, z)
				var shape = CollisionShape3D.new()
				var box = BoxShape3D.new()
				shape.name = "shape_{0}_{1}_{2}".format([x, y, z])
				box.size = box_size
				shape.set_shape(box)
				self.add_child(area)
				area.add_child(shape)
				area.global_position = Vector3((512 * x) + 256, 256 * y, (512 * z) + 256)
				if y == 0:
					var pos = area.global_position
					var point_1 = $"../MTerrain".get_closest_height(area.global_position)
					var point_2 = $"../MTerrain".get_closest_height(Vector3(area.global_position.x + 256, area.global_position.y, area.global_position.z + 256))
					var point_3 = $"../MTerrain".get_closest_height(Vector3(area.global_position.x + 256, area.global_position.y, area.global_position.z))
					var point_4 = $"../MTerrain".get_closest_height(Vector3(area.global_position.x - 256, area.global_position.y, area.global_position.z))
					var point_5 = $"../MTerrain".get_closest_height(Vector3(area.global_position.x, area.global_position.y, area.global_position.z - 256))
					var point_6 = $"../MTerrain".get_closest_height(Vector3(area.global_position.x - 256, area.global_position.y, area.global_position.z - 256))
					var mean = (point_1 + point_2 + point_3 + point_4 + point_5 + point_6) / 6
					area.mean_altitude = mean
				area.generate_data(40, "BSH")
	
