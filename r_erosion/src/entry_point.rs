use godot::prelude::*;
use image_latest::{Luma, ImageBuffer};
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
    fn erode_heightmap(cycles: i32, seed: real) {
        println!("Starting erosion job...");
        let img_lvl1 = image_latest::io::Reader::open(format!("heightmap.png").as_str())
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
        for cycle in 0..cycles {
            erosion_world.erode(width as usize);
            println!("Cycle: {}", cycle);
        }
        let elapsed = now.elapsed();
        println!("Erosion finished. Elapsed: {:.2?}", elapsed);
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
            .save(format!("eroded_lvl1.png").as_str())
            .unwrap();
        let h = image_latest::io::Reader::open(format!("eroded_lvl1.png").as_str())
            .unwrap()
            .decode()
            .unwrap();
        h.to_rgb16()
            .save(format!("eroded_rgb_lvl1.png"))
            .unwrap();

            let discharge_buffer: image_latest::ImageBuffer<Luma<u8>, Vec<u8>> =
            image_latest::ImageBuffer::from_raw(width, height, discharge_map.clone()).unwrap();
        discharge_buffer
            .save(format!("discharge_lvl1.png").as_str())
            .unwrap();

        let proc_water =
            image_latest::io::Reader::open(format!("discharge_lvl1.png").as_str())
                .unwrap()
                .decode()
                .unwrap();
        let mut gray = proc_water.to_luma8();
        imageproc::contrast::stretch_contrast_mut(&mut gray, 130, 200);
        gray.save(format!("discharge_lvl1.png").as_str())
            .unwrap();
    
    
    }

    #[func]
    fn create_normal() {
        let image_for_normal =
            image_normal::open(format!("eroded_lvl1.png").as_str()).unwrap();
        normal_heights::map_normals_with_strength(&image_for_normal, 10.0)
            .save_with_format(
                format!("normal_lvl1.png").as_str(),
                image_normal::ImageFormat::Png,
            )
            .expect("ERROR WHEN CREATING NORMAL!!");
        
    }
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
