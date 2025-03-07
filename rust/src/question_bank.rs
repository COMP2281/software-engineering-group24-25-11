use godot::{
    classes::{file_access::ModeFlags, FileAccess, Time},
    global::randi_range,
    prelude::*,
};
use serde::Deserialize;

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
    pub question: String,
    pub choices: Vec<String>,
    pub answers: Vec<i64>,
    pub difficulty: f64,
}
impl Question {
    pub fn to_answered(&self, selected: Array<i64>, time_taken_ms: i64) -> AnsweredQuestion {
        AnsweredQuestion {
            question: GString::from(&self.question),
            choices: self.choices.iter().map(GString::from).collect(),
            correct_answers: self.answers.iter().cloned().collect(),
            given_answers: selected.iter_shared().collect(),
            correct: selected.iter_shared().eq(self.answers.iter().cloned()),
            time_taken_ms,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct AnsweredQuestion {
    pub question: GString,
    pub choices: Array<GString>,
    pub correct_answers: Array<i64>,
    pub given_answers: Array<i64>,
    pub correct: bool,
    pub time_taken_ms: i64,
}

#[derive(Debug, Default)]
pub struct QuestionBank {
    questions: Vec<Question>,
    answered_questions: Vec<AnsweredQuestion>,

    question_limit: Option<usize>,
    pub current_difficulty: f64,
    pub start_time: u64,
    mode: Mode,
}

impl QuestionBank {
    pub fn new(course: Course, mode: Mode, question_limit: Option<usize>) -> Option<Self> {
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
            question_limit,
            ..Default::default()
        })
    }

    pub fn get_question(&mut self) -> Option<Question> {
        if self.question_limit == Some(0) {
            return None;
        }

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

        if let Some(limit) = self.question_limit {
            self.question_limit = Some(limit.saturating_sub(1))
        };

        Some(self.questions.remove(idx))
    }

    pub fn adjust_difficulty(&mut self, amount: f64) {
        self.current_difficulty = (self.current_difficulty + amount).clamp(0., 1.);
    }
}
