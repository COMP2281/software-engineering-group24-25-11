use godot::{
    classes::{file_access::ModeFlags, FileAccess, Time},
    global::randi_range,
    prelude::*,
};
use serde::Deserialize;

use crate::scenes::question_panel::QuestionPanel;
#[derive(Debug, Default, GodotConvert, Var, Export, Copy, Clone)]
#[godot(via = u8)]
pub enum Course {
    #[default]
    DataFundamentals,
    WebDev,
    AiModules,
}

impl From<Course> for GString {
    fn from(value: Course) -> Self {
        match value {
            Course::DataFundamentals => crate::resources::COURSE_DATA_FUNDAMENTALS.into(),
            Course::WebDev => crate::resources::COURSE_WEB_DEV.into(),
            Course::AiModules => crate::resources::COURSE_AI_MODULES.into(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, GodotConvert, Var, Export)]
#[godot(via = u8)]
pub enum Mode {
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
    pub current_difficulty: f64,
    pub start_time: u64,
    mode: Mode,
    panel: Option<Gd<QuestionPanel>>,
}

impl QuestionBank {
    pub fn new(course: Course, mode: Mode) -> Option<Self> {
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
            panel: None,
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
    pub fn create_panel(question: Question, mut room: Gd<Node3D>, position: Vector3) {
        let mut panel = QuestionPanel::create_panel(
            GString::from(question.question),
            question
                .choices
                .iter()
                .map(GString::from)
                .collect::<Array<GString>>(),
            question.answers.iter().cloned().collect::<Array<i64>>(),
        );
        // panel.set_position(Vector3::new(0., 5., 0.));
        panel.set_position(position);
        panel.set_name(&GString::from("QuestionPanel"));
        room.add_child(&panel);
    }
    pub fn remove_panel(&mut self) {
        if let Some(mut panel) = self.panel.take() {
            panel.queue_free();
        }
    }
}
