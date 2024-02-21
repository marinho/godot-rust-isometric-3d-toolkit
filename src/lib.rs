use godot::prelude::*;

mod learning_rust;
mod ui;
mod interaction;
mod scenes;

struct IsometricToolkit;

#[gdextension]
unsafe impl ExtensionLibrary for IsometricToolkit {}