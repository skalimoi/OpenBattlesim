#![feature(exclusive_range_pattern)]

mod data_generation;


use godot::prelude::*;


struct Scenario;

#[gdextension]
unsafe impl ExtensionLibrary for Scenario {}


enum Season {
    Summer,
    Winter,
    Spring,
    Fall
}


#[derive(GodotClass)]
#[class(base=Area3D)]
struct GridComponent {
    index: Vector3,
    altitude: real,
    temperature: real,
    wind: Vector3,
    pressure: real,
    humidity: real,
    is_precipitating: bool,
    dew_point: real
}

