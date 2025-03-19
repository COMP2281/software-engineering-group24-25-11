use crate::scene_manager::{InputMode, SceneManager};
use cfg_if::cfg_if;
use godot::{
    classes::{Button, Control, IButton, Label},
    prelude::*,
};

use super::leaderboard::Leaderboard;

#[derive(Debug, Default, Clone, Copy, GodotConvert, Var, Export, PartialEq, Eq)]
#[godot(via = u8)]
enum MainMenuAction {
    #[default]
    EnterVR,
    Enter3D,
    Options,
    Leaderboard,
    Credits,
    Quit,
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
    fn ready(&mut self) {
        cfg_if! {
            if #[cfg(target_arch="wasm32")] {
                if self.action == MainMenuAction::Quit {
                    self.base_mut().hide();
                };
            }
        }

        let mouse_entered = self.base().callable("mouse_entered");
        self.base_mut().connect("mouse_entered", &mouse_entered);
    }

    fn pressed(&mut self) {
        godot_print!("title screen button clicked");

        let mut scene_manager = SceneManager::get_manager(self.base().clone().upcast());
        let mut scene_manager = scene_manager.bind_mut();
        let title_screen = scene_manager
            .get_title_screen()
            .expect("currently in title screen");
        let title_screen = title_screen.bind();

        let sfx = scene_manager.get_sfx_controller();
        let sfx = sfx.bind();
        sfx.play_menu_select();

        match self.action {
            MainMenuAction::EnterVR => {
                scene_manager.set_input_mode(InputMode::VR);
                title_screen.show_course_selection();
            }
            MainMenuAction::Enter3D => {
                scene_manager.set_input_mode(InputMode::Normal);
                title_screen.show_course_selection();
            }
            MainMenuAction::Options => title_screen.show_options_menu(),
            MainMenuAction::Credits => title_screen.show_credits_menu(),
            MainMenuAction::Leaderboard => {
                title_screen.show_leaderboard();
            }
            MainMenuAction::Quit => {
                let mut scene_tree = self.base().get_tree().expect("able to get the scene tree");
                scene_tree.quit();
            }
        }
    }
}

#[godot_api]
impl MainMenuButton {
    #[func]
    pub fn mouse_entered(&mut self) {
        let scene_manager = SceneManager::get_manager(self.base().clone().upcast());
        let scene_manager = scene_manager.bind();

        let sfx = scene_manager.get_sfx_controller();
        let sfx = sfx.bind();
        sfx.play_menu_hover();
    }
}

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct TitleScreen {
    base: Base<Control>,
}

#[godot_api]
impl TitleScreen {
    // FIXME: move out of functions?
    #[func]
    pub fn show_course_selection(&self) {
        self.swap_to(self.base().get_node_as::<Control>("CourseSelection"));
    }

    #[func]
    pub fn show_options_menu(&self) {
        self.swap_to(self.base().get_node_as::<Control>("Options"));
    }

    #[func]
    pub fn show_credits_menu(&self) {
        self.swap_to(self.base().get_node_as::<Control>("Credits"));
    }

    #[func]
    pub fn show_leaderboard(&self) {
        let leaderboard = self.base().get_node_as::<Leaderboard>("Leaderboard");
        leaderboard.bind().update();
        self.swap_to(leaderboard.upcast());
    }

    #[func]
    pub fn show_message(&self, message: GString) {
        let message_box = self.base().get_node_as::<Control>("MessageBox");
        let mut label = message_box
            .get_node_as::<Label>("MarginContainer/PanelContainer/VBoxContainer/Message");
        label.set_text(&message);
        self.swap_to(message_box);
    }

    pub fn swap_to(&self, mut obj: Gd<Control>) {
        for child in self.base().get_children().iter_shared() {
            if child.get_name() == "TextureRect".into() {
                continue;
            }
            let mut child = child
                .try_cast::<Control>()
                .expect("all items in completion screen inherit from control");
            child.hide()
        }
        obj.show();
        obj.grab_focus();
    }
}
