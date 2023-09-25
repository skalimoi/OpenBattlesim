use std::{collections::HashMap, fs::File, io::Read};
use r_vegetation::{Biom, Map, SimConfig, Soil, SunConfig, Vegetation};
use vegetation::{Biom, Map, SimConfig, Soil, SunConfig, Vegetation};

const MAP_NAME: &'static str = "Tiny";
const VEGETATION_NAME: &'static str = "Appletree";

fn main() {
    let mut data = String::new();
    File::open("data/maps.yml").unwrap().read_to_string(&mut data).unwrap();
    let maps: HashMap<String, Map> = serde_yaml::from_str(&data).unwrap();

    let mut data = String::new();
    File::open("data/bioms.yml").unwrap().read_to_string(&mut data).unwrap();
    let bioms: HashMap<String, Biom> = serde_yaml::from_str(&data).unwrap();

    let mut data = String::new();
    File::open("data/soil_types.yml").unwrap().read_to_string(&mut data).unwrap();
    let soils: HashMap<String, Soil> = serde_yaml::from_str(&data).unwrap();

    let mut data = String::new();
    File::open("data/vegetation_types.yml")
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
    sim_config.calculate_maps(MAP_NAME, VEGETATION_NAME, &sun_config, reflection_coefficient);
    sim_config.calculate_probabilities(MAP_NAME, VEGETATION_NAME, sun_config.daylight_hours);
}
