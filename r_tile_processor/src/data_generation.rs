/* use godot::prelude::*;
use noise::{Fbm, Simplex};
use std::ops::Range;
use rand::{thread_rng, Rng};

use crate::{Season, Scenario};

pub const EQUATOR_TEMP_RANGE: Range<f32> = 0.0..3.0;
pub const TEMPERATE_TEMP_RANGE: Range<f32> = 7.0..10.0;
pub const SUB_POLAR_TEMP_RANGE: Range<f32> = 12.0..25.0;

pub const EQUATOR_BASE_TEMPS: Range<f32> = 25.0..35.0;
pub const TEMPERATE_BASE_TEMPS: Range<f32> = 15.0..20.0;
pub const SUB_POLAR_BASE_TEMPS: Range<f32> = 0.0..15.0;

pub const SUB_POLAR_NORTH_LATITUDES: Range<i32> = 55..90;
pub const SUB_POLAR_SOUTH_LATITUDES: Range<i32> = -90..-55;
pub const TEMPERATE_NORTH_LATITUDES: Range<i32> = 25..55;
pub const TEMPERATE_SOUTH_LATITUDES: Range<i32> = -55..-25;
pub const EQUATOR_LATITUDES: Range<i32> = -25..25;

pub struct GenData {
    pub index: Vector3,
    pub temperature: Vec<real>,
    pub altitude: real,
    pub pressure: Vec<real>,
    pub humidity: Vec<f32>,
    pub wind: Vec<Vector3>,
    pub dew_point: Vec<real>
}

impl GenData {
    fn calculate_seasonal_temperature(latitude: i32) -> (real, real, real, real) {
        let mut rng = thread_rng();
        let spring_base_temp = match latitude {
            -25..25 => rng.gen_range(25.0..35.0),
            25..55 => rng.gen_range(15.0..20.0),
            -55..-25 => rng.gen_range(15.0..20.0),
            55..90 => rng.gen_range(0.0..15.0),
            -90..-55 => rng.gen_range(0.0..15.0),
            _ => 0.0,
        };
        let winter_base_temp = match latitude {
            -25..25 => rng.gen_range(15.0..=22.0),
            25..55 => rng.gen_range(0.0..=10.0),
            -55..-25 => rng.gen_range(0.0..=10.0),
            55..90 => rng.gen_range(-25.0..=0.0),
            -90..-55 => rng.gen_range(-25.0..=0.0),
            _ => 0.0,
        };

        let fall_base_temp = match latitude {
            -25..25 => rng.gen_range(17.0..=25.0),
            25..55 => rng.gen_range(7.0..=15.0),
            -55..-25 => rng.gen_range(7.0..=15.0),
            55..90 => rng.gen_range(-10.0..=5.0),
            -90..-55 => rng.gen_range(-10.0..=5.0),
            _ => 0.0,
        };
        let summer_base_temp = match latitude {
            -25..25 => rng.gen_range(25.0..=40.0),
            25..55 => rng.gen_range(20.0..=35.0),
            -55..-25 => rng.gen_range(21.0..=35.0),
            55..90 => rng.gen_range(5.0..=17.0),
            -90..-55 => rng.gen_range(5.0..=17.0),
            _ => 0.0,
        };
        (
            spring_base_temp,
            winter_base_temp,
            fall_base_temp,
            summer_base_temp,
        )
    }

    pub fn generate_year_data(/* curve */ latitude: i32, season: Season, altitude: real, index: Vector3, noise: Fbm<Simplex>) -> GenData {
        let mut rng = thread_rng();
        let mut temperature_vec: Vec<real> = vec![];
        let mut pressure_vec: Vec<real> = vec![];
        let mut wind_vec: Vec<Vector3> = vec![];
        let mut hum_vec: Vec<real> = vec![];
        let mut td_vec: Vec<real> = vec![];
        let mut current_season = Season::Winter;

        let night_variation_range = match latitude {
            55..90 => SUB_POLAR_TEMP_RANGE,
            -90..-55 => SUB_POLAR_TEMP_RANGE,
            25..55 => TEMPERATE_TEMP_RANGE,
            -55..-25 => TEMPERATE_TEMP_RANGE,
            -25..25 => EQUATOR_TEMP_RANGE,
            _ => 0.0..1.0,
        };

        for day in 1..=360 {
            // println!("DAY: {}", day);
            match day {
                1..=90 => current_season = Season::Winter,
                91..=180 => current_season = Season::Spring,
                181..=270 => current_season = Season::Summer,
                271..=360 => current_season = Season::Fall,
                _ => current_season = Season::Winter,
            }

            let base_temp_gen = GenData::calculate_seasonal_temperature(latitude);

            let type_val = 1; // TODO cambiar la variación según la elección de climate     //set_season_type(current_season.clone());

            // if index.y == 1 { println!("SEASONAL TEMPERATURE IS: \n SPRING: {} \n WINTER: {} \n FALL: {} \n SUMMER: {}", base_temp_gen.0, base_temp_gen.1, base_temp_gen.2, base_temp_gen.3);
            // }

            let night_val = rng.gen_range(night_variation_range.clone());

            let base_temp = match current_season {
                Season::Spring => base_temp_gen.0,
                Season::Winter => base_temp_gen.1,
                Season::Fall => base_temp_gen.2,
                Season::Summer => base_temp_gen.3,
            };

            // println!("SEASON: {:?} \n BASE TEMP: {:?} \n DAY: {} \n ----------", current_season, base_temp, day);

            // ----- GENERACION DE TEMPERATURA ----- //

            let day_temp = Scenario::calculate_temperature(
                index,
                noise.clone(),
                altitude,
                base_temp,
                Vector3::new(0.1, 0.1, 0.1),
            );

            let night_temp = day_temp - (night_val); // TODO SEE IF IT WORKS

            for hour in 1..=24 {
                let factor = curve.value_at(hour as f32);
                let temp = (day_temp * factor) + (night_temp * (1.0 - factor));
                temperature_vec.push(temp);
            }

            // ----- GENERACION DE PRESIÓN ----- //

            for hour in 1..=24 {
                let index = hour * day;
                let temp_value = temperature_vec.get(index - 1).unwrap();
                let pres = Grid::calculate_pressure(altitude, *temp_value, 1013.25);
                pressure_vec.push(pres);
            }

            // ----- GENERACIÓN DE VIENTO ----- //

            for hour in 1..=24 {
                let index = hour * day;

                let pressure = pressure_vec.get(index - 1).unwrap();
                let wind = Vector3::new(
                    pressure.to_degrees().cos(),
                    pressure.to_degrees().sin(),
                    pressure.to_degrees().cos(),
                );
                wind_vec.push(wind);
            }

            // ----- GENERACIÓN DE HUMEDAD ----- //

            for hour in 1..=24 {
                let water = altitude <= -0.6;
                let seasonal_factor =
                    Grid::calculate_seasonal_factor(latitude, current_season.clone(), type_val);
                let rel = Grid::calculate_rel_hum(
                    *temperature_vec.get((hour * day) - 1).unwrap(),
                    seasonal_factor,
                    water,
                    component.rain_shadow_dry,
                    component.no_rain_shadow,
                    component.is_precipitating,
                    component.rain_shadow_hum,
                    component.td,
                );
                hum_vec.push(rel.0);
                td_vec.push(rel.1);
            }

        }
        GenData {
            index: component.index,
            temperature: temperature_vec,
            altitude: component.altitude,
            pressure: pressure_vec,
            humidity: hum_vec,
            wind: wind_vec,
            dew_point: td_vec,
        }

    }
    
} */