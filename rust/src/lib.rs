use godot::init::{gdextension, ExtensionLibrary};

mod character;
mod menu;
mod scenes;
mod state;

// Provide an entry for GDExtension -- required - can be named anything.
struct VRGame;
#[gdextension]
unsafe impl ExtensionLibrary for VRGame {}
