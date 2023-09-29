/*
//// NOTE ////
Original program by Martin Lesser can be found here: https://github.com/MartinLesser/Procedural-distribution-of-vegetation
I had it adapted to Rust with his permission and support.
*/

use godot::prelude::*;
mod config;
mod edaphology;
mod entry_point;
mod hydrology;
mod insolation;
mod orography;
mod probabilities;

struct VegetationPlugin;

#[gdextension]
unsafe impl ExtensionLibrary for VegetationPlugin {}