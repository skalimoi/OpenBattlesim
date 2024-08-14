#![feature(isqrt)]

use std::collections::HashMap;
use std::path::Path;
use godot::classes::Image;
use godot::classes::image::Format;
use godot::prelude::*;
use map_range::MapRange;

use image as image_crate;
use image::{GenericImage, ImageBuffer, Luma, Rgb};
use savefile::load_file;
use savefile_derive::Savefile;

struct MapLoader;

#[gdextension]
unsafe impl ExtensionLibrary for MapLoader {}

#[derive(GodotClass)]
#[class(base=Node)]
struct ChunkLoader;

#[derive(Savefile)]
struct HeightData {
    min: i32,
    max: i32,
}

#[derive(Savefile)]
struct SoilData {
    soil_value:  HashMap<u8, String>
}

#[godot_api]
impl INode for ChunkLoader {
    fn init(base: Base<Self::Base>) -> Self {
        Self
    }
}

#[godot_api]

impl ChunkLoader {
    #[func]
    pub fn get_master_height(scenario: GString, test:bool) -> Vector2i {
        if test {
            let height: HeightData = load_file(format!("data/debug/test_scenes/{}/master1.dat", scenario), 0).unwrap();
            Vector2i::new(height.min, height.max)
        } else {
            todo!()
        }
        
    }

    #[func]
    pub fn get_master_soils(scenario: GString, test:bool) -> Dictionary {
        if test {
            let soils: SoilData = load_file(format!("data/debug/test_scenes/{}/master2.dat", scenario), 0).unwrap();
            let dic: Dictionary = Dictionary::from_iter(soils.soil_value.into_iter());
            dic
        } else {
            todo!()
        }

    }
    #[func]
    pub fn get_height_data(scenario: GString, test: bool) -> Gd<Image> {
        let mut image = Image::create(8192, 8192, false, Format::RGBH).unwrap();
        let data = Self::load_big_data(scenario, test, "terrain");
        Self::populate_image(data, image.clone());
        image
    }
    
    #[func]
    pub fn get_soil_data(scenario: GString, id: i64, test: bool) -> Gd<Image> {
        let mut image = Image::create(8192, 8192, false, Format::RGB8).unwrap();
        let data = Self::load_soil_big_data(id, scenario, test);
        Self::populate_soil_image(data, image.clone());
        image
    }
    
    fn populate_image(data: ImageBuffer<Luma<u16>, Vec<u16>>, mut image: Gd<Image>){
        for x in 0..8192 {
            for y in 0..8192 {
                let d = data.get_pixel(x, y).0[0];
                let color = Color::from_rgba16(d, d, d, 0xffff);
                image.set_pixel(x as i32, y as i32, color);
            }
        }
    }

    fn populate_soil_image(data: ImageBuffer<Rgb<u8>, Vec<u8>>, mut image: Gd<Image>){
        for x in 0..8192 {
            for y in 0..8192 {
                let d = data.get_pixel(x, y).0[0];
                let color = Color::from_rgba8(d, d, d, 255);
                image.set_pixel(x as i32, y as i32, color);
            }
        }
    }
    fn load_soil_big_data(soil_id: i64, scenario_name: GString, is_test: bool) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut image : ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(8192, 8192);
        for x in 0..8 {
            for y in 0..8 {
                let raw = ChunkLoader::load_soil_from_file(scenario_name.clone(), soil_id, x, y, is_test);
                let piece : ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(1024, 1024, raw).unwrap();
                image.copy_from(&piece, (1024 * x) as u32, (1024 * y) as u32).unwrap();
            }
        }
        image
    }
    fn load_big_data(scenario_name: GString, is_test: bool, data_type: &str) -> ImageBuffer<Luma<u16>, Vec<u16>> {
        let mut image : ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::new(8192, 8192);
        for x in 0..8 {
            for y in 0..8 {
                let raw = ChunkLoader::load_from_file(scenario_name.clone(), x, y, is_test, data_type);
                let piece : ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::from_raw(1024, 1024, raw).unwrap();
                image.copy_from(&piece, (1024 * x) as u32, (1024 * y) as u32).unwrap();
            }
        }
        image
    }
    // #[func]
    // pub fn load_big_data_real_height(scenario_name: GString, is_test: bool, min: i64, max: i64) -> PackedInt32Array {
    //     let mut data: PackedInt32Array = PackedInt32Array::new();
    //     for x in 0..8 {
    //         for y in 0..8 {
    //             let raw = ChunkLoader::load_from_file(scenario_name.clone(), x, y, is_test);
    //             
    //             for i in 0..1024 {
    //                 for j in 0..1024 {
    //                     let val = ChunkLoader::get_height_from_bounds(raw.clone(), i as real, j as real, min as real, max as real);
    //                     data.push(val as i32);
    //                 }
    //             }
    //         }
    //     }
    //     data
    // }
    
    fn load_soil_from_file(scenario_name: GString, id: i64, x: i64, y: i64, is_test: bool) -> Vec<u8> {
        use savefile::prelude::*;
        let mut path = String::new();
        if is_test {
                path = format!("data/debug/test_scenes/{}/soils/id{}_x{}_y{}.dat", scenario_name, id, x, y);
                // don't ever remove this!! Somehow it avoids the filesystem errors.
                godot_script_error!("Path: {:?}", Path::new(&path).canonicalize()); 
                    let d: Vec<u8> = load_file(path, 0).unwrap();
                    return d;

        } else {
            todo!()
        }


    }
    
    fn load_from_file(scenario_name: GString, x: i64, y: i64, is_test: bool, data_type: &str) -> Vec<u16> {
        use savefile::prelude::*;
        let mut path = String::new();
        if is_test {
            match data_type { 
                "terrain" => { path = format!("data/debug/test_scenes/{}/terrain/h_map_tile_x{}_y{}.dat", scenario_name, x, y);
                    // don't ever remove this!! Somehow it avoids the filesystem errors.
                    godot_script_error!("Path: {:?}", Path::new(&path).canonicalize());
                    let d: Vec<u16> = load_file(path, 0).unwrap();

                    return d;}
                "soil" => { Vec::new() }
                "water" => { path = format!("data/debug/test_scenes/{}/terrain/s_map_tile_x{}_y{}.dat", scenario_name, x, y);
                    // don't ever remove this!! Somehow it avoids the filesystem errors.
                    godot_script_error!("Path: {:?}", Path::new(&path).canonicalize());
                    let d: Vec<u8> = load_file(path, 0).unwrap();
                    let e: Vec<u16> = d.iter().map(|x| {
                        *x as u16
                    }).collect();
                    return e;}
                _ => {return Vec::new()}
            }
            
        } else { 
            todo!()
        }
      
        
    }
    #[func]
    pub fn get_height_from_bounds(data: Array<i64>, x: real, y: real, min: real, max: real) -> real {
        let x = x as usize;
        let y = y as usize;
        let index = x * 1024 + y;
        data.get(index).unwrap().map_range(0..65535, (min as i64)..(max as i64)) as real
    }
}