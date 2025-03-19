use std::time::Duration;

use crate::scene_manager::SceneManager;
use godot::{
    classes::{
        file_access::ModeFlags, Button, Control, FileAccess, IButton, IControl, Label, LineEdit,
        Os, Theme, VBoxContainer,
    },
    prelude::*,
};
use humantime::format_duration;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct LeaderboardEntry {
    pub name: String,
    pub score: u64,
    pub time_taken: u64,
}

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct Leaderboard {
    base: Base<Control>,
}

#[godot_api]
impl IControl for Leaderboard {
    fn ready(&mut self) {
        self.update();
    }
}

impl Leaderboard {
    pub fn update(&self) {
        let mut entries: Vec<LeaderboardEntry> = Vec::new();
        if let Some(mut file) =
            FileAccess::open(crate::resources::datapaths::LEADERBOARD, ModeFlags::READ)
        {
            let contents = file.get_buffer(file.get_length() as i64);
            let contents = contents.as_slice();

            entries = serde_json::from_slice(contents).expect("failed to parse");
            file.close();
        }

        // populate questions into the overview
        let mut container = self
            .base()
            .get_node_as::<VBoxContainer>("VBoxContainer/ScrollContainer/VBoxContainer");
        let btn_theme = load::<Theme>(crate::resources::themes::GENERAL_BUTTON);

        // clear previous entries added
        for node in container.get_children().iter_shared() {
            container.remove_child(&node);
        }

        if entries.is_empty() {
            let mut empty_label = Label::new_alloc();
            empty_label.set_text("Leaderboard is currenty empty");
            container.add_child(&empty_label);
        }

        entries.sort_unstable_by_key(|x| x.score);
        for entry in entries.iter().rev() {
            let mut button = Button::new_alloc();
            button.set_theme(&btn_theme);
            button.set_text(&format!(
                "{}pts: {}: {}",
                entry.score,
                format_duration(Duration::from_millis(entry.time_taken)),
                entry.name,
            ));
            container.add_child(&button);
        }
    }
}

#[derive(GodotClass)]
#[class(init, base=Button)]
struct SaveButton {
    base: Base<Button>,
}

#[godot_api]
impl IButton for SaveButton {
    fn ready(&mut self) {
        let mouse_entered = self.base().callable("mouse_entered");
        self.base_mut().connect("mouse_entered", &mouse_entered);
    }
    fn pressed(&mut self) {
        let mut entries: Vec<LeaderboardEntry> = Vec::new();
        if let Some(mut file) =
            FileAccess::open(crate::resources::datapaths::LEADERBOARD, ModeFlags::READ)
        {
            let contents = file.get_buffer(file.get_length() as i64);
            let contents = contents.as_slice();

            entries = serde_json::from_slice(contents).expect("failed to parse");
            file.close();
        }

        let manager = SceneManager::get_manager(self.base().clone().upcast());
        let completion_screen = manager
            .bind()
            .get_completion_screen()
            .expect("save button is in tree when completion screen is.");

        // let results = format!(
        //     "{} / {}",
        //     completion_screen.bind().question_bank
        //         .answered_questions
        //         .iter()
        //         .filter(|x| x.correct)
        //         .count(),
        //     completion_screen.bind().question_bank.question_limit
        // );

        let name: String = completion_screen
            .bind()
            .get_save_scene()
            .get_node_as::<LineEdit>("VBoxContainer/LineEdit")
            .get_text()
            .into();
        if name.is_empty() {
            return;
        }

        let score = completion_screen.bind().question_bank.score;
        let time_taken = completion_screen.bind().time_elapsed
            - completion_screen.bind().question_bank.start_time;

        entries.push(LeaderboardEntry {
            name,
            score,
            time_taken,
        });

        let Some(mut file) =
            FileAccess::open(crate::resources::datapaths::LEADERBOARD, ModeFlags::WRITE)
        else {
            let mut os = Os::singleton();
            os.alert("failed to open leaderboard file, make sure you have allowed cookies!");
            return;
        };
        let new_entries: String = serde_json::to_string(&entries).expect("failed to parse");
        file.store_string(&new_entries);
        file.close();

        let mut os = Os::singleton();
        os.alert("saved to leaderboard");

        self.base_mut().set_disabled(true);
    }
}

#[godot_api]
impl SaveButton {
    #[func]
    pub fn mouse_entered(&mut self) {
        let scene_manager = SceneManager::get_manager(self.base().clone().upcast());
        let scene_manager = scene_manager.bind();

        let sfx = scene_manager.get_sfx_controller();
        let sfx = sfx.bind();
        sfx.play_menu_hover();
    }
}
