use std::fs;

use godot::prelude::*;
use image_latest::{Luma, ImageBuffer};
use rand::seq::IteratorRandom;
use rand::thread_rng;
use crate::climate::{Climate, KOPPEN_CFC};

use crate::erosion::{*, world::World};
use crate::entry_point::world::Vec2;



#[derive(GodotClass)]
#[class(base=Node)]
pub struct ErosionActor {
    pub path_to_heightmap: String,
    pub cycles: i32,
    pub seed: real,
    #[base]
    base: Base<Node>
}





#[godot_api]
impl ErosionActor {
    #[func]
    pub fn erode_heightmap(cycles: i16, seed: i16) {
        let img_lvl1 = image_latest::io::Reader::open("data/raw/heightmap.png")
            .unwrap()
            .decode()
            .unwrap()
            .into_luma16();
        let (width, height) = img_lvl1.dimensions();
        let heightmap = img_lvl1.into_raw();
        let mut erosion_world = World::new(heightmap, width as usize, height as usize, seed);
        let mut discharge_map = vec![0; (width * height) as usize];
        use std::time::Instant;
        let now = Instant::now();
        let cycle_int = cycles as i32;
        for cycle in 0..cycle_int {
            erosion_world.erode(width as usize);
        }
        let elapsed = now.elapsed();
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
        let heightmap_buffer: image_latest::ImageBuffer<Luma<u16>, Vec<u16>> =
            ImageBuffer::from_raw(width, height, eroded_heightmap).unwrap();
        heightmap_buffer
        .save(format!("data/raw/eroded.r16").as_str())
            .unwrap();
        
        // let h = image_latest::io::Reader::open(format!("data/raw/eroded.png").as_str())
        //     .unwrap()
        //     .decode()
        //     .unwrap();
        // h.to_rgb16()
        //     .save(format!("data/raw/eroded_rgb.png"))
        //     .unwrap();

            let discharge_buffer: image_latest::ImageBuffer<Luma<u8>, Vec<u8>> =
            image_latest::ImageBuffer::from_raw(width, height, discharge_map.clone()).unwrap();
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
    /*
    #[func]
    pub fn choose_and_copy_biome() {
        let biome = KOPPEN_CFC;
        let mut name = String::from("");
        name.push(biome.general_type);
        name.push(biome.second_type);
        name.push(biome.third_type);
    
        let directory_path = "r_erosion/unselected_data";
        let name_str = name.as_str();
    
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
    
        {
            fs::copy(
                format!("data/raw/eroded_rgb.png").as_str(),
                "r_erosion/data/eroded_rgb.png",
            )
                .unwrap();
    
        }
    
    }

    
    */
    
}

#[godot_api]
impl NodeVirtual for ErosionActor {
    fn init(base: Base<Node>) -> Self {
        ErosionActor {
            path_to_heightmap: "heightmap.png".to_string(),
            cycles: 30,
            seed: 199565.0,
            base
        }
    }

    fn ready(&mut self) {
        
    }
}
