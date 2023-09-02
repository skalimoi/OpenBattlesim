extern crate image;
extern crate ndarray;
extern crate ndarray_rand;
extern crate ndarray_npy;
extern crate rand;
extern crate glob;

use image::{DynamicImage, GenericImage, GenericImageView};
use ndarray::{Array, Array1, Array2, Array3};
use ndarray_rand::RandomExt;
use rand::Rng;
use glob::glob;
use std::fs::File;
use std::io::Write;

fn main() {
    gen_clut();
    apply_clut_png();
}

fn gen_clut() {
    println!("Generating CLUT...");

    let clut_res = 32;
    let dir_list = glob("data/*").expect("Failed to read directories");

    for entry in dir_list {
        if let Ok(cmap_path) = entry {
            let cmap = cmap_path.file_name().unwrap().to_str().unwrap();
            println!("{}", cmap);

            let fname_dem = format!("data/{}/dem.png", cmap);
            let fname_col = format!("data/{}/aerial.png", cmap);

            let z = load_img(&fname_dem);
            let (dzx, dzy) = gradient(&z);
            let clut3 = generate_clut(&z, &dzx, &dzy, &fname_col, clut_res);

            np_save("clut_3.npy", &clut3);
        }
    }
}

fn load_img(fname: &str) -> Array2<u16> {
    let img = image::open(fname).expect("Failed to open image");
    let img = img.to_rgb();
    let mut result = Array2::zeros((img.width() as usize, img.height() as usize));

    for (x, y, pixel) in img.enumerate_pixels() {
        result[[x as usize, y as usize]] = pixel.data[0] as u16;
    }

    result
}

fn gradient(z: &Array2<u16>) -> (Array2<f64>, Array2<f64>) {
    let (dzx, dzy) = gradient_xy(z);
    (dzx, dzy)
}
fn gradient_xy(z: &Array2<u16>) -> (Array2<f64>, Array2<f64>) {
    let dx = ndarray::im::filter::spatial_gradient(z, 0.0, 1.0);
    let dy = ndarray::im::filter::spatial_gradient(z, 0.0, 1.0);
    (dx, dy)
}

fn gaussian_curvature(z: &Array2<u16>, sigma: f64) -> (Array2<f64>, Array2<f64>) {
    let z = ndarray::im::filter::gaussian_blur(z, sigma);
    let (zxx, zxy) = gradient_xy(&z);
    let (zxy, zyy) = gradient_xy(&zxy);

    let k = (zxx * zyy - (zxy * zxy)) / (1.0 + (zxx * zxx + zyy * zyy)).powf(2.0);
    let h = (zxx * (1.0 + zyy * zyy) - 2.0 * zxy * zxy * zxx * zyy + zyy * (1.0 + zxx * zxx)).sqrt()
        / (1.0 + zxx * zxx + zyy * zyy).powf(1.5);

    (k, h)
}

fn hillshade(z: &Array2<u16>, azimuth: f64, zenith: f64, talus_ref: f64) -> Array2<f64> {
    let azimuth_rad = std::f64::consts::PI * azimuth / 180.0;
    let zenith_rad = std::f64::consts::PI * zenith / 180.0;

    let aspect = gradient_angle(z);
    let dn = gradient_norm(z) / talus_ref;
    let slope = dn.atan();

    let sh = (zenith_rad.cos() * slope.cos()
        + zenith_rad.sin() * slope.sin() * (azimuth_rad - aspect).cos())
        .max(0.0);

    (sh - sh.min()).mapv(|x| x / sh.max() - sh.min())
}

fn gradient_angle(z: &Array2<u16>) -> Array2<f64> {
    let (dx, dy) = gradient_xy(z);
    dy.atan2(dx)
}

fn gradient_norm(z: &Array2<u16>) -> Array2<f64> {
    let (dx, dy) = gradient_xy(z);
    (dx * dx + dy * dy).mapv(f64::sqrt)
}

