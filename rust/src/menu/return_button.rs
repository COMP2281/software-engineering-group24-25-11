use godot::{
    classes::{Button, Control, IButton, MarginContainer},
    prelude::*,
};

use crate::scene_manager::SceneManager;

#[derive(GodotClass)]
#[class(init, base=Button)]
struct ReturnButton {
    base: Base<Button>,
}

#[godot_api]
impl IButton for ReturnButton {
    fn pressed(&mut self) {
        let scene_manager = SceneManager::get_manager(self.base().clone().upcast());
        let scene_manager = scene_manager.bind();

        if let Some(title_screen) = scene_manager.get_title_screen() {
            // hide everything that isn't the main menu
            title_screen
                .get_node_as::<Control>("CourseSelection")
                .hide();
            title_screen.get_node_as::<Control>("MessageBox").hide();

            title_screen
                .get_node_as::<MarginContainer>("MainMenu")
                .show();

            // regrab the focus.
            scene_manager
                .get_title_screen()
                .expect("return button inside title screen")
                .grab_focus();
        }
    }
}
