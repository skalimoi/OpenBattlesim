mod parser;
mod datatype;
mod syntax;
mod token;
mod loader;

use godot::prelude::*;

struct OpenBattlesimTokenLanguage;

#[gdextension]
unsafe impl ExtensionLibrary for OpenBattlesimTokenLanguage {}