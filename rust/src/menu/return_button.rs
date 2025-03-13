use godot::{
    classes::{Button, Control, IButton},
    prelude::*,
};

use crate::scene_manager::SceneManager;

#[derive(GodotClass)]
#[class(init, base=Button)]
struct ReturnButton {
    #[export]
    prev: Option<Gd<Control>>,
    base: Base<Button>,
}

#[godot_api]
impl IButton for ReturnButton {
    fn ready(&mut self) {
        let mouse_entered = self.base().callable("mouse_entered");
        self.base_mut().connect("mouse_entered", &mouse_entered);
    }
    fn pressed(&mut self) {
        let scene_manager = SceneManager::get_manager(self.base().clone().upcast());
        let scene_manager = scene_manager.bind();

        let sfx = scene_manager.get_sfx_controller();
        let sfx = sfx.bind();
        sfx.play_menu_select();

        let Some(prev) = &mut self.prev else {
            godot_warn!("return button: no return node selected!");
            return;
        };

        if let Some(mut title_screen) = scene_manager.get_title_screen() {
            title_screen.bind_mut().swap_to(prev.clone());
        } else if let Some(mut completion_screen) = scene_manager.get_completion_screen() {
            completion_screen.bind_mut().swap_to(prev.clone());
        }
    }
}

#[godot_api]
impl ReturnButton {
    #[func]
    pub fn mouse_entered(&mut self) {
        let scene_manager = SceneManager::get_manager(self.base().clone().upcast());
        let scene_manager = scene_manager.bind();

        let sfx = scene_manager.get_sfx_controller();
        let sfx = sfx.bind();
        sfx.play_menu_hover();
    }
}
