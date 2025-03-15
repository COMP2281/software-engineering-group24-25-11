use std::time::Duration;

use crate::{
    question_bank::{AnsweredQuestion, QuestionBank},
    scene_manager::SceneManager,
};
use godot::{
    classes::{Button, Control, IButton, IControl, Label, Theme, VBoxContainer},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct CompletionScreen {
    pub time_elapsed: u64,
    pub question_bank: QuestionBank,
    base: Base<Control>,
}

#[godot_api]
impl IControl for CompletionScreen {
    fn ready(&mut self) {
        // populate questions into the overview
        let mut questions_container = self.base().get_node_as::<VBoxContainer>(
            "QuestionsOverview/VBoxContainer/ScrollContainer/VBoxContainer",
        );
        for question in &self.question_bank.answered_questions {
            let button = QuestionOverviewButton::with_question(question.clone());
            questions_container.add_child(&button);
        }
        // populate details into overview

        let mut completed = self
            .base()
            .get_node_as::<Label>("Overview/VBoxContainer/Score");
        completed.set_text(&format!(
            "{} / {}",
            self.question_bank
                .answered_questions
                .iter()
                .filter(|x| x.correct)
                .count(),
            self.question_bank.question_limit
        ));
        let mut points = self
            .base()
            .get_node_as::<Label>("Overview/VBoxContainer/Points");
        points.set_text(&format!("{}pts", self.question_bank.score));
        let mut time = self
            .base()
            .get_node_as::<Label>("Overview/VBoxContainer/Time");
        time.set_text(&format!(
            "{}",
            humantime::format_duration(Duration::from_millis(
                self.time_elapsed - self.question_bank.start_time
            )),
        ));
    }
}

#[godot_api]
impl CompletionScreen {
    pub fn new_screen(question_bank: QuestionBank, time_elapsed: u64) -> Gd<Self> {
        let mut completion_scene = load::<PackedScene>(crate::resources::COMPLETION_SCENE)
            .instantiate_as::<CompletionScreen>();
        completion_scene.set_name("CompletionScreen");
        completion_scene.bind_mut().question_bank = question_bank;
        completion_scene.bind_mut().time_elapsed = time_elapsed;
        completion_scene
    }

    fn show_details(&self, question: &AnsweredQuestion) {
        let question_details = self.base().get_node_as::<Control>("QuestionDetails");

        let mut questions_label = question_details.get_node_as::<Label>("VBoxContainer/Question");
        questions_label.set_text(&question.question);
        let mut correct_answer_label =
            question_details.get_node_as::<Label>("VBoxContainer/CorrectAnswer");
        correct_answer_label.set_text(
            &question
                .correct_answers
                .iter_shared()
                .map(|x| question.choices.get(x as usize).unwrap().to_string())
                .collect::<Vec<_>>()
                .join("\n\n"),
        );

        let mut given_answer_label =
            question_details.get_node_as::<Label>("VBoxContainer/YourAnswer");
        given_answer_label.set_text(
            &question
                .given_answers
                .iter_shared()
                .map(|x| question.choices.get(x as usize).unwrap().to_string())
                .collect::<Vec<_>>()
                .join("\n\n"),
        );

        let question_details = self.base().get_node_as::<Control>("QuestionDetails");
        self.swap_to(question_details);
    }

    pub fn get_review_scene(&self) -> Gd<Control> {
        self.base().get_node_as::<Control>("QuestionsOverview")
    }
    pub fn get_save_scene(&self) -> Gd<Control> {
        self.base().get_node_as::<Control>("SetUsername")
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

#[derive(Debug, Default, Clone, Copy, GodotConvert, Var, Export, PartialEq, Eq)]
#[godot(via = u8)]
enum OverviewAction {
    #[default]
    Review,
    Save,
    MainMenu,
}
#[derive(GodotClass)]
#[class(init, base=Button)]
struct OverviewButton {
    #[export]
    action: OverviewAction,
    base: Base<Button>,
}

#[godot_api]
impl IButton for OverviewButton {
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

        let completion_screen = scene_manager
            .get_completion_screen()
            .expect("overview button inside completion screen");
        let completion_screen = completion_screen.bind();
        match self.action {
            OverviewAction::Review => {
                completion_screen.swap_to(completion_screen.get_review_scene())
            }
            OverviewAction::Save => completion_screen.swap_to(completion_screen.get_save_scene()),
            OverviewAction::MainMenu => scene_manager.close_completion_screen(),
        }
    }
}

#[godot_api]
impl OverviewButton {
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
#[class(init, base=Button)]
struct QuestionOverviewButton {
    question: AnsweredQuestion,
    base: Base<Button>,
}

#[godot_api]
impl IButton for QuestionOverviewButton {
    fn ready(&mut self) {
        self.base_mut()
            .set_theme(&load::<Theme>(crate::resources::themes::GENERAL_BUTTON));
        let question = self.question.question.clone();
        self.base_mut().set_text(&question);

        let mouse_entered = self.base().callable("mouse_entered");
        self.base_mut().connect("mouse_entered", &mouse_entered);
    }
    fn pressed(&mut self) {
        let scene_manager = SceneManager::get_manager(self.base().clone().upcast());
        let scene_manager = scene_manager.bind();
        let sfx = scene_manager.get_sfx_controller();
        let sfx = sfx.bind();
        sfx.play_menu_select();

        let completion_screen = scene_manager
            .get_completion_screen()
            .expect("inside completion screen");
        let completion_screen = completion_screen.bind();

        completion_screen.show_details(&self.question);
    }
}

#[godot_api]
impl QuestionOverviewButton {
    pub fn with_question(question: AnsweredQuestion) -> Gd<Self> {
        Gd::from_init_fn(|base| Self { question, base })
    }

    #[func]
    pub fn mouse_entered(&mut self) {
        let scene_manager = SceneManager::get_manager(self.base().clone().upcast());
        let scene_manager = scene_manager.bind();

        let sfx = scene_manager.get_sfx_controller();
        let sfx = sfx.bind();
        sfx.play_menu_hover();
    }
}
