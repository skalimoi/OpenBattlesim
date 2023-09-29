use std::{collections::HashMap, fs, fs::File, io::Read};
use std::io::Write;
use godot::builtin::{GodotString, real};
use godot::prelude::{Base, godot_api, godot_print, GodotClass, Node, NodeVirtual};
use crate::config::{Biom, Map, SimConfig, Soil, SunConfig, Vegetation};

const MAP_NAME: &'static str = "Tiny";
const VEGETATION_NAME: &'static str = "Appletree";

#[derive(GodotClass)]
#[class(base=Node)]
pub struct VegetationGen {
    #[base]
    base: Base<Node>
}

#[godot_api]
impl VegetationGen {
    #[func]
    pub fn generate_yaml_map(&mut self, biom: GodotString, height_conversion: real, height_map_path: GodotString, max_soil_depth: real, pixel_size: real, texture_map_path: GodotString) {
        let map = Map {
            biom: biom.to_string(),
            height_conversion: height_conversion as f64,
            height_map_path: height_map_path.to_string(),
            max_soil_depth: max_soil_depth as f64,
            pixel_size: pixel_size as f64,
            texture_map_path: texture_map_path.to_string()
        };
        let yaml = serde_yaml::to_string(&map).unwrap();
        File::create("data/vegetation_raw_data/map.yml").unwrap();
        fs::write("data/vegetation_raw_data/map.yml", yaml).unwrap();

    }
    #[func]
    pub fn example_function(&mut self, map_name: GodotString, vegetation: GodotString) {
        let mut data = String::new();
        File::open("data/vegetation_raw_data/map.yml").unwrap().read_to_string(&mut data).unwrap();

        godot_print!("{}", data);
        let maps: Map = serde_yaml::from_str(&data).unwrap();

        let mut data = String::new();
        File::open("data/vegetation_raw_data/bioms.yml").unwrap().read_to_string(&mut data).unwrap();
        let bioms: HashMap<String, Biom> = serde_yaml::from_str(&data).unwrap();

        let mut data = String::new();
        File::open("data/vegetation_raw_data/soil_types.yml").unwrap().read_to_string(&mut data).unwrap();
        let soils: HashMap<String, Soil> = serde_yaml::from_str(&data).unwrap();

        let mut data = String::new();
        File::open("data/vegetation_raw_data/vegetation_types.yml")
            .unwrap()
            .read_to_string(&mut data)
            .unwrap();
        let vegetations: HashMap<String, Vegetation> = serde_yaml::from_str(&data).unwrap();

        let sun_config = SunConfig {
            daylight_hours: 13,
            sun_start_elevation: 0.0,
            sun_start_azimuth: 0.0,
            sun_max_elevation: 80.0,
        };
        let reflection_coefficient = 0.1;

        let sim_config = SimConfig::from_configs(maps, bioms, soils, vegetations);
        sim_config.calculate_maps(map_name.to_string().as_str(), vegetation.to_string().as_str(), &sun_config, reflection_coefficient);
        sim_config.calculate_probabilities(map_name.to_string().as_str(), vegetation.to_string().as_str(), sun_config.daylight_hours);
    }

}

#[godot_api]
impl NodeVirtual for VegetationGen {
    fn init(base: Base<Node>) -> Self {
        VegetationGen {
            base
        }
    }

    fn ready(&mut self) {

    }
}