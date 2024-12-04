use godot::init::{gdextension, ExtensionLibrary};

pub mod character;
pub mod init;
pub mod title_screen;

// Provide an entry for GDExtension -- required - can be named anything.
struct VRGame;
#[gdextension]
unsafe impl ExtensionLibrary for VRGame {}
