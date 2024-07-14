use godot::prelude::*;
use map_range::{CheckedMapRange, MapRange};

struct MapLoader;

#[gdextension]
unsafe impl ExtensionLibrary for MapLoader {}

#[derive(GodotClass)]
#[class(base=Node)]
struct ChunkLoader;

#[godot_api]
impl INode for ChunkLoader {
    fn init(base: Base<Self::Base>) -> Self {
        Self
    }
}
#[godot_api]
impl ChunkLoader {
    #[func]
    pub fn load_from_file(scenario_name: GString, x: real, y: real, is_test: bool) -> Array<u16> {
        use savefile::prelude::*;
        let mut data: Array<u16> = Array::new();
        if is_test {
            let d: Vec<u16> = load_file_noschema(format!("data/debug/test_scenes/{}/h_map_tile_x{}_y{}.dat", scenario_name, x, y), 0).unwrap();
            data = Array::from(d.as_slice())
        }
        if !is_test {
            array![]
        }
        else { array![] }
    }
    pub fn get_height_from_bounds(data: Array<u16>, x: real, y: real, min: real, max: real) -> real {
        data.get(((x as usize) * 8192 + (y as usize))).unwrap().map_range(0..16384, (min as u16)..(max as u16)).into()
    }
}