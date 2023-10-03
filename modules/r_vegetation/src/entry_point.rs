use std::{collections::HashMap, fs, fs::File, io::Read};
use std::io::Write;
use godot::builtin::{GodotString, NodePath, real, Variant, Vector3};
use godot::engine::{CapsuleMesh, Mesh, MultiMesh, MultiMeshInstance3D};
use godot::prelude::{Base, FromVariant, Gd, godot_api, godot_print, GodotClass, Node, NodeVirtual};
use image::{load, Pixel};
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
    pub fn scatter_values(&mut self, vegetation: GodotString) {
        let mut density = 0;
        let mut terrain = self.base.get_node("/root/Node3D/MTerrain".parse().unwrap()).unwrap();
        let mut to_clone = self.base.get_node("original".parse().unwrap()).unwrap();
        let placeholder: Gd<CapsuleMesh> = godot::prelude::load::<CapsuleMesh>("tmp/placeholder/scatter_mesh_1.tres");
        let mut multimesh = MultiMesh::new();
        multimesh.set_mesh(placeholder.upcast());
        let mask = image::io::Reader::open(vegetation.to_string()).unwrap().decode().unwrap();
        for x in 0..=100 {
            for y in 0..=100 {
                let height= terrain.call("get_height_by_pixel".parse().unwrap(), [Variant::from(x), Variant::from(y)].as_slice());
                let pos = terrain.call("get_pixel_world_pos".parse().unwrap(), [Variant::from(x + 10), Variant::from(y + 10)].as_slice());
                let mask_value = mask.to_luma8().get_pixel(x as u32, y as u32).0[0];
                match mask_value {
                    15..=60 => density = 100,
                    61..=80 => density = 250,
                    81..=100 => density = 450,
                    101..=255 => density = 700,
                    _ => density = 0,
            }
                if density != 0 {
                    let height_float: Vector3 = pos.to();
                    let mut scatter_node: Gd<MultiMeshInstance3D> = to_clone.duplicate().unwrap().cast();
                    // scatter_node.set_script(Variant::from("res://addons/multimesh_scatter/multimesh_scatter.gd"));
                    scatter_node.set("count".parse().unwrap(), Variant::from(density));
                    scatter_node.set("placement_type".parse().unwrap(), Variant::from("Sphere"));
                    scatter_node.set("placement_size".parse().unwrap(), Variant::from(Vector3::new(2.0, 1.0, 2.0)));
                    let name = format!("arctic_grass_{}_{}", x, y);
                    scatter_node.set("name".parse().unwrap(), Variant::from(name.clone()));
                    // scatter_node.set_multimesh(multimesh.clone());
                    self.base.add_child(scatter_node.clone().upcast());
                    scatter_node.cast::<MultiMeshInstance3D>().set_global_position(Vector3::new(Vector3::from_variant(&pos).x, height_float.y, Vector3::from_variant(&pos).z));
                }
            }
        }
    }
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