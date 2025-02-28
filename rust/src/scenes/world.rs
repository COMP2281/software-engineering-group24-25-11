use std::str::FromStr;

use godot::{
    classes::{file_access::ModeFlags, input::MouseMode, FileAccess, Time, Timer},
    global::randi_range,
    prelude::*,
};
use serde::Deserialize;

use super::question_panel::QuestionPanel;

#[derive(Debug, Default, GodotConvert, Var, Export)]
#[godot(via = u8)]
enum Course {
    #[default]
    DataFundamentals,
    WebDev,
    AiModules,
}

impl From<Course> for GString {
    fn from(value: Course) -> Self {
        match value {
            Course::DataFundamentals => "res://question_bank/data_fundamentals.json".into(),
            Course::WebDev => "res://question_bank/web_dev.json".into(),
            Course::AiModules => "res://question_bank/ai_modules.json".into(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, GodotConvert, Var, Export)]
#[godot(via = u8)]
enum Mode {
    Easy,
    #[default]
    Normal,
    Hard,
}

impl From<Mode> for f64 {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Easy => 0.3,
            Mode::Normal => 0.5,
            Mode::Hard => 0.7,
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Question {
    question: String,
    choices: Vec<String>,
    answers: Vec<i64>,
    difficulty: f64,
}

pub struct AnsweredQuestion {
    question: GString,
    correct_answer: GString,
    given_answer: GString,
    correct: bool,
    time_taken_ms: i64,
}

pub struct QuestionBank {
    questions: Vec<Question>,
    current_difficulty: f64,
    start_time: u64,
    mode: Mode,
}

impl QuestionBank {
    fn new(mode: Mode, course: Course) -> Option<Self> {
        let course_path: GString = course.into();
        let file = FileAccess::open(&course_path, ModeFlags::READ)
            .expect("failed to open course question bank");
        let contents = file.get_buffer(file.get_length() as i64);
        let contents = contents.as_slice();

        let questions: Vec<Question> = serde_json::from_slice(contents).expect("failed to parse");

        let spawn_time = Time::singleton().get_ticks_msec();

        Some(Self {
            questions,
            current_difficulty: mode.into(),
            start_time: spawn_time,
            mode,
        })
    }

    pub fn get_question(&mut self) -> Question {
        let difficulty = self.current_difficulty + (randi_range(-15, 15) as f64 * 0.01);
        let idx = self
            .questions
            .iter()
            .enumerate()
            .min_by(|(_, x), (_, y)| {
                x.difficulty
                    .partial_cmp(&difficulty)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .cmp(
                        &y.difficulty
                            .partial_cmp(&difficulty)
                            .unwrap_or(std::cmp::Ordering::Equal),
                    )
            })
            .map(|(idx, _)| idx)
            .expect("have a question left");
        self.questions.remove(idx)
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct WorldScene {
    pub question_bank: QuestionBank,
    base: Base<Node>,
}

#[godot_api]
impl INode for WorldScene {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            question_bank: QuestionBank::new(Mode::Normal, Course::WebDev)
                .expect("failed to create question bank"),
        }
    }
    fn ready(&mut self) {
        let scene_tree = self.base().get_tree().expect("world is in scene tree");
        let mut room = WorldScene::get_room(scene_tree).expect("world has room");
        let question = self.question_bank.get_question();
        self.question_bank.start_time = Time::singleton().get_ticks_msec();
        let mut panel = QuestionPanel::create_panel(
            GString::from(question.question),
            question
                .choices
                .iter()
                .map(GString::from)
                .collect::<Array<GString>>(),
            question.answers.iter().cloned().collect::<Array<i64>>(),
        );
        panel.set_position(Vector3::new(0., 5., 0.));
        panel.set_name(&GString::from("QuestionPanel"));
        room.add_child(&panel);
        // Check mode
        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::CAPTURED);
    }
}

#[godot_api]
impl WorldScene {
    #[func]
    pub fn answered_correctly(&mut self) {
        let scene_tree = self.base().get_tree().expect("world is in scene tree");
        let mut room = WorldScene::get_room(scene_tree.clone()).expect("world has room");
        let panel = QuestionPanel::find_panel(scene_tree)
            .expect("answered correctly called with a panel existing");
        room.remove_child(&panel);

        self.question_bank.current_difficulty =
            (self.question_bank.current_difficulty + 0.1).clamp(0., 1.);

        let time_taken = Time::singleton()
            .get_ticks_msec()
            .saturating_sub(self.question_bank.start_time);
        self.question_bank.start_time = Time::singleton().get_ticks_msec();
        godot_print!("Took {}ms to answer question", time_taken);

        let question = self.question_bank.get_question();
        let mut panel = QuestionPanel::create_panel(
            GString::from(question.question),
            question
                .choices
                .iter()
                .map(GString::from)
                .collect::<Array<GString>>(),
            question.answers.iter().cloned().collect::<Array<i64>>(),
        );
        panel.set_position(Vector3::new(0., 5., 0.));
        panel.set_name(&GString::from("QuestionPanel"));
        room.add_child(&panel);
    }
    #[func]
    pub fn answered_incorrectly(&mut self) {
        let scene_tree = self.base().get_tree().expect("world is in scene tree");
        let mut room = WorldScene::get_room(scene_tree.clone()).expect("world has room");
        let panel = QuestionPanel::find_panel(scene_tree)
            .expect("answered correctly called with a panel existing");

        let time_taken = Time::singleton()
            .get_ticks_msec()
            .saturating_sub(self.question_bank.start_time);
        self.question_bank.start_time = Time::singleton().get_ticks_msec();
        godot_print!("Took {}ms to answer question", time_taken);

        room.remove_child(&panel);
        self.question_bank.current_difficulty =
            (self.question_bank.current_difficulty - 0.1).clamp(0., 1.);
        let question = self.question_bank.get_question();
        let mut panel = QuestionPanel::create_panel(
            GString::from(question.question),
            question
                .choices
                .iter()
                .map(GString::from)
                .collect::<Array<GString>>(),
            question.answers.iter().cloned().collect::<Array<i64>>(),
        );
        panel.set_position(Vector3::new(0., 5., 0.));
        panel.set_name(&GString::from("QuestionPanel"));
        room.add_child(&panel);
    }

    #[func]
    pub fn get_world(scene_tree: Gd<SceneTree>) -> Option<Gd<Self>> {
        let path: NodePath =
            NodePath::from_str("/root/Root/World").expect("failed to create world node path");
        scene_tree
            .get_root()
            .unwrap()
            .get_node_or_null(&path)?
            .try_cast::<Self>()
            .ok()
    }

    #[func]
    pub fn get_room(scene_tree: Gd<SceneTree>) -> Option<Gd<Node3D>> {
        let path: NodePath =
            NodePath::from_str("/root/Root/World/Room").expect("failed to create room node path");
        scene_tree
            .get_root()
            .unwrap()
            .get_node_or_null(&path)?
            .try_cast::<Node3D>()
            .ok()
    }
}
