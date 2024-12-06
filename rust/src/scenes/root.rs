use godot::{classes::input::MouseMode, prelude::*};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct RootScene {
    base: Base<Node>,
}

#[godot_api]
impl INode for RootScene {
    fn init(base: Base<Node>) -> Self {
        let root_scene = Self { base };
        root_scene.base().add_child(crate::state::State::new_gd());
        root_scene
    }
    fn ready(&mut self) {
        // Check mode
        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::CAPTURED);
    }
}
