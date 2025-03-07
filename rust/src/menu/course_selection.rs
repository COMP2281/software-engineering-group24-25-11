use crate::{
    question_bank::{Course, Mode},
    scene_manager::SceneManager,
};
use godot::{
    classes::{Button, IButton},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Button)]
struct CourseSelectionButton {
    #[export]
    course: Course,
    base: Base<Button>,
}

#[godot_api]
impl IButton for CourseSelectionButton {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            course: Default::default(),
            base,
        }
    }
    fn pressed(&mut self) {
        let mut scene_manager = SceneManager::get_manager(self.base().clone().upcast());
        let mut scene_manager = scene_manager.bind_mut();

        // TODO: move difficulty into its own slider or something
        scene_manager.set_difficulty(Mode::Normal);
        scene_manager.set_course(self.course);
        scene_manager.enter_world();
    }
}
