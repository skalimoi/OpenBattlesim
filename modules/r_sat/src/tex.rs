/*
//// NOTE ////
Original implementation in Python by OttoLink: https://github.com/otto-link/SatColorLUT
All credits to him. I just had it translated to Rust and adapted it to my needs.
*/

use self::sat_lut::TerrainTextureActor;

pub mod sat_lut {
    use glob::glob;
use image::io::Reader as ImageReader;
use image::ImageBuffer;
use image::Rgb;
use ndarray::ArrayView;
use ndarray_ndimage::gaussian_filter;
use godot::prelude::*;
use godot::prelude::godot_api;
use std::fs;
use rand::thread_rng;
use rand::prelude::IteratorRandom;

const CLUT_RES: usize = 128;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct TerrainTextureActor {
    #[base]
    pub base: Base<Node>
}

#[godot_api]
impl TerrainTextureActor {
fn gradient(values: &Vec<f64>, width: usize) -> (Vec<f64>, Vec<f64>) {
    let height = values.len() / width;
    (
        (0..height)
            .into_iter()
            .map(|y| {
                (0..width).into_iter().map(move |x| {
                    if y == 0 {
                        values[width + x] - values[x]
                    } else if y == height - 1 {
                        values[y * width + x] - values[(y - 1) * width + x]
                    } else {
                        (values[(y + 1) * width + x] - values[(y - 1) * width + x]) / 2.0
                    }
                })
            })
            .flatten()
            .collect(),
        (0..height)
            .into_iter()
            .map(|y| {
                (0..width).into_iter().map(move |x| {
                    if x == 0 {
                        values[y * width + 1] - values[y * width]
                    } else if x == width - 1 {
                        values[y * width + x] - values[y * width + x - 1]
                    } else {
                        (values[y * width + x + 1] - values[y * width + x - 1]) / 2.0
                    }
                })
            })
            .flatten()
            .collect(),
    )
}


#[func]
    pub fn choose_and_copy_biome() {
    
        let directory_path = "r_tex/unselected_data";
        let name_str = "KOPPEN_CFC";
    
        let folder_data = fs::read_dir(directory_path).expect("Failure reading climate dir!");
    
        let mut rng = thread_rng();
    
        let mut vec = vec![];
    
        for entry in folder_data {
            if let Ok(entry) = entry {
                if entry.file_type().unwrap().is_dir() {
                    let folder_name = entry.file_name();
                    let folder_name_str = folder_name.to_string_lossy();
    
                    if folder_name_str.contains(name_str) {
                        vec.push(entry);
                    }
                }
            }
        }
    
        
        use fs_extra::dir::CopyOptions;
        let chosen = vec.iter().choose(&mut rng).unwrap();
        fs_extra::copy_items(
            &[chosen.path().as_path()],
            "r_erosion/data",
            &CopyOptions::new(),
        )
        .expect("Error copying climate files!");
    
    
    }

#[func]
pub fn create_texture(climate: GodotString) {
    let directory_path = "data/climate_sat_data";
    let name_str = climate.to_string();
    let name = name_str.as_str();
    let folder_data = fs::read_dir(directory_path).expect("Failure reading climate dir!");

    let mut rng = thread_rng();
    
    let mut vec = vec![];

    for entry in folder_data {
        if let Ok(entry) = entry {
            if entry.file_type().unwrap().is_dir() {
                let folder_name = entry.file_name();
                let folder_name_str = folder_name.to_string_lossy();

                if folder_name_str.contains(name) {
                    vec.push(entry);
                }
            }
        }
    }

    let chosen = vec.iter().choose(&mut rng).unwrap();

    let path = chosen.path();
    let path_string = path.to_string_lossy();

    let dem_path = format!("{}/dem.png", path_string);
    let aerial_path = format!("{}/aerial.png", path_string);

    // ------------- Generate -------------

    println!("Generating texture...");

    let dem_img = ImageReader::open(&dem_path).unwrap().decode().unwrap().into_luma16();
    let dem_img_ref = &dem_img;
    let (width, height) = dem_img.dimensions();

    let z: Vec<f64> = (0..height)
        .into_iter()
        .map(|y| (0..width).into_iter().map(move |x| dem_img_ref.get_pixel(x, y).0[0] as f64))
        .flatten()
        .collect();
    let (dzx, dzy) = Self::gradient(&z, width as usize);

    let img_col = ImageReader::open(&aerial_path)
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb8()
        .into_raw();

    let x_iter = Self::linspace(*z.iter().min_by(Self::f64_cmp).unwrap(), *z.iter().max_by(Self::f64_cmp).unwrap(), CLUT_RES);
    let y_iter: &Vec<f64> =
        &Self::linspace(*dzx.iter().min_by(Self::f64_cmp).unwrap(), *dzx.iter().max_by(Self::f64_cmp).unwrap(), CLUT_RES).collect();
    let z_iter: &Vec<f64> =
        &Self::linspace(*dzy.iter().min_by(Self::f64_cmp).unwrap(), *dzy.iter().max_by(Self::f64_cmp).unwrap(), CLUT_RES).collect();

    let img_coordinates: Vec<(usize, [f64; 3])> = z
        .into_iter()
        .zip(dzx.into_iter().zip(dzy.into_iter()))
        .map(|(x, (y, z))| [x, y, z])
        .enumerate()
        .collect();

    let tree = kd_tree::KdTree3::build_by_key(img_coordinates, |item, k| ordered_float::OrderedFloat(item.1[k]));
    let (red, (green, blue)): (Vec<f64>, (Vec<f64>, Vec<f64>)) = x_iter
        .map(|x| y_iter.iter().map(move |y| z_iter.iter().map(move |z| [x, *y, *z])))
        .flatten()
        .flatten()
        .map(|coord| tree.nearest_by(&coord, |item, k| item.1[k]).unwrap().item.0)
        .map(|p| (img_col[p * 3] as f64, (img_col[p * 3 + 1] as f64, img_col[p * 3 + 2] as f64)))
        .unzip();

    let red = gaussian_filter(
        &ArrayView::from_shape((CLUT_RES, CLUT_RES, CLUT_RES), &red).unwrap(),
        1.0,
        0,
        ndarray_ndimage::BorderMode::Reflect,
        4,
    )
    .into_raw_vec();
    let green = gaussian_filter(
        &ArrayView::from_shape((CLUT_RES, CLUT_RES, CLUT_RES), &green).unwrap(),
        1.0,
        0,
        ndarray_ndimage::BorderMode::Reflect,
        4,
    )
    .into_raw_vec();
    let blue = gaussian_filter(
        &ArrayView::from_shape((CLUT_RES, CLUT_RES, CLUT_RES), &blue).unwrap(),
        1.0,
        0,
        ndarray_ndimage::BorderMode::Reflect,
        4,
    )
    .into_raw_vec();

    // ------------- Apply -------------

    println!("Applying...");

    let eroded_img: ImageBuffer<image::Luma<u16>, Vec<u16>> = ImageReader::open("data/raw/texturing_buffer.png")
        .unwrap()
        .decode()
        .unwrap()
        .into_luma16();
    let eroded_img_ref = &eroded_img;
    let (width, height) = eroded_img.dimensions();

    let z: Vec<f64> = (0..height)
        .into_iter()
        .map(|y| {
            (0..width)
                .into_iter()
                .map(move |x| eroded_img_ref.get_pixel(x, y).0[0] as f64)
        })
        .flatten()
        .collect();
    let (dzx, dzy) = Self::gradient(&z, width as usize);

    let mx = *z.iter().min_by(Self::f64_cmp).unwrap();
    let my = *dzx.iter().min_by(Self::f64_cmp).unwrap();
    let mz = *dzy.iter().min_by(Self::f64_cmp).unwrap();

    let dx = (z.iter().max_by(Self::f64_cmp).unwrap() - mx) / (CLUT_RES - 1) as f64;
    let dy = (dzx.iter().max_by(Self::f64_cmp).unwrap() - my) / (CLUT_RES - 1) as f64;
    let dz = (dzy.iter().max_by(Self::f64_cmp).unwrap() - mz) / (CLUT_RES - 1) as f64;

    let raw_image: Vec<u8> = z
        .into_iter()
        .zip(dzx.into_iter().zip(dzy.into_iter()))
        .map(|(x, (y, z))| [x, y, z])
        .into_iter()
        .map(|coord| {
            ((coord[0] - mx) / dx).round() as usize * CLUT_RES * CLUT_RES
                + ((coord[1] - my) / dy).round() as usize * CLUT_RES
                + ((coord[2] - mz) / dz).round() as usize
        })
        .flat_map(|p| [red[p] as u8, green[p] as u8, blue[p] as u8])
        .collect();

    println!("Saving...");
    let img = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_vec(width as u32, height as u32, raw_image).unwrap();
    img.save("data/raw/texture.png").unwrap();
}

pub fn f64_cmp(x: &&f64, y: &&f64) -> std::cmp::Ordering {
    x.partial_cmp(y).unwrap()
}

pub fn meshgrid(from: [&Vec<f64>; 3]) -> Vec<[f64; 3]> {
    let x_iter = Self::linspace(*from[0].iter().min_by(Self::f64_cmp).unwrap(), *from[0].iter().max_by(Self::f64_cmp).unwrap(), CLUT_RES);
    let y_iter: &Vec<f64> =
        &Self::linspace(*from[1].iter().min_by(Self::f64_cmp).unwrap(), *from[1].iter().max_by(Self::f64_cmp).unwrap(), CLUT_RES).collect();
    let z_iter: &Vec<f64> =
        &Self::linspace(*from[2].iter().min_by(Self::f64_cmp).unwrap(), *from[2].iter().max_by(Self::f64_cmp).unwrap(), CLUT_RES).collect();

    x_iter
        .map(|x| y_iter.iter().map(move |y| z_iter.iter().map(move |z| [x, *y, *z])))
        .flatten()
        .flatten()
        .collect()
}

pub fn linspace(x0: f64, xend: f64, n: usize) -> impl Iterator<Item = f64> {
    let to_float = |i: usize| i as f64;
    let dx = (xend - x0) / to_float(n - 1);
    (0..n).map(move |i| x0 + to_float(i) * dx)
}

}
}

use godot::prelude::*;
#[godot_api]
impl NodeVirtual for TerrainTextureActor {
    fn init(base: Base<Node>) -> Self {
        TerrainTextureActor {
            base
        }
    }

    fn ready(&mut self) {
        
    }
}