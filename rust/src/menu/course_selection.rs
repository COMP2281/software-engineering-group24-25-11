use crate::{
    question_bank::{Course, Mode},
    scene_manager::SceneManager,
};
use godot::{
    classes::{Button, Control, IButton, MarginContainer},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Button)]
struct CourseSelectionButton {
    #[export]
    course: Course,
    // FIXME: this should be something better.
    #[export]
    back_button: bool,
    base: Base<Button>,
}

#[godot_api]
impl IButton for CourseSelectionButton {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            course: Default::default(),
            back_button: false,
            base,
        }
    }
    fn pressed(&mut self) {
        let scene_tree = self.base().get_tree().expect("able to get the scene tree");
        let root_node = scene_tree
            .get_root()
            .unwrap()
            .get_node_as::<Node>("/root/Root");

        let mut scene_manager = root_node
            .find_child(&GString::from("SceneManager"))
            .expect("SceneManager not present at the root of the scene")
            .try_cast::<SceneManager>()
            .expect("path /SceneManager is not a SceneManager!");
        let mut scene_manager = scene_manager.bind_mut();

        if self.back_button {
            let title_screen = root_node
                .find_child(&GString::from("TitleScreen"))
                .expect("custom menu button used outside of title screen");
            title_screen
                .get_node_as::<Control>("CourseSelection")
                .hide();
            title_screen
                .get_node_as::<MarginContainer>("MainMenu")
                .show();
        } else {
            // TODO: move difficulty into its own slider or something
            scene_manager.set_difficulty(Mode::Normal);
            scene_manager.set_course(self.course);
            scene_manager.enter_world();
        }
    }
}
