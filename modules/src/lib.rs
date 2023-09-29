use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use palette::luma::Luma;
use palette::{encoding, Srgb};
use structopt::StructOpt;

pub struct Opt {
    /// Input file
    input: PathBuf,

    /// Number of value steps to display.
    num_steps: u8,

    /// Specify user value steps, example: `1,5,9`. Maximum of 11 values.
    values: Vec<u8>,

    extension: String,

    /// Use values declared with `--values` to color image and determine buckets.
    keep: bool,

    /// Debug flag, prints arguments and does not output image.
    debug: bool,

    /// Output file. When input is multiple files, this string will be appended
    /// to the filename. File type extension can be declared here for `.jpg`.
    output: Option<PathBuf>,
}

pub fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    let files = opt.input;
    let files_len = files.len();
    let values_len = opt.values.len() as u8;
    let luma_vec;
    let generated_colors;

    luma_vec = luma_threshold(opt.num_steps);
    generated_colors = get_greyscale_hashmap(&luma_vec);

    for file in files {
        let img = image::open(&file)?.to_rgb();
        let imgx = img.dimensions().0;
        let imgy = img.dimensions().1;
        let mut imgbuf: image::RgbImage = image::ImageBuffer::new(imgx, imgy);

        for (x, y, out_pixel) in imgbuf.enumerate_pixels_mut() {
            let in_pixel = img.get_pixel(x, y);
            let luma = (Luma::<encoding::Srgb>::from(
                Srgb::from_components((in_pixel[0], in_pixel[1], in_pixel[2])).into_format::<f32>(),
            )
            .luma
                * 255.0)
                .round() as u8;
            let color_key = get_threshold_key(luma, &luma_vec);
            let out_rgb = generated_colors.get(&color_key).unwrap();
            *out_pixel = image::Rgb([out_rgb.red, out_rgb.green, out_rgb.blue]);
        }

        // If single file, use output provided or generate filename.
        // If multiple files, try using output filename with extension provided.
        let title;
        if files_len == 1 {
            match &opt.output {
                Some(x) => {
                    let mut temp = x.clone();
                    match temp.extension() {
                        Some(_) => {}
                        None => {
                            temp.set_extension(&opt.extension);
                        }
                    }
                    title = temp;
                }
                None => {
                    let mut temp = PathBuf::from(generate_filename(&file)?);
                    temp.set_extension(&opt.extension);
                    title = temp;
                }
            }
        } else {
            match &opt.output {
                Some(x) => {
                    let mut temp = x.clone();
                    let clone = temp.clone();
                    let ext;
                    match clone.extension() {
                        Some(y) => {
                            ext = y.to_str().unwrap();
                        }
                        None => {
                            ext = &opt.extension;
                        }
                    }
                    temp.set_file_name(format!(
                        "{}-{}",
                        &file.file_stem().unwrap().to_str().unwrap(),
                        &temp.file_stem().unwrap().to_str().unwrap()
                    ));
                    title = temp.with_extension(ext);
                }
                None => {
                    let mut temp = PathBuf::from(generate_filename(&file)?);
                    temp.set_extension(&opt.extension);
                    title = temp;
                }
            }
        }

        if opt.debug {
            return Ok(());
        }

        // Delete file that gets created but can't be written to.
        match imgbuf.save(&title) {
            Ok(_) => {}
            Err(err) => {
                println!("Error: {}.", err);
                fs::remove_file(&title)?;
            }
        }
    }

    Ok(())
}

/// Generates the threshold buckets for `levels` to be divided into.
fn luma_threshold(num: u8) -> Vec<u8> {
    let step = 255 / num;
    let mut v = Vec::with_capacity(usize::from(num));
    for i in 0..num {
        v.push(i * step);
    }
    v
}

/// Generates user specified threshold buckets for `levels` to be divided into.
fn luma_threshold_custom(values: Vec<u8>) -> Vec<u8> {
    const BUCKET: u8 = 23;
    const LEN: usize = 11;
    let mut levels: Vec<u8> = Vec::with_capacity(11);
    let mut arr = [0; LEN];

    for val in values {
        if val < 11 {
            levels.push(val);
        } else {
            println!("Maximum value level is 10, {} will be clamped to 10.", val);
            levels.push(10);
        }
    }
    levels.sort();
    levels.dedup();

    let mut counter = 0;
    let mut next = *levels.get(1).unwrap();
    for (n, item) in arr.iter_mut().enumerate().take(LEN) {
        *item = *levels.get(counter).unwrap() * BUCKET;
        if n + 1 == usize::from(next) {
            counter += 1;
            if counter < levels.len() - 1 {
                next = *levels.get(counter + 1).unwrap()
            }
        }
    }
    arr.to_vec()
}

