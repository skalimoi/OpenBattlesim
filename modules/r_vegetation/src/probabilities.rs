use std::collections::HashMap;
use crate::config::{GreyscaleImage, Vegetation};

pub fn calculate_probability(needed: f64, available: f64) -> f64 {
    if available <= needed {
        available / needed
    } else if available <= needed * 2.0 {
        1.0 - (available / needed - 1.0)
    } else {
        0.0
    }
}

pub fn calculate_insolation_probability(vegetation: &Vegetation, insolation: f64) -> f64 {
    calculate_probability(vegetation.energy_demand, insolation)
}

pub fn calculate_soil_demand_probability(soil_demand_id: u8, soil_id: u8) -> f64 {
    if soil_demand_id == soil_id {
        1.0
    } else {
        0.0
    }
}

pub fn calculate_soil_depth_probability(vegetation: &Vegetation, edaphic: f64) -> f64 {
    (edaphic / vegetation.soil_depth_demand).min(1.0)
}

pub fn calculate_water_demand_probability(vegetation: &Vegetation, hydrology: f64) -> f64 {
    calculate_probability(vegetation.water_demand, hydrology)
}

pub fn calculate_probabilities(
    vegetation: &Vegetation,
    soil_ids_map: &GreyscaleImage<u8>,
    soil_names: &HashMap<String, u8>,
    insolation_map: &GreyscaleImage<f64>,
    edaphic_map: &GreyscaleImage<f64>,
    hydrology_map: &GreyscaleImage<f64>,
) -> GreyscaleImage<f64> {
    let mut reasons_for_not_growing = [0, 0, 0, 0];
    let probabilities = GreyscaleImage::new(
        (0..insolation_map.len())
            .into_iter()
            .map(|y| {
                (0..insolation_map.len()).into_iter().map(move |x| {
                    let probability = [
                        calculate_soil_demand_probability(soil_names[&vegetation.soil_demand], soil_ids_map[(x, y)]),
                        calculate_insolation_probability(vegetation, insolation_map[(x, y)]),
                        calculate_soil_depth_probability(vegetation, edaphic_map[(x, y)]),
                        calculate_water_demand_probability(vegetation, hydrology_map[(x, y)]),
                    ]
                    .into_iter()
                    .enumerate()
                    .min_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
                    .unwrap();
                    probability
                })
            })
            .flatten()
            .map(|(idx, prob)| {
                if prob == 0.0 {
                    reasons_for_not_growing[idx] += 1
                }
                prob
            })
            .collect(),
    );
    let mut location_factor_with_max_reasons_for_not_growing =
        reasons_for_not_growing[1..].iter().enumerate().max_by_key(|x| x.1).unwrap();
    if location_factor_with_max_reasons_for_not_growing.1 == &reasons_for_not_growing[1] {
        location_factor_with_max_reasons_for_not_growing = (3, &0)
    }
    println!(
        "Main reason for not growing (except soil demand): {}",
        ["insolation", "soil depth", "water demand", "only soil demand"][location_factor_with_max_reasons_for_not_growing.0]
    );
    probabilities
}
