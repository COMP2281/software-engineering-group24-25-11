use godot::prelude::*;

#[derive(Debug, Default, GodotConvert, Var, Export)]
#[godot(via = GString)]
enum InputMethod {
    VR,
    #[default]
    Normal,
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct State {
    input_method: InputMethod,
    base: Base<Node>,
}
