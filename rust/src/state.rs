use godot::prelude::*;

#[derive(Debug, Default, GodotConvert, Var, Export)]
#[godot(via = u8)]
enum InputMethod {
    #[default]
    Normal,
    VR,
}

// #[derive(Debug)]
// struct SessionState {
//     input_method: InputMethod,
//     course: String,
// }

#[derive(GodotClass, Debug)]
#[class(init, base=Node)]
pub struct AudioSettings {
    #[export]
    master: f64,
    #[export]
    music: f64,
    #[export]
    commentary: f64,
    base: Base<Node>,
}
