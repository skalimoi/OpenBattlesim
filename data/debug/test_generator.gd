extends VoxelGeneratorScript

# Change channel to SDF
const channel : int = VoxelBuffer.CHANNEL_SDF

func _generate_block(out_buffer : VoxelBuffer, origin_in_voxels : Vector3i, lod : int) -> void:
	
	var sample_data : Array = []
	
	
	# We'll have to iterate every 3D voxel in the block this time
	for rz in out_buffer.get_size().z:
		for rx in out_buffer.get_size().x:
			# The following part only depends on `x` and `z`, 
			# so moving it out of the innermost loop optimizes things a little.

			# Get voxel world position.
			# To account for LOD we multiply local coordinates by 2^lod.
			# This can be done faster than `pow()` by using binary left-shift.
			# Y is left out because we'll compute it in the inner loop.
			var pos_world := Vector3(origin_in_voxels) + Vector3(rx << lod, 0, rz << lod)

			# Generates infinite "wavy" hills.
			var height := 10.0 * (sin(pos_world.x * 0.1) + cos(pos_world.z * 0.1))

			# Innermost loop
			for ry in out_buffer.get_size().y:
				pos_world.y = origin_in_voxels.y + (ry << lod)

				# This is a cheap approximation for the signed distance of a heightfield
				var signed_distance := pos_world.y - height

				# When outputting signed distances, use `set_voxel_f` instead of `set_voxel`
				out_buffer.set_voxel_f(signed_distance, rx, ry, rz, channel)
