use godot::{prelude::*, engine::Area3D};
use noise::{Fbm, Simplex};
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
    pub wind: Vector3,
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
    #[func]
    pub fn generate_data(&mut self, latitude: i32, climate: GodotString) {
        let mut fetched: Climate = KOPPEN_AW;
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
        godot_print!("Exporting weather data");
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
            wind: Vector3::new(0.0, 0.0, 0.0),
            pressure: 0.0,
            humidity: 0.0,
            td: 0.0,
            base
        }
    }

    fn ready(&mut self) {
        
    }
}