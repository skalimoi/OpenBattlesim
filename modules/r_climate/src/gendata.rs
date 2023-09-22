use godot::{prelude::*, engine::Curve2D};
use noise::{Fbm, NoiseFn, Simplex};
use crate::{grid::GridComponent, climate::{Climate, KOPPEN_AF_AM}};
use std::{ops::Range, collections::HashMap, fs};
use rand::{thread_rng, Rng};
use std::f32::consts::E;
use std::fmt::format;
use std::fs::File;
use std::io::Write;
use std::ptr::write;
use godot::engine::{Curve, ResourceImporter};
use godot::engine::utilities::lerpf;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use crate::climate::HumidDry;

pub const EQUATOR_TEMP_RANGE: Range<i32> = 0..3;
pub const TEMPERATE_TEMP_RANGE: Range<i32> = 7..10;
pub const SUB_POLAR_TEMP_RANGE: Range<i32> = 12..25;

#[derive(Copy, Clone)]
pub enum Season {
    Winter,
    Spring,
    Fall,
    Summer
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GenData {
    pub index: (i32, i32, i32),
    pub temperature: Vec<real>,
    pub altitude: real,
    pub pressure: Vec<real>,
    pub humidity: Vec<real>,
    pub wind: Vec<(f32, f32, f32)>,
    pub td: Vec<real>,
}

impl GenData {

    pub fn calculate_temperature(
        position: (i32, i32, i32),
        noise: Fbm<Simplex>,
        altitude: real,
        base_temp: real,
        offset: Vector3,
    ) -> f32 {
        let _noise_factor = noise.get([
            position.0 as f64 * offset.x as f64,
            position.1 as f64 * offset.y as f64,
            position.2 as f64 * offset.z as f64,
        ]) * 5.;

        // Fórmula: rT - (m/100 * 0.6) = T

        base_temp - ((altitude / 100.) * 0.6) /* + (noise_factor) as f32 */
    }

    pub fn calculate_pressure(altitude: f32, temperature: f32, land_pressure: f32) -> f32 {
        let frac = (0.0065 * altitude) / (temperature + (0.0065 * altitude) + 273.15);

        land_pressure * (1. - frac).powf(5.257) // land pressure is given in hectopascals
    }

    #[allow(clippy::too_many_arguments)]
    pub fn calculate_rel_hum(
        temperature: f32,
        factor: f32,
        tdprev: f32
    ) -> (f32, f32) {
        let mut td: f32 = 0.0;

        td = temperature - (factor);

        // if is_prec {
        //     td = tdprev;
        // }

        let frac1 = E.powf((17.625 * td) / (243.04 + td));

        let frac2 = E.powf((17.625 * temperature) / (243.04 + temperature));

        let hum = 100.0 * (frac1 / frac2);

        (hum, td)
    }

    fn seasonal_factor(latitude: i32, season: Season, climate: Climate) -> f32 {

        let season_factor: f32 = match (season, latitude) {
            // COGER RANDOM DEL RANGO ENTRE 0 Y EL MODIFICADOR Y ESO ES LO QUE SE RESTA, CUANTO MAS SE RESTA MENOS HUMEDAD
            // EQUATOR
            (Season::Summer, 0..=25) => 5.0,
            (Season::Winter, 0..=25) => 5.0,
            (Season::Fall, 0..=25) => 5.0,
            (Season::Spring, 0..=25) => 5.0,
            // TEMPERATE
            (Season::Summer, 25..=55) => 25.0,
            (Season::Winter, 25..=55) => 10.0,
            (Season::Fall, 25..=55) => 13.0,
            (Season::Spring, 25..=55) => 15.0,

            // SUB-POLAR (HUMID)
            (Season::Winter, 55..=90) => 6.0,
            (Season::Summer, 55..=90) => 12.0,
            (Season::Fall, 55..=90) => 7.0,
            (Season::Spring, 55..=90) => 10.0,
            // WILDCARD
            (Season::Winter, _) => 0.0,
            (Season::Summer, _) => 0.0,
            (Season::Fall, _) => 0.0,
            (Season::Spring, _) => 0.0,
        };

        let seasonal_factor_hum = match season {
            Season::Fall => HumidDry::None,
            Season::Spring => HumidDry::None,
            Season::Summer => climate.summer.0,
            Season::Winter => climate.winter.0
        };

        let range = match seasonal_factor_hum {
            HumidDry::None => 0..season_factor as i32,
            HumidDry::Dry => (season_factor * (4.0 / 6.0)) as i32..season_factor as i32,
            HumidDry::Humid => 0..(season_factor * (2.0 / 6.0)) as i32
        };

        let mut rng = thread_rng();
        rng.gen_range(range) as f32
    }
    

    pub fn gen_year_data(
        component: &mut GridComponent,
        latitude: i32,
        altitude: real,
        index: (i32, i32, i32),
        noise: Fbm<Simplex>,
        climate: Climate
    ) -> GenData {
        let mut temperature_vec: Vec<real> = vec![];
        let mut pressure_vec: Vec<real> = vec![];
        let mut wind_vec: Vec<(f32, f32, f32)> = vec![];
        let mut hum_vec: Vec<real> = vec![];
        let mut td_vec: Vec<real> = vec![];
        let mut current_season = Season::Winter;

        let night_variation_range = match latitude {
            55..=90 => SUB_POLAR_TEMP_RANGE,
            -90..=-55 => SUB_POLAR_TEMP_RANGE,
            25..=55 => TEMPERATE_TEMP_RANGE,
            -55..=-25 => TEMPERATE_TEMP_RANGE,
            -25..=25 => EQUATOR_TEMP_RANGE,
            _ => 0..1,
        };

        let curve: Gd<Curve> = load("resources/diurnal_temp_curve.tres");

        let mut rng = thread_rng();

        for day in 1..=360 {
            match day {
                1..=90 => current_season = Season::Winter,
                91..=180 => current_season = Season::Spring,
                181..=270 => current_season = Season::Summer,
                271..=360 => current_season = Season::Fall,
                _ => current_season = Season::Winter,
            }

            let base_temp_range = match current_season {
                Season::Fall => climate.fall.clone(),
                Season::Spring => climate.spring.clone(),
                Season::Summer => climate.summer.1.clone(),
                Season::Winter => climate.winter.1.clone()
            };

            let night_val = rng.gen_range(night_variation_range.clone());
            let base_temp = rng.gen_range(base_temp_range);

            // ---- TEMPERATURE ---- //

            let day_temp = Self::calculate_temperature(
                index,
                noise.clone(),
                altitude,
                base_temp,
                Vector3::new(0.1, 0.1, 0.1),
            );

            let night_temp = day_temp - (night_val as f32);

            for hour in 1..=24 {
                let factor = curve.sample(0.042 * hour as f32); // 0.042 -> 1h
                let temp = (day_temp * factor) + (night_temp * (1.0 - factor));
                temperature_vec.push(temp);
            }

            // ---- PRESSURE ---- //

            for hour in 1..=24 {
                let index = hour * day;
                let temp_value = temperature_vec.get(index - 1).unwrap();
                let pres = Self::calculate_pressure(altitude, *temp_value, 1013.25);
                pressure_vec.push(pres);
            }

            // ---- WIND ---- //

            for hour in 1..=24 {
                let index = hour * day;

                let pressure = pressure_vec.get(index - 1).unwrap();
                let wind = (
                    pressure.to_degrees().cos(),
                    pressure.to_degrees().sin(),
                    pressure.to_degrees().cos()
                );
                wind_vec.push(wind);
            }

            let seasonal_factor = Self::seasonal_factor(latitude, current_season, climate.clone());

            for hour in 1..=24 {
                // let water = altitude <= -0.6;
                let rel = Self::calculate_rel_hum(
                    *temperature_vec.get((hour * day) - 1).unwrap(),
                    seasonal_factor,
                    component.td
                );
                hum_vec.push(rel.0);
                td_vec.push(rel.1);
            }
        }
        GenData {
            index,
            temperature: temperature_vec,
            altitude,
            pressure: pressure_vec,
            humidity: hum_vec,
            wind: wind_vec,
            td: td_vec,
        }
    }

    pub fn save_data(data: GenData) -> std::io::Result<()> {
        let file_name = format!("data/weather_grid_data/{}_{}_{}.ron", data.index.0, data.index.1, data.index.2);
        let mut file = File::create(file_name)?;
        file.write_all(ron::ser::to_string_pretty(&data, PrettyConfig::default()).unwrap().as_ref())
    }
}

