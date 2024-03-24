mod parser;
mod datatype;
mod syntax;
mod token;

use godot::prelude::*;

struct OpenBattlesimTokenLanguage;

#[gdextension]
unsafe impl ExtensionLibrary for OpenBattlesimTokenLanguage {}