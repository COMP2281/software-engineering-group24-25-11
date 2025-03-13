use std::collections::VecDeque;

use godot::{
    classes::{file_access::ModeFlags, FileAccess},
    global::randi_range,
    prelude::*,
};
use serde::Deserialize;

use crate::resources;

// FIXME: maybe dynamic course loading + selection based of the question bank json
// this would make it easier to add new question banks, therefore improving maintainability.
#[derive(Debug, Default, GodotConvert, Var, Export, Copy, Clone)]
#[godot(via = u8)]
pub enum Course {
    #[default]
    DataFundamentals,
    WebDev,
    IntroToAi,
}

impl From<Course> for GString {
    fn from(value: Course) -> Self {
        match value {
            Course::DataFundamentals => resources::courses::DATA_FUNDAMENTALS.into(),
            Course::WebDev => resources::courses::WEB_DEV.into(),
            Course::IntroToAi => resources::courses::INTRO_TO_AI.into(),
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
    pub fn to_answered(&self, selected: VecDeque<i64>, time_taken_ms: u64) -> AnsweredQuestion {
        AnsweredQuestion {
            question: self.question.clone().into(),
            choices: self.choices.iter().map(GString::from).collect(),
            difficulty: self.difficulty,
            correct_answers: self.answers.iter().cloned().collect(),

            correct: selected.iter().eq(self.answers.iter()),
            given_answers: selected.into_iter().collect(),
            time_taken_ms,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct AnsweredQuestion {
    pub question: GString,
    pub choices: Array<GString>,
    pub difficulty: f64,
    pub correct_answers: Array<i64>,

    pub correct: bool,
    pub given_answers: Array<i64>,
    pub time_taken_ms: u64,
}

#[derive(Debug, Default, Clone)]
pub struct QuestionBank {
    questions: Vec<Question>,
    mode: Mode,
    pub question_limit: usize,

    pub answered_questions: Vec<AnsweredQuestion>,
    pub score: u64,
    pub current_difficulty: f64,
    pub start_time: u64,
}

impl QuestionBank {
    pub fn new(
        course: Course,
        mode: Mode,
        question_limit: Option<usize>,
        start_time: u64,
    ) -> Option<Self> {
        let course_path: GString = course.into();
        let mut file = FileAccess::open(&course_path, ModeFlags::READ)
            .expect("failed to open course question bank");
        let contents = file.get_buffer(file.get_length() as i64);
        let contents = contents.as_slice();
        file.close();

        let questions: Vec<Question> = serde_json::from_slice(contents).expect("failed to parse");
        let num_questions = questions.len();

        Some(Self {
            questions,
            mode,
            question_limit: question_limit.unwrap_or(num_questions),

            answered_questions: Vec::new(),
            score: 0,
            start_time,
            current_difficulty: mode.into(),
        })
    }

    pub fn get_question(&mut self) -> Option<Question> {
        if self.answered_questions.len() == self.question_limit {
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

        Some(self.questions.remove(idx))
    }

    pub fn submit_answer(&mut self, question: AnsweredQuestion) {
        godot_print!("question took {}ms to answer", question.time_taken_ms);
        self.adjust_difficulty(&question);
        self.add_score(&question);
        self.answered_questions.push(question);
    }

    fn adjust_difficulty(&mut self, question: &AnsweredQuestion) {
        let difficulty_factor = question.difficulty * 0.1;
        let time_factor = (question.time_taken_ms as f64) * 0.000001;
        let base_adjustment = (difficulty_factor + time_factor).clamp(0.0, 0.2);
        self.current_difficulty = (self.current_difficulty
            + (if question.correct {
                base_adjustment
            } else {
                -base_adjustment
            }))
        .clamp(0., 1.);
    }

    fn add_score(&mut self, question: &AnsweredQuestion) {
        // percentage of correct answers in their given answers.
        let correctness_percentage = question
            .given_answers
            .iter_shared()
            .filter(|x| question.correct_answers.contains(*x))
            .count() as f64
            / question.correct_answers.len() as f64;

        let difficulty_factor = 100. * question.difficulty;
        let time_factor = 20. / (0.00005 * question.time_taken_ms as f64).min(1.);
        godot_print!("adding to score: difficulty : {difficulty_factor} + time : {time_factor} * correctness : {correctness_percentage}");
        let adjustment = (difficulty_factor + time_factor).clamp(10., 250.);
        self.score += (adjustment * correctness_percentage) as u64;
    }
}