fn generate_clut(
    z: &Array2<u16>,
    dzx: &Array2<f64>,
    dzy: &Array2<f64>,
    fname_col: &str,
    clut_res: usize,
) -> Array3<u16> {
    let nfeatures = 3; // You can change this according to your needs

    // Load color image
    let img_col = load_img(fname_col);
    let img_col = img_col.mapv(|x| x as f64);

    // Initialize color LUT
    let mut clut = Array::from_elem((clut_res, clut_res, clut_res, 4), 65535u16);

    // Prepare grids and data
    let mut lspace = Vec::new();
    for i in 0..nfeatures {
        lspace.push(
            ndarray_stats::QuantileExt::quantile_axis(&z.mapv(|x| x as f64), clut_res, i).unwrap(),
        );
    }

    let mut Xg = Vec::new();
    for i in 0..nfeatures {
        Xg.push(lspace[i].into_shape((clut_res, 1)).unwrap());
    }

    let mut Xitp = Vec::new();
    for i in 0..nfeatures {
        Xitp.push(Xg[i].iter().cloned().collect::<Vec<_>>());
    }

    let mut X = Vec::new();
    for i in 0..nfeatures {
        X.push(z
            .iter()
            .zip(dzx.iter())
            .zip(dzy.iter())
            .map(|((&z_val, &dzx_val), &dzy_val)| (z_val as f64, dzx_val, dzy_val))
            .collect::<Vec<_>>());
    }

    let X = X.iter().map(|x| x.iter().map(|&(a, b, c)| (a, b, c)).collect::<Vec<_>>()).collect::<Vec<_>>();

    for k in 0..3 {
        let v = img_col.iter().map(|&x| x as f64).collect::<Vec<_>>();
        let fitp = ndarray_stats::QuantileExt::quantile_interp(X.iter().map(|x| x.iter().map(|&(a, _, _)| a).collect::<Vec<_>>()), &v, 4);
        let mut tk = fitp.predict(&Xitp.iter().map(|x| x.as_slice().unwrap()));
        tk = ndarray::im::filter::gaussian_blur(&tk.into_shape((clut_res, clut_res, clut_res)).unwrap(), 1.0);
        clut.indexed_iter_mut().for_each(|(index, c)| {
            let value = tk[index];
            *c = value as u16;
        });
    }

    clut
}

fn apply_clut(features: Vec<Array2<u16>>, clut: Array3<u16>) -> Array3<u16> {
    let nfeatures = features.len();
    let clut_shape = clut.shape();

    let mut lspace = Vec::new();
    for i in 0..nfeatures {
        lspace.push(
            ndarray_stats::QuantileExt::quantile_axis(&features[i].mapv(|x| x as f64), clut_shape[i], i)
                .unwrap(),
        );
    }

    let mut Xg = Vec::new();
    for i in 0..nfeatures {
        Xg.push(lspace[i].into_shape(clut_shape[i]).unwrap());
    }

    let mut Xitp = Vec::new();
    for i in 0..nfeatures {
        Xitp.push(features[i].iter().map(|&x| x as f64).collect::<Vec<_>>());
    }

    let Xitp = Xitp.iter().map(|x| x.as_slice().unwrap()).collect::<Vec<_>>();

    let mut img_out = Array::from_elem((features[0].shape()[0], features[0].shape()[1], 4), 65535u16);

    for k in 0..3 {
        let v = clut.slice(s![.., .., .., k]).iter().map(|&x| x as f64).collect::<Vec<_>>();
        let fitp = ndarray_stats::QuantileExt::quantile_interp(X.iter().map(|x| x.iter().map(|&a| a).collect::<Vec<_>>()), &v, 4);
        let mut img_data = fitp.predict(Xitp.iter().map(|x| x.as_slice().unwrap()));
        img_data = img_data.into_shape(features[0].shape()).unwrap();
        clut_shape.iter().enumerate().for_each(|(i, &size)| {
            if size > 1 {
                img_data.swap_axes(i, 0);
            }
        });
        clut.indexed_iter().zip(img_data.indexed_iter()).for_each(|((_, mut c), (_, &i))| {
            *c = i as u16;
        });
    }

    img_out
}

fn apply_clut_png() {
    println!("Applying CLUT...");

    let dir_list = glob("data/*").expect("Failed to read directories");

    for entry in dir_list {
        if let Ok(cmap_path) = entry {
            let cmap = cmap_path.file_name().unwrap().to_str().unwrap();
            println!("{}", cmap);

            let z = load_img("data/eroded_rgb.png");
            let (dzx, dzy) = gradient(&z);
            let (k, _) = gaussian_curvature(&z, 2.0);
            let img3 = apply_clut(vec![z, dzx, dzy], load_img("clut_3.npy"));

            // Add hillshading (uncomment this section if needed)
            // let sh = hillshade(&z, 180.0, 45.0, 10.0 * z.iter().map(|&x| x as f64).fold(0.0, f64::max) / z.shape()[0] as f64);
            // for k in 0..3 {
            //     for ((pixel, &s), &v) in img3.iter_mut().zip(sh.iter()).zip(img3.slice(s![.., .., k]).iter()) {
            //         *pixel = (v.sqrt() * *v as f64) as u16 * v as u16;
            //     }
            // }

            image::save(
                "texture.png",
                &DynamicImage::ImageRgba16(image::RgbImage::from_raw(
                    img3.shape()[0] as u32,
                    img3.shape()[1] as u32,
                    img3.iter().map(|&x| x as u16).collect::<Vec<u16>>(),
                ).unwrap()),
            )
                .expect("Failed to save image");
            File::create("TEXTURING_FINISHED").expect("Failed to create TEXTURING_FINISHED file");
        }
    }
}

fn np_save<T>(filename: &str, array: &Array<T, ndarray::Dim<[usize; 4]>>)
    where
        T: ndarray::Scalar + ndarray_npy::ReadNpyElement + ndarray_npy::WriteNpyElement,
{
    let file = File::create(filename).expect("Failed to create file");
    let writer = std::io::BufWriter::new(file);
    ndarray_npy::write_npy(writer, array.view()).expect("Failed to write npy file");
}
