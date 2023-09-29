use nalgebra::Vector3;
use crate::config::{clamp_idx, GreyscaleImage, Map, round, SimArgs, Sun};

const SOLAR_CONSTANT_K_CALORIES_PER_HOUR: f64 = 1200.0;

/// Calculates if a given point on the terrain receives light at a given daylight hour. Atmospheric absorption etc.
/// is not considered during this calculation.
/// :param sun: Object of the sun class. Used for calculating from which direction the sun shines.
/// :param x, y: Integer of the x and y position of the point on the terrain.
/// :param map_size: Integer of the size of the terrain.
/// :param pixel_size: Float. The size a pixel represents of the real terrain.
/// :param heightmap_max_height: Integer. Maximal height of the terrain.
/// :param height_conversion: Float. Conversion value of the height of the heightmap to calculate the real height.
pub fn calculate_raw_insolation(
    insolation_map: &mut GreyscaleImage<f64>,
    height_map: &GreyscaleImage<f64>,
    sun: &Sun,
    x: usize,
    y: usize,
    map: &Map,
    heightmap_max_height: f64,
) {
    let step = sun.convert_to_uv_coordinates(); // transforms the polar coordinates of the sun to cartesian coordinates
    let start_world_pos = Vector3::new(x as f64, y as f64, height_map[(x, y)]).component_mul(&Vector3::new(
        map.pixel_size,
        map.pixel_size,
        map.height_conversion,
    ));
    let mut real_world_pos = start_world_pos;
    let mut t = 0.0; // used for the vector equation
    let mut sun_beam_reaches_pixel = true;
    let map_x_y_boundary = height_map.len() as f64 * map.pixel_size; // boundary of the map

    while 0.0 <= real_world_pos.x
        && real_world_pos.x < map_x_y_boundary
        && 0.0 <= real_world_pos.y
        && real_world_pos.y < map_x_y_boundary
        && real_world_pos.z < heightmap_max_height
    {
        // this if statement decides how far the sun direction vector will be followed until a new
        // pixel in the pixel space will be reached. Only then a new height can be compared. This accelerates the
        // algorithm.
        t += if step.x != 0.0 || step.y != 0.0 {
            if real_world_pos.x % map.pixel_size >= real_world_pos.y % map.pixel_size && step.x != 0.0 {
                (map.pixel_size - (real_world_pos.x % map.pixel_size)) / step.x
            } else {
                (map.pixel_size - (real_world_pos.y % map.pixel_size)) / step.y
            }
        } else {
            break; // sun stands in zenith so every pixel will receive light
        }
        .abs();

        real_world_pos = start_world_pos + t * step;
        let pixel_x = (real_world_pos.x / map.pixel_size) as i32;
        let pixel_y = (real_world_pos.y / map.pixel_size) as i32;

        if pixel_x < 0 || pixel_y < 0 || pixel_x > (height_map.len() - 1) as i32 || pixel_y > (height_map.len() - 1) as i32 {
            break; // sun beam leaves the map boundary
        }

        if height_map[(pixel_x as usize, pixel_y as usize)] * map.height_conversion > real_world_pos.z {
            sun_beam_reaches_pixel = false;
            break; // something blocks the light from the sun for that pixel
        }
    }
    if sun_beam_reaches_pixel {
        insolation_map[(x, y)] += SOLAR_CONSTANT_K_CALORIES_PER_HOUR // * (map.pixel_size ** 2)
    }
}

/// Adds the energy of the neighbours of a pixel and calculates the average. A fraction of this number will
/// be added to the currently observed pixel.
/// :param reflection_coefficient: Float. Fraction of the average energy of the neighbourspixels that the pixel will receive.
fn add_reflection_insolation(insolation_map: &mut GreyscaleImage<f64>, reflection_coefficient: f64) {
    let len = insolation_map.len();
    for y in 0..len {
        println!("Reflection: Row: {y}");
        for x in 0..len {
            let neighbor_insolation_sum: f64 = [-1, 0, 1]
                .into_iter()
                .map(|offset_x| {
                    [-1, 0, 1]
                        .into_iter()
                        .map(|offset_y| {
                            if !(offset_x == 0 && offset_y == 0) {
                                insolation_map[(clamp_idx(x, offset_x, len - 1), clamp_idx(y, offset_y, len - 1))]
                            } else {
                                0.0
                            }
                        })
                        .sum::<f64>()
                })
                .sum();
            insolation_map[(x, y)] += neighbor_insolation_sum / 8.0 * reflection_coefficient
        }
    }
}

/// Calculates the actual energy of each pixel based on the previously calculated raw energy. The atmosphere and
/// reflection reduce the raw energy.
/// :param map_name: String of the current map name.
/// :param daylight_hours: Integer of the number of daylight hours.
/// :param sun_start_elevation: Float of the start elevation of the sun.
/// :param sun_start_azimuth: Float of the start azimuth of the sun.
/// :param sun_max_elevation: Float of the maximal sun elevation (noon).
/// :param reflection_coefficient: Float of the reflection coeficient. It states how much light of the neighbour pixel will be reflected.
/// :return: insolation_image: Image of the calculated actual energy of each pixel.
pub fn calculate_actual_insolation(sim_args: &SimArgs) -> GreyscaleImage<f64> {
    let mut insolation_map = calculate_insolation_for_daylight_hours(&sim_args);
    for y in 0..sim_args.height_map.len() {
        for x in 0..sim_args.height_map.len() {
            let pixel_raw_insolation = insolation_map[(x, y)];
            let cloud_reflection_loss = pixel_raw_insolation * sim_args.biom.cloud_reflection / 100.0;
            let atmospheric_absorption_loss = pixel_raw_insolation * sim_args.biom.atmospheric_absorption / 100.0;
            let atmospheric_diffusion_loss = pixel_raw_insolation * sim_args.biom.atmospheric_diffusion / 100.0;
            let soil = &sim_args.soils[&sim_args.soil_ids_map[(x, y)]];
            insolation_map[(x, y)] =
                (pixel_raw_insolation - cloud_reflection_loss - atmospheric_absorption_loss - atmospheric_diffusion_loss)
                    * (1.0 - soil.albedo)
        }
    }
    add_reflection_insolation(&mut insolation_map, sim_args.reflection_coefficient);
    insolation_map
}

/// Calculates the sun position for every day light hours. At each hour the raw energy for each pixel wil be
/// calculated.
/// :param map_name: String of the current map name.
/// :param daylight_hours: Integer of the number of daylight hours.
/// :param sun_start_elevation: Float of the start elevation of the sun.
/// :param sun_start_azimuth: Float of the start azimuth of the sun.
/// :param sun_max_elevation: Float of the maximal sun elevation (noon).
fn calculate_insolation_for_daylight_hours(sim_args: &SimArgs) -> GreyscaleImage<f64> {
    assert!(sim_args.sun_config.daylight_hours > 0, "Daylight hours must be at least one!");

    // elevation
    let elevation_per_hour = match sim_args.sun_config.daylight_hours {
        1 => 0.0,
        2 => sim_args.sun_config.sun_max_elevation - sim_args.sun_config.sun_start_elevation,
        _ if sim_args.sun_config.daylight_hours % 2 == 1 => {
            (sim_args.sun_config.sun_max_elevation - sim_args.sun_config.sun_start_elevation)
                / ((sim_args.sun_config.daylight_hours as f64 - 2.0) / 2.0).ceil()
        } // the sun shall rise to 90 (or less) degrees till noon and then fall again
        _ => {
            (sim_args.sun_config.sun_max_elevation - sim_args.sun_config.sun_start_elevation)
                / (sim_args.sun_config.daylight_hours as f64 / 2.0)
        }
    };
    let azimuth_per_hour = match sim_args.sun_config.daylight_hours {
        1 => 0.0,
        2 => 180.0 - 2.0 * sim_args.sun_config.sun_start_azimuth,
        _ => 180.0 / (sim_args.sun_config.daylight_hours as f64 - 1.0), // the sun shall wander 180 degrees
    };

    let mut sun = Sun {
        azimuth: sim_args.sun_config.sun_start_azimuth,
        elevation: sim_args.sun_config.sun_start_elevation,
    };
    let mut insolation_map = GreyscaleImage::new(vec![0.0; sim_args.height_map.len() * sim_args.height_map.len()]);

    let max_height = sim_args
        .height_map
        .image
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap()
        * sim_args.map.height_conversion;
    for hour in 0..sim_args.sun_config.daylight_hours {
        println!("############ Hour: {} ############", hour + 1);
        println!("Sun polar coordinates: Azimuth: {}° Elevation: {}°", round(sun.azimuth, 1), round(sun.elevation, 1));
        for x in 0..sim_args.height_map.len() {
            //println!("Raw Insolation: Row: {x}");
            for y in 0..sim_args.height_map.len() {
                calculate_raw_insolation(&mut insolation_map, &sim_args.height_map, &sun, x, y, sim_args.map, max_height);
            }
        }

        if sim_args.sun_config.daylight_hours % 2 == 1 {
            if hour < sim_args.sun_config.daylight_hours / 2 {
                sun.elevation += elevation_per_hour
            } else {
                sun.elevation -= elevation_per_hour
            }
        } else {
            if hour != sim_args.sun_config.daylight_hours / 2 - 1 {
                if hour < sim_args.sun_config.daylight_hours / 2 {
                    sun.elevation += elevation_per_hour
                } else {
                    sun.elevation -= elevation_per_hour;
                }
            }
        };
        sun.azimuth += azimuth_per_hour
    }
    insolation_map
}
