use godot::prelude::*;

mod tex;

struct TerrainTexturePlugin;

#[gdextension]
unsafe impl ExtensionLibrary for TerrainTexturePlugin {
}