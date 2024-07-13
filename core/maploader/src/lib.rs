use godot::prelude::*;

struct MapLoader;

#[gdextension]
unsafe impl ExtensionLibrary for MapLoader {}

#[derive(GodotClass)]
#[class(base=Node)]
struct ChunkLoader;

#[godot_api]
impl ChunkLoader {
    #[func]
    pub fn load_from_file(scenario_name: GString, is_test: bool) {
        //TODO get x and y from name
        use savefile::prelude::*;
        let data: Vec<u8> = load_file(scenario_name.into(), 0).unwrap_or(Vec::new());
    }
}