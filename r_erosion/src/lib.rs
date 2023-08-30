use godot::prelude::*;

mod erosion;
mod entry_point;

struct ErosionPlugin;

#[gdextension]
unsafe impl ExtensionLibrary for ErosionPlugin {}