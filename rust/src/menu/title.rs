use crate::scene_manager::{InputMode, SceneManager};
use godot::{
    classes::{Button, Control, IButton, MarginContainer},
    prelude::*,
};

#[derive(Debug, Default, Clone, Copy, GodotConvert, Var, Export, PartialEq, Eq)]
#[godot(via = u8)]
enum MainMenuAction {
    #[default]
    EnterVR,
    Enter3D,
    Tutorial,
    Options,
    Credits,
    Quit,
    Unknown,
}

#[derive(GodotClass)]
#[class(init, base=Button)]
struct MainMenuButton {
    #[export]
    action: MainMenuAction,
    base: Base<Button>,
}

#[godot_api]
impl IButton for MainMenuButton {
    #[cfg(target_arch = "wasm32")]
    fn ready(&mut self) {
        if self.action == MainMenuAction::Quit {
            self.base_mut().hide();
        };
    }

    fn pressed(&mut self) {
        let mut scene_tree = self.base().get_tree().expect("able to get the scene tree");
        let root_node = scene_tree
            .get_root()
            .unwrap()
            .get_node_as::<Node>("/root/Root");

        let title_screen = root_node.get_node_as::<Control>(&NodePath::from("TitleScreen"));
        let mut scene_manager = root_node
            .find_child(&GString::from("SceneManager"))
            .expect("SceneManager not present at the root of the scene")
            .try_cast::<SceneManager>()
            .expect("path /SceneManager is not a SceneManager!");
        let mut scene_manager = scene_manager.bind_mut();

        match self.action {
            MainMenuAction::EnterVR => {
                scene_manager.set_input_mode(InputMode::VR);
                title_screen
                    .get_node_as::<MarginContainer>("MainMenu")
                    .hide();
                let mut course_selection = title_screen.get_node_as::<Control>("CourseSelection");
                course_selection.show();
                course_selection.grab_focus();
            }
            MainMenuAction::Enter3D => {
                scene_manager.set_input_mode(InputMode::Normal);
                title_screen
                    .get_node_as::<MarginContainer>("MainMenu")
                    .hide();
                let mut course_selection = title_screen.get_node_as::<Control>("CourseSelection");
                course_selection.show();
                course_selection.grab_focus();
            }
            MainMenuAction::Quit => {
                scene_tree.quit();
            }
            _ => {
                godot_print!("called unimplemented main menu button");
            }
        }
    }
}
