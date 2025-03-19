use crate::scene_manager::SceneManager;
use godot::{
    classes::{Button, Control, IButton, MarginContainer},
    prelude::*,
};

#[derive(Debug, Default, Clone, Copy, GodotConvert, Var, Export, PartialEq, Eq)]
#[godot(via = u8)]
enum PauseMenuAction {
    #[default]
    Resume,
    ChangeCourse,
    Options,
    ReturnToMainMenu,
}

#[derive(GodotClass)]
#[class(init, base=Button)]
struct PauseMenuButton {
    #[export]
    action: PauseMenuAction,
    base: Base<Button>,
}

#[godot_api]
impl IButton for PauseMenuButton {
    fn ready(&mut self) {
        let mouse_entered = self.base().callable("mouse_entered");
        self.base_mut().connect("mouse_entered", &mouse_entered);
    }

    fn pressed(&mut self) {
        let mut scene_manager = SceneManager::get_manager(self.base().clone().upcast());

        let sfx = scene_manager.bind().get_sfx_controller();
        let sfx = sfx.bind();
        sfx.play_menu_select();

        match self.action {
            PauseMenuAction::Resume => {
                godot_print!("clicked: resume game");
                scene_manager.bind().resume_game();
            }
            PauseMenuAction::ChangeCourse => {
                godot_print!("clicked: change course");
                scene_manager.bind().hide_pause_menu();
                scene_manager.bind_mut().exit_world();
                let title_screen = scene_manager
                    .bind()
                    .get_title_screen()
                    .expect("in title screen after exiting world");

                title_screen
                    .get_node_as::<MarginContainer>("MainMenu")
                    .hide();
                let mut course_selection = title_screen.get_node_as::<Control>("CourseSelection");
                course_selection.show();
                course_selection.grab_focus();
                // TODO: make this
            }
            PauseMenuAction::Options => {
                godot_print!("clicked: options");
            }
            PauseMenuAction::ReturnToMainMenu => {
                godot_print!("clicked: return to main menu");
                scene_manager.bind().hide_pause_menu();
                scene_manager.bind_mut().exit_world();
                // TODO: make this
            }
        }
    }
}

#[godot_api]
impl PauseMenuButton {
    #[func]
    pub fn mouse_entered(&mut self) {
        let scene_manager = SceneManager::get_manager(self.base().clone().upcast());
        let scene_manager = scene_manager.bind();

        let sfx = scene_manager.get_sfx_controller();
        let sfx = sfx.bind();
        sfx.play_menu_hover();
    }
}
