use godot::{classes::input::MouseMode, prelude::*};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct WorldScene {
    base: Base<Node>,
}

#[godot_api]
impl INode for WorldScene {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
    fn ready(&mut self) {
        // Check mode
        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::CAPTURED);
    }
}
