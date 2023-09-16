use crate::{GreyscaleImage, Map};
use nalgebra::Vector3;

const STANDARD_NORMAL: Vector3<f64> = Vector3::new(0.0, 0.0, 1.0);

/// It calculates the soil depth at every point of the terrain. It uses the previously calculated angles (steepness).
/// :param map: Object of Map class. Used to get the max soil depth.
/// :param size: Integer. Size of the terrain. Used to initialize the image.
/// :param angles: 3D-List of all normal vectors.
/// :return: angles: List all angles on the terrain.
pub fn calculate_soil_depth(normal_map: &GreyscaleImage<Vector3<f64>>, map: &Map) -> GreyscaleImage<f64> {
    GreyscaleImage::new(
        normal_map
            .image
            .iter()
            .map(|normal| {
                let dot_product = STANDARD_NORMAL.dot(normal);
                (dot_product / (STANDARD_NORMAL.norm() * normal.norm())).acos().to_degrees()
            })
            .map(|angle| (1.0 - angle / 90.0) * map.max_soil_depth)
            .collect(),
    )
}
