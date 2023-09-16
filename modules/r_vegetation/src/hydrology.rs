use crate::{GreyscaleImage, SimArgs};

const K_CALORIES_NEEDED_TO_EVAPORATE_1_G_WATER: f64 = 0.54;

/// Calculates the available water at every point on the terrain. It uses the average rainfall and groundwater from
/// the biom. The water absorption and depth of the soil is also considered and the evaporation by the sun is calculated.
/// :param map: Object of the map class.
/// :param edaphic_map: Object of the Edaphology class. Used to get the soil depth.
/// :param soil_ids_map: Map of the soil ids. Used to get the water absorption of the soil.
/// :param image_insolation_map: Result of the insolation calculation. Used for the calculation of the evaporation.
/// :param biom: Object of the biom class. Used to get the groundwater and rainfall values.
/// :return: hydrology_map: Result of water calculations.
pub fn calculate_hydrology_map(
    sim_args: &SimArgs,
    edaphic_map: &GreyscaleImage<f64>,
    insolation_map: &GreyscaleImage<f64>,
) -> GreyscaleImage<f64> {
    GreyscaleImage::new(
        (0..edaphic_map.len())
            .into_iter()
            .map(|y| {
                (0..edaphic_map.len()).into_iter().map(move |x| {
                    let soil = &sim_args.soils[&sim_args.soil_ids_map[(x, y)]];
                    let depth_coefficient = (edaphic_map[(x, y)] / 100.0).min(1.0);
                    let water_supply = (sim_args.biom.groundwater + sim_args.biom.avg_rainfall_per_day)
                        * depth_coefficient
                        * soil.water_absorption;
                    let evaporated_water = (insolation_map[(x, y)] * K_CALORIES_NEEDED_TO_EVAPORATE_1_G_WATER) / 1000.0;
                    (water_supply - evaporated_water).max(0.0)
                })
            })
            .flatten()
            .collect(),
    )
}
