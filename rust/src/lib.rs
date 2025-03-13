use godot::init::{gdextension, ExtensionLibrary};

mod character;
mod menu;
mod question_bank;
mod resources;
mod scene_manager;
mod scenes;
mod sfx;
mod state;

// Provide an entry for GDExtension -- required - can be named anything.
struct VRGame;
#[gdextension]
unsafe impl ExtensionLibrary for VRGame {}
