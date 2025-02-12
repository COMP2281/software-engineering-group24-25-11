use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct PlayerBase {
    base: Base<Node>,
}
