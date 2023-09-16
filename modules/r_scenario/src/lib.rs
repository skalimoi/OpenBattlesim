use godot::prelude::*;

mod erosion;
mod entry_point;
mod climate;
mod normal_gen;

struct ErosionPlugin;

#[gdextension]
unsafe impl ExtensionLibrary for ErosionPlugin {}