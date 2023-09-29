use std::ffi::CString;
use std::fs;

use fs_extra::file::write_all;
use godot::prelude::*;
use image_latest::{Luma, ImageBuffer, DynamicImage, Rgb, GenericImage, EncodableLayout};
use imageproc::map::red_channel;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::fs::{File};
use std::io::{BufWriter, Write};
use std::mem::ManuallyDrop;
use std::str::FromStr;
use byteorder::{LittleEndian, WriteBytesExt};
use godot::engine::utilities::round;
use godot::prelude::utilities::roundf;
use image_latest::imageops::FilterType;
use nalgebra::Dynamic;
use posterust::Opt;

use crate::erosion::{*, world::World};
use crate::entry_point::world::Vec2;


#[derive(GodotClass)]
#[class(base = Node)]
pub struct ErosionActor {
    #[var]
    pub path_to_heightmap: GodotString,
    pub cycles: i32,
    pub seed: real,
    #[var]
    pub current_cycle: real,
    #[base]
    base: Base<Node>,
}


#[godot_api]
impl ErosionActor {
    #[func]
    pub fn erode_heightmap(&mut self, cycles: i16, seed: i16) {
        let img_lvl1 = image_latest::io::Reader::open(self.path_to_heightmap.to_string())
            .unwrap()
            .decode()
            .unwrap()
            .into_luma16();
        let (width, height) = img_lvl1.dimensions();
        let heightmap = img_lvl1.into_raw();
        let mut erosion_world = World::new(heightmap, width as usize, height as usize, seed);
        let mut discharge_map = vec![0; (width * height) as usize];
        let cycle_int = cycles as i32;

        for cycle in 0..cycle_int {
            erosion_world.erode(width as usize);
            self.current_cycle = cycle.clone() as real;
            godot_print!("Erosion cycle: {cycle}");

        }
        for i in 0..discharge_map.len() {
            let pos = Vec2::new(i as f64 % width as f64, (i / width as usize) as f64);
            discharge_map[i] = ((erosion_world.map.discharge(pos) + 1.0) * 0.5 * 255.0) as u8;
        }
        let eroded_heightmap: Vec<u16> = erosion_world
            .map
            .heightmap
            .iter()
            .map(|x| (x.height * 255.0) as u16)
            .collect();


        let mut file = File::create("data/raw/m_terrain_heightmap_eroded.r16").unwrap();
        for value in eroded_heightmap.as_slice() {
            file.write_u16::<LittleEndian>(*value).unwrap();
        }

        let buffer: ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::from_raw(8193, 8193, eroded_heightmap).unwrap();
        buffer.save("data/raw/texturing_buffer.png").unwrap();

        let to_resample = image_latest::io::Reader::open("data/raw/texturing_buffer.png").unwrap().decode().unwrap();
        let resized = to_resample.resize(1025, 1025, FilterType::Nearest);
        resized.save("data/raw/height_map_veg.png").unwrap();
        posterize(resized);

        let discharge_buffer: ImageBuffer<Luma<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width, height, discharge_map.clone()).unwrap();
        discharge_buffer
            .save(format!("data/raw/discharge.png").as_str())
            .unwrap();

        let proc_water =
            image_latest::io::Reader::open(format!("data/raw/discharge.png").as_str())
                .unwrap()
                .decode()
                .unwrap();
        let mut gray = proc_water.to_luma8();
        imageproc::contrast::stretch_contrast_mut(&mut gray, 130, 200);
        gray.save(format!("data/raw/discharge.png").as_str())
            .unwrap();

    }

    /*
    #[func]
    pub fn create_normal() {
    
        godot_print!("Creating normal maps...");
        let image_for_normal =
            image_normal::open(format!("data/raw/eroded.png").as_str()).unwrap();
        normal_gen::normal_gen::map_normals_with_strength(&image_for_normal, 1.0)
            .save_with_format(
                format!("data/raw/normal.png").as_str(),
                image_normal::ImageFormat::Png,
            )
            .expect("ERROR WHEN CREATING NORMAL!!");
    }

    #[func]
    pub fn generate_tile_data() {
        let path = "data/raw/texture_blurred.png";
        fs::copy("r_erosion/texture.png", "data/raw/texture.png");
        match fs::metadata(path) {
            Ok(_) => (),
            Err(_) => {
                let img = image_latest::open("data/raw/texture.png").unwrap();
                image_latest::imageops::blur(&img, 1.5).save("data/raw/texture_blurred.png").unwrap();
            }
        }

        let tile_size: usize = 512;
    
        let total_image = image_latest::io::Reader::open("data/raw/eroded.png")
            .unwrap()
            .decode()
            .unwrap();
        
    
        let discharge_tile = image_latest::open("data/raw/discharge.png").unwrap();
        
    
        let tex_tile = image_latest::open("data/raw/texture_blurred.png").unwrap();
        
    
        let normal_tile = image_latest::open("data/raw/normal.png")
            .unwrap();
        
    
    
        fs::create_dir_all("data/tiles").expect("Error creating tile dir");
       
    
        for tile_x in 0..=15 {
            for tile_y in 0..=15 {
                let height1 = image_latest::imageops::crop_imm(&total_image, (tile_x * tile_size) as u32, (tile_y * tile_size) as u32, tile_size as u32, tile_size as u32);
                height1.to_image().save_with_format(format!("data/tiles/height_{}_{}.png", tile_x, tile_y), image_latest::ImageFormat::Png);
                let rgb8_im = image_latest::open(format!("data/tiles/height_{}_{}.png", tile_x, tile_y)).unwrap();
                rgb8_im.to_rgb16().save_with_format(format!("data/tiles/height_{}_{}.png", tile_x, tile_y), image_latest::ImageFormat::Png);
                
                
    
                let normal1 = image_latest::imageops::crop_imm(&normal_tile, (tile_x * tile_size) as u32, (tile_y * tile_size) as u32, tile_size as u32, tile_size as u32);
                normal1.to_image().save(format!("data/tiles/normal_{}_{}.png", tile_x, tile_y)).expect("Error creating height tile!");
                let rgb8_im = image_latest::open(format!("data/tiles/normal_{}_{}.png", tile_x, tile_y)).unwrap();
                rgb8_im.to_rgb16().save_with_format(format!("data/tiles/normal_{}_{}.png", tile_x, tile_y), image_latest::ImageFormat::Png);
    
                // DISCHARGE
    
                let distile = discharge_tile.crop_imm((tile_x * tile_size) as u32, (tile_y * tile_size) as u32, tile_size as u32, tile_size as u32);
                distile.save(format!("data/tiles/discharge_{}_{}.png", tile_x, tile_y)).expect("Error creating water tile!");
                let rgb8_im = image_latest::open(format!("data/tiles/discharge_{}_{}.png", tile_x, tile_y)).unwrap();
                rgb8_im.to_rgb16().save_with_format(format!("data/tiles/discharge_{}_{}.png", tile_x, tile_y), image_latest::ImageFormat::Png);
    
    
                // TEXTURE
    
                let textile = tex_tile.crop_imm((tile_x * tile_size) as u32, (tile_y * tile_size) as u32, tile_size as u32, tile_size as u32);
                textile.save(format!("data/tiles/tex_{}_{}.png", tile_x, tile_y)).expect("Error creating tex tile!");
                let rgb8_im = image_latest::open(format!("data/tiles/tex_{}_{}.png", tile_x, tile_y)).unwrap();
                rgb8_im.to_rgb16().save_with_format(format!("data/tiles/tex_{}_{}.png", tile_x, tile_y), image_latest::ImageFormat::Png);
                
            }
        }
    
        // fs::remove_file("data/raw/eroded.png");
        // fs::remove_file("data/raw/texture.png");
        // fs::remove_file("data/raw/normal.png");
        // fs::remove_file("data/raw/eroded_rgb.png");
    
    
    }
     */
}

fn posterize(image: DynamicImage) {
    let raw = image.to_luma8().into_raw();
    let mut new: Vec<u8> = vec![];
    for mut pixel in raw.clone() {
        let mut pixel_new: u8;
        pixel_new = match pixel {
            0u8..=39u8 => 40u8,
            40u8..=79u8 => 40u8,
            80u8..=119u8 => 80u8,
            120u8..=159u8 => 120u8,
            160u8..=199u8 => 160u8,
            200u8..=239u8 => 200u8,
            240u8..=255u8 => 240u8,
            _ => 0u8
        };
        new.push(pixel_new);
    }
    let buffer: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_raw(1025, 1025, new).unwrap();
    buffer.save("data/raw/soil_map.png").unwrap();

}

#[godot_api]
impl NodeVirtual for ErosionActor {
    fn init(base: Base<Node>) -> Self {
        ErosionActor {
            path_to_heightmap: "heightmap.png".to_string().parse().unwrap(),
            cycles: 30,
            current_cycle: 1 as real,
            seed: 199565.0,
            base,
        }
    }

    fn ready(&mut self) {}
}
