use crate::{
    question_bank::{Course, Mode},
    scene_manager::SceneManager,
};
use godot::{
    classes::{Button, IButton},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=Button)]
struct CourseSelectionButton {
    #[export]
    course: Course,
    base: Base<Button>,
}

#[godot_api]
impl IButton for CourseSelectionButton {
    fn ready(&mut self) {
        let mouse_entered = self.base().callable("mouse_entered");
        self.base_mut().connect("mouse_entered", &mouse_entered);
    }

    fn pressed(&mut self) {
        let mut scene_manager = SceneManager::get_manager(self.base().clone().upcast());
        let mut scene_manager = scene_manager.bind_mut();

        let sfx = scene_manager.get_sfx_controller();
        let sfx = sfx.bind();
        sfx.play_menu_select();

        // TODO: move difficulty into its own slider or something
        scene_manager.set_difficulty(Mode::Normal);
        scene_manager.set_course(self.course);
        scene_manager.enter_world();
    }
}

#[godot_api]
impl CourseSelectionButton {
    #[func]
    pub fn mouse_entered(&mut self) {
        let scene_manager = SceneManager::get_manager(self.base().clone().upcast());
        let scene_manager = scene_manager.bind();

        let sfx = scene_manager.get_sfx_controller();
        let sfx = sfx.bind();
        sfx.play_menu_hover();
    }
}
