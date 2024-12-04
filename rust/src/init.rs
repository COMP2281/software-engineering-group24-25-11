use godot::{
    classes::{input::MouseMode, XrServer},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct RootScene {
    base: Base<Node>,
}

#[godot_api]
impl INode for RootScene {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
    fn ready(&mut self) {
        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::CAPTURED);
    }
}
