
use nalgebra::Vector3;
use crate::config::{clamp_idx, GreyscaleImage, SimArgs};

/// Calculates all normal vector of a map. It needs the previously calculated vertex list. For calculating the
/// normal of a vertex all neighbour normal will be calculated, added up and normalized. Each vertex has six
/// neighbour faces, which have a normal vector. These normals have to be calculated. The calculation is done by
/// determining the direction vectors of three vertices (the oberserved vertex + 2 neighbour vertices). The
/// cross product of the two directions vectors will be calculated resulting in the normal of that surface.
/// :param map: Object of the map class. Used for creating the vertex list.
/// :param image_height_map: Image of the height map. Used for creating the vertex list.
/// :return: normals: List of all calculated normals of each pixel.
pub fn calculate_normal_map(sim_args: &SimArgs) -> GreyscaleImage<Vector3<f64>> {
    let len = sim_args.height_map.len();
    GreyscaleImage::new(
        (0..len)
            .into_iter()
            .map(|y| {
                (0..len).into_iter().map(move |x| {
                    [
                        [(0, -1), (-1, -1)],
                        [(-1, -1), (-1, 0)],
                        [(-1, 0), (-1, 1)],
                        [(-1, 1), (0, 1)],
                        [(0, 1), (1, 1)],
                        [(1, 1), (1, 0)],
                        [(1, 0), (1, -1)],
                        [(1, -1), (0, -1)],
                    ]
                    .into_iter()
                    .map(|offset| {
                        let v1 = Vector3::new(
                            offset[0].0 as f64 * sim_args.map.pixel_size,
                            offset[0].1 as f64 * sim_args.map.pixel_size,
                            (sim_args.height_map[(clamp_idx(x, offset[0].0, len - 1), clamp_idx(y, offset[0].1, len - 1))]
                                - sim_args.height_map[(x, y)])
                                * sim_args.map.height_conversion,
                        );
                        let v2 = Vector3::new(
                            offset[1].0 as f64 * sim_args.map.pixel_size,
                            offset[1].1 as f64 * sim_args.map.pixel_size,
                            (sim_args.height_map[(clamp_idx(x, offset[1].0, len - 1), clamp_idx(y, offset[1].1, len - 1))]
                                - sim_args.height_map[(x, y)])
                                * sim_args.map.height_conversion,
                        );
                        v2.cross(&v1)
                    })
                    .sum::<Vector3<f64>>()
                    .normalize()
                })
            })
            .flatten()
            .collect(),
    )
}