/// Replace user specified value colors in `luma_vec` with evenly spaced colors
/// as in `luma_threshold`.
fn luma_threshold_keep(vec: &[u8], num: u8) -> Vec<u8> {
    let step = 255 / num;
    let mut ret = Vec::with_capacity(11);
    let mut counter = 0;
    let mut curr = *vec.get(0).unwrap();
    for i in vec {
        if *i != curr {
            curr = *i;
            counter += 1;
        }
        ret.push(counter * step);
    }
    ret
}

/// Called when no output name is supplied. Appends a timestamp to the input
/// filename.
fn generate_filename(path: &PathBuf) -> Result<String, CliError> {
    let filename = path.file_stem().unwrap().to_str().unwrap().to_string();
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let secs = now.as_secs();
    let millis = format!("{:03}", now.subsec_millis());
    Ok(filename + "-" + &secs.to_string() + &millis)
}

/// Generate the greyscale colors to fill in the image based on values calculated
/// from `custom_luma_threshold`.
fn get_greyscale_hashmap(luma_zones: &[u8]) -> HashMap<u8, Srgb<u8>> {
    let mut hash = HashMap::new();
    if let Some((last, elements)) = luma_zones.split_last() {
        for i in elements {
            let x = *i;
            hash.insert(x, Srgb::from_components((x, x, x)));
        }
        hash.insert(*last, Srgb::from_components((255, 255, 255)));
    }
    hash
}

/// Generate the user colors to fill in the image based on values calculated
/// from `custom_luma_threshold`.
fn get_custom_greyscale_hashmap(luma_zones: &[u8]) -> HashMap<u8, Srgb<u8>> {
    let mut hash = HashMap::new();
    for i in luma_zones {
        let x = *i;
        hash.insert(x, Srgb::from_components((x, x, x)));
    }
    hash
}

/// Retrieve corresponding luma bucket key value.
fn get_threshold_key(in_color: u8, luma_vec: &[u8]) -> u8 {
    let mut key = luma_vec[0];
    for i in luma_vec {
        if in_color <= *i {
            return key;
        }
        key = *i;
    }
    key
}

/// Generate the user colors to fill in the image based on number of values
/// specified in `opt.colors`.
fn get_color_hashmap(
    colors: &[String],
    luma_zones: &[u8],
) -> Result<HashMap<u8, Srgb<u8>>, CliError> {
    let mut hash = HashMap::new();
    let iter = colors.iter().zip(luma_zones.iter());
    for (color, luma) in iter {
        let c = color.trim_start_matches("#");
        let x = *luma;
        hash.insert(x, parse_color(&c)?);
    }
    Ok(hash)
}

/// Generate the user colors to fill in the image based on values specified in
/// in `-v` and colors in `opt.colors`. Similar to `luma_threshold_keep`.
fn get_color_hashmap_custom(
    colors: &[String],
    luma_zones: &[u8],
) -> Result<HashMap<u8, Srgb<u8>>, CliError> {
    let mut hash = HashMap::new();
    let mut counter = 0;
    let mut curr = luma_zones[0];
    for luma in luma_zones.iter() {
        if *luma != curr {
            curr = *luma;
            counter += 1;
        }
        let c = colors[counter].trim_start_matches("#");
        let x = *luma;
        hash.insert(x, parse_color(&c)?);
    }
    Ok(hash)
}

fn parse_color(c: &str) -> Result<Srgb<u8>, CliError> {
    let red = u8::from_str_radix(
        match &c.get(0..2) {
            Some(x) => x,
            None => {
                println!("Invalid color: {}", c);
                return Err(CliError::InvalidHex);
            }
        },
        16,
    )?;
    let green = u8::from_str_radix(
        match &c.get(2..4) {
            Some(x) => x,
            None => {
                println!("Invalid color: {}", c);
                return Err(CliError::InvalidHex);
            }
        },
        16,
    )?;
    let blue = u8::from_str_radix(
        match &c.get(4..6) {
            Some(x) => x,
            None => {
                println!("Invalid color: {}", c);
                return Err(CliError::InvalidHex);
            }
        },
        16,
    )?;
    Ok(Srgb::new(red, green, blue))
}

#[derive(Debug)]
pub enum CliError {
    File(std::io::Error),
    Parse(std::num::ParseIntError),
    Time(std::time::SystemTimeError),
    InvalidHex,
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> CliError {
        CliError::File(err)
    }
}

impl From<std::num::ParseIntError> for CliError {
    fn from(err: std::num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}

impl From<std::time::SystemTimeError> for CliError {
    fn from(err: std::time::SystemTimeError) -> CliError {
        CliError::Time(err)
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::File(ref err) => write!(f, "File error: {}", err),
            CliError::InvalidHex => {
                write!(f, "Error: Invalid hex color length, must be 6 characters.")
            }
            CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
            CliError::Time(ref err) => write!(f, "Time error: {}", err),
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CliError::File(err) => Some(err),
            CliError::InvalidHex => None,
            CliError::Parse(err) => Some(err),
            CliError::Time(err) => Some(err),
        }
    }
}
