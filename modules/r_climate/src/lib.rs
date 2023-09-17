use godot::prelude::*;

mod climate;
mod grid;
mod gendata;



struct ClimatePlugin;

#[gdextension]
unsafe impl ExtensionLibrary for ClimatePlugin {}