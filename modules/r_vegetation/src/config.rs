


use image::io::Reader as ImageReader;
use image::ImageBuffer;
use image::Luma;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use image::imageops::FilterType;
use crate::edaphology::calculate_soil_depth;
use crate::hydrology::calculate_hydrology_map;
use crate::insolation::calculate_actual_insolation;
use crate::orography::calculate_normal_map;
use crate::probabilities::calculate_probabilities;


pub struct GreyscaleImage<T> {
    pub image: Vec<T>,
    len: usize,
}

impl<T> GreyscaleImage<T> {
    pub fn new(image: Vec<T>) -> Self {
        let len = (image.len() as f64).sqrt() as usize;
        Self { image, len }
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> std::ops::Index<(usize, usize)> for GreyscaleImage<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.image.index(index.0 + index.1 * self.len)
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for GreyscaleImage<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.image.index_mut(index.0 + index.1 * self.len)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Map {
    pub biom: String,
    pub height_map_path: String,
    pub texture_map_path: String,
    pub height_conversion: f64, // The factor to convert a height value of the height-map to the actual height
    pub max_soil_depth: f64,    // in cm, states the maximal depth the ground can have when it has no tilt
    pub pixel_size: f64,        // the size that a pixel covers of the real map in m
}

#[derive(Deserialize)]
pub struct Biom {
    // this value corresponds to the diffuse sun beam scattering by the atmosphere
    pub atmospheric_diffusion: f64,  // in percent
    pub atmospheric_absorption: f64, // in percent
    pub cloud_reflection: f64,       // in percent
    pub avg_rainfall_per_day: f64,   // in l/cm²
    pub groundwater: f64,            // in l/cm²
}

#[derive(Deserialize)]
pub struct Soil {
    pub id: u8,
    pub albedo: f64,
    pub water_absorption: f64,
}

#[derive(Deserialize)]
pub struct Vegetation {
    pub energy_demand: f64,
    pub water_demand: f64,
    pub soil_demand: String,
    pub soil_depth_demand: f64,
}

pub struct Sun {
    pub azimuth: f64,
    pub elevation: f64,
}

impl Sun {
    pub fn convert_to_uv_coordinates(&self) -> Vector3<f64> {
        let u = self.azimuth.to_radians().cos() * self.elevation.to_radians().cos();
        let v = self.azimuth.to_radians().sin() * self.elevation.to_radians().cos();
        let w = self.elevation.to_radians().sin();
        Vector3::new(round(u, 4), round(v, 4), round(w, 4))
    }
}

pub fn round(x: f64, n: i32) -> f64 {
    let p = 10_f64.powi(n);
    (x * p).round() / p
}

pub fn clamp_idx(c: usize, o: i32, len: usize) -> usize {
    (c as i32 + o).clamp(0, len as i32) as usize
}

pub struct SunConfig {
    pub daylight_hours: i32,
    pub sun_start_elevation: f64,
    pub sun_start_azimuth: f64,
    pub sun_max_elevation: f64,
}

pub struct SimArgs<'a> {
    pub height_map: GreyscaleImage<f64>,
    pub soil_ids_map: GreyscaleImage<u8>,
    pub soils: &'a HashMap<u8, Soil>,
    pub sun_config: &'a SunConfig,
    pub reflection_coefficient: f64,
    pub map: &'a Map,
    pub vegetation: &'a Vegetation,
    pub biom: &'a Biom,
}

#[derive(Deserialize)]
pub struct SimConfig {
    maps: Map,
    bioms: HashMap<String, Biom>,
    #[serde(deserialize_with = "deserialize_soils")]
    soil_ids: HashMap<u8, Soil>,
    soil_names: HashMap<String, u8>,
    vegetations: HashMap<String, Vegetation>,
}

fn deserialize_soils<'de, D>(deserializer: D) -> Result<HashMap<u8, Soil>, D::Error>
    where
        D: serde::Deserializer<'de>,
{
    let s: HashMap<String, Soil> = Deserialize::deserialize(deserializer)?;
    Ok(s.into_values().map(|soil| (soil.id, soil)).collect())
}

impl SimConfig {
    pub fn from_configs(
        maps: Map,
        bioms: HashMap<String, Biom>,
        soils: HashMap<String, Soil>,
        vegetations: HashMap<String, Vegetation>,
    ) -> Self {
        let (soil_names, soil_ids): (HashMap<String, u8>, HashMap<u8, Soil>) = soils
            .into_iter()
            .map(|(name, soil)| ((name, soil.id), (soil.id, soil)))
            .unzip();
        Self {
            maps,
            bioms,
            soil_ids,
            soil_names,
            vegetations,
        }
    }
    pub fn calculate_maps(
        &self,
        map_name: &str,
        vegetation_name: &str,
        sun_config: &SunConfig,
        reflection_coefficient: f64,
    ) {
        let map = &self.maps;

        let height_map = GreyscaleImage::new(
            ImageReader::open(&map.height_map_path)
                .unwrap()
                .decode()
                .unwrap()
                .into_luma16()
                .into_raw()
                .into_iter()
                .map(|x| x as f64)
                .collect(),
        );

        let soil_ids_map = GreyscaleImage::new(
            ImageReader::open(&map.texture_map_path)
                .unwrap()
                .decode()
                .unwrap()
                .into_luma8()
                .into_raw(),
        );

        let sim_args = SimArgs {
            height_map,
            soil_ids_map,
            soils: &self.soil_ids,
            sun_config,
            reflection_coefficient,
            map,
            vegetation: &self.vegetations[vegetation_name],
            biom: &self.bioms[&map.biom],
        };
        let insolation_map = calculate_actual_insolation(&sim_args);
        let orographic_map = calculate_normal_map(&sim_args);
        let edaphic_map = calculate_soil_depth(&orographic_map, sim_args.map);
        let hydrology_map = calculate_hydrology_map(&sim_args, &edaphic_map, &insolation_map);

        //std::fs::write("insolation_rust.json", serde_json::to_string(&insolation_map.image).unwrap()).unwrap();
        let insolation_image = ImageBuffer::<Luma<u16>, Vec<u16>>::from_raw(
            insolation_map.len() as u32,
            insolation_map.len() as u32,
            insolation_map.image.into_iter().map(|x| x as u16).collect(),
        )
            .unwrap();
        //std::fs::write("edaphic_rust.json", serde_json::to_string(&edaphic_map.image).unwrap()).unwrap();
        let edaphic_image = ImageBuffer::<Luma<u16>, Vec<u16>>::from_raw(
            edaphic_map.len() as u32,
            edaphic_map.len() as u32,
            edaphic_map.image.into_iter().map(|x| x as u16).collect(),
        )
            .unwrap();
        //std::fs::write("hydrology_rust.json", serde_json::to_string(&hydrology_map.image).unwrap()).unwrap();
        let hydrology_image = ImageBuffer::<Luma<u16>, Vec<u16>>::from_raw(
            hydrology_map.len() as u32,
            hydrology_map.len() as u32,
            hydrology_map.image.into_iter().map(|x| (x * 1000.0) as u16).collect(),
        )
            .unwrap();

        std::fs::create_dir_all(format!("data/vegetation_data/{map_name}")).unwrap();

        insolation_image
            .save(format!("data/vegetation_data/{map_name}/{map_name}_insolation.png"))
            .unwrap();
        edaphic_image
            .save(format!("data/vegetation_data/{map_name}/{map_name}_edaphic.png"))
            .unwrap();
        hydrology_image
            .save(format!("data/vegetation_data/{map_name}/{map_name}_water.png"))
            .unwrap();

        std::fs::write(
            format!("data/vegetation_data/{map_name}/{map_name}_normals.json"),
            serde_json::to_string(&orographic_map.image.into_iter().map(|x| [x.x, x.y, x.z]).collect::<Vec<_>>()).unwrap(),
        )
            .unwrap();

        // let orographic_image = ImageBuffer::<Luma<u16>, Vec<u16>>::from_raw(
        //     orographic_map.len() as u32,
        //     orographic_map.len() as u32,
        //     orographic_map.image,
        // )
        // .unwrap();
    }
    pub fn calculate_probabilities(&self, map_name: &str, vegetation_name: &str, daylight_hours: i32) {
        let soil_ids_map = GreyscaleImage::new(
            ImageReader::open(&self.maps.texture_map_path)
                .unwrap()
                .decode()
                .unwrap()
                .into_luma8()
                .into_raw(),
        );
        let insolation_map = GreyscaleImage::new(
            ImageReader::open(format!(
                "data/vegetation_data/{map_name}/{map_name}_insolation.png",
            ))
                .unwrap()
                .decode()
                .unwrap()
                .into_luma16()
                .into_raw()
                .into_iter()
                .map(|x| x as f64)
                .collect(),
        );
        let edaphic_map = GreyscaleImage::new(
            ImageReader::open(format!("data/vegetation_data/{map_name}/{map_name}_edaphic.png"))
                .unwrap()
                .decode()
                .unwrap()
                .into_luma16()
                .into_raw()
                .into_iter()
                .map(|x| x as f64)
                .collect(),
        );
        let hydrology_map = GreyscaleImage::new(
            ImageReader::open(format!("data/vegetation_data/{map_name}/{map_name}_water.png"))
                .unwrap()
                .decode()
                .unwrap()
                .into_luma16()
                .into_raw()
                .into_iter()
                .map(|x| x as f64 / 1000.0)
                .collect(),
        );

        let probabilities_map = calculate_probabilities(
            &self.vegetations[vegetation_name],
            &soil_ids_map,
            &self.soil_names,
            &insolation_map,
            &edaphic_map,
            &hydrology_map,
        );
        //std::fs::write("probabilities_rust.json", serde_json::to_string(&probabilities_map.image).unwrap()).unwrap();
        let probabilities_image = ImageBuffer::<Luma<u16>, Vec<u16>>::from_raw(
            probabilities_map.len() as u32,
            probabilities_map.len() as u32,
            probabilities_map.image.into_iter().map(|x| (x * 1000.0) as u16).collect(),
        )
            .unwrap();
        probabilities_image
            .save(format!(
                "data/vegetation_data/{map_name}/{vegetation_name}_total.png"
            ))
            .unwrap();
        let to_tiling = image::io::Reader::open(format!("data/vegetation_data/height_map/{vegetation_name}_total.png")).unwrap().decode().unwrap();
        let brightened = imageproc::contrast::stretch_contrast(&to_tiling.into_luma8(), 0, 2);
        brightened.save(format!("data/vegetation_data/height_map/{vegetation_name}_equalized.png")).unwrap();
        let blur = imageproc::filter::gaussian_blur_f32(&brightened, 2.0);
        let resampled_blur = image::imageops::resize(&blur, 8193, 8193, FilterType::Gaussian);
        let tile_size: usize = 513;
        for tile_x in 0..=15 {
            for tile_y in 0..=15 {
                let tile = image::imageops::crop_imm(&resampled_blur, (tile_x * tile_size) as u32, (tile_y * tile_size) as u32, tile_size as u32, tile_size as u32);
                tile.to_image().save(format!("data/vegetation_data/height_map/{vegetation_name}_{tile_x}_{tile_y}.png")).unwrap();
            }
        }

    }
}
