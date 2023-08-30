#[derive(GodotClass)]
#[class(base=MeshInstance3D)]
struct TerrainTile {
    detail_level: i8,
    coordinates: Vector2,
}