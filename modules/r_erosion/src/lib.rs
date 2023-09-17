use godot::prelude::*;

mod erosion;
mod entry_point;
mod climate;

struct ErosionPlugin;

#[gdextension]
unsafe impl ExtensionLibrary for ErosionPlugin {}