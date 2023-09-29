use std::fs::File;
use godot::{prelude::*, engine::Area3D};
use noise::{Fbm, Simplex};
use ron::de::from_reader;
use crate::climate::{Climate, KOPPEN_AF_AM, KOPPEN_AS, KOPPEN_AW, KOPPEN_BSH, KOPPEN_BSK, KOPPEN_BWH, KOPPEN_BWK, KOPPEN_CFA, KOPPEN_CFB, KOPPEN_CFC, KOPPEN_CSA, KOPPEN_CSB, KOPPEN_CWA, KOPPEN_CWB, KOPPEN_CWC, KOPPEN_DFA, KOPPEN_DFB, KOPPEN_DFC, KOPPEN_DSA, KOPPEN_DSB, KOPPEN_DSC, KOPPEN_ET};
use crate::gendata::GenData;

#[derive(GodotClass)]
#[class(base=Area3D)]
pub struct GridComponent {
    #[var]
    pub index: Vector3i,
    #[var]
    pub mean_altitude: real,
    #[var]
    pub temperature: real,
    #[var]
    pub wind_p: Vector3,
    #[var]
    pub pressure: real,
    #[var]
    pub humidity: real,
    #[var]
    pub td: real,
    #[base]
    base: Base<Area3D>
}

#[godot_api]
impl GridComponent {
    /// index number => 0: humidity, 1: td, 2: pressure, 3: temperature
    #[func]
    pub fn load_float_data(&mut self, index: i32) -> PackedFloat32Array {
        let index_v = self.index;
        let gendata_file = File::open(format!("data/weather_grid_data/{}_{}_{}.ron", index_v.x, index_v.y, index_v.z)).unwrap();
        let gendata: GenData = from_reader(gendata_file).expect("Error loading weather file.");
        match index {
            // humidity
            0 => PackedFloat32Array::from(gendata.humidity.as_slice()),
            // td
            1 => PackedFloat32Array::from(gendata.td.as_slice()),
            // pressure
            2 => PackedFloat32Array::from(gendata.pressure.as_slice()),
            // temperature
            3 => PackedFloat32Array::from(gendata.temperature.as_slice()),
            _ => {PackedFloat32Array::new()}
        }
    }
    #[func]
    pub fn load_vector3_data(&mut self) -> PackedVector3Array {
        let index_v = self.index;
        let gendata_file = File::open(format!("data/weather_grid_data/{}_{}_{}.ron", index_v.x, index_v.y, index_v.z)).unwrap();
        let gendata: GenData = from_reader(gendata_file).expect("Error loading weather file.");
        let mut wind_builtin_vec3: Vec<Vector3> = vec![];
        for tuple in gendata.wind.clone() {
            let vector = Vector3::new(tuple.0, tuple.1, tuple.2);
            wind_builtin_vec3.push(vector);
        };
        let packed = PackedVector3Array::from(wind_builtin_vec3.as_slice());
        packed
    }

    #[func]
    pub fn set_data(&mut self, acc_hour: real, humidity: PackedFloat32Array, wind: PackedVector3Array, td: PackedFloat32Array, pressure: PackedFloat32Array, temperature: PackedFloat32Array) {

        self.humidity = humidity.to_vec()[acc_hour as usize];
        self.wind_p = wind.to_vec()[acc_hour as usize];
        self.td = td.to_vec()[acc_hour as usize];
        self.pressure = pressure.to_vec()[acc_hour as usize];
        self.temperature = temperature.to_vec()[acc_hour as usize];
    }
    #[func]
    pub fn generate_data(&mut self, latitude: i32, climate: GodotString) {
        let mut fetched: Climate = KOPPEN_ET;
        let climate_vec = vec![KOPPEN_AS, KOPPEN_AF_AM, KOPPEN_AW, KOPPEN_BSH, KOPPEN_BSK, KOPPEN_BWH, KOPPEN_CFA, KOPPEN_CFB, KOPPEN_BWK, KOPPEN_CFC, KOPPEN_CSA, KOPPEN_CSB, KOPPEN_CWA, KOPPEN_CWB, KOPPEN_CWC, KOPPEN_DFA, KOPPEN_DFB, KOPPEN_DFC, KOPPEN_DSA, KOPPEN_DSB, KOPPEN_DSC, KOPPEN_ET];
        for climate_iter in climate_vec {
            let formatted = format!("{}{}{}", climate_iter.general_type, climate_iter.second_type, climate_iter.third_type);
            let name = formatted.as_str();
            let formatted_two = climate.to_string();
            let typed_name = formatted_two.as_str();
            if typed_name == name {
                fetched = climate_iter;
                break
            } else {
                continue
            }
        }
        let noise: Fbm<Simplex> = Fbm::new(345435);
        let gen_data = GenData::gen_year_data(self, latitude, self.mean_altitude, (self.index.x, self.index.y, self.index.z), noise, fetched);
        GenData::save_data(gen_data).expect("Error exporting GenData!");
    }
    // FUNCION PARA PROCESS - ASIGNAR VALORES SEGÚN HORA
}

#[godot_api]
impl NodeVirtual for GridComponent {
    fn init(base: Base<Area3D>) -> Self {
        GridComponent {
            index: Vector3i::new(0, 0, 0),
            mean_altitude: 0.0,
            temperature: 0.0,
            wind_p: Vector3::new(0.0, 0.0, 0.0),
            pressure: 0.0,
            humidity: 0.0,
            td: 0.0,
            base
        }
    }

    fn ready(&mut self) {
        
    }
}