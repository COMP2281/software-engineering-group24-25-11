use csv::ReaderBuilder;
use serde_derive::Deserialize;
use std::fs::File;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Question {
    question: String,
    choice1: String,
    choice2: String,
    choice3: String,
    choice4: String,
    answer: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    difficulty: Option<f64>,
}

struct User {
    name: String,
    score: f64,
    difficulty: f64,
}

enum Mode {
    Easy,
    Normal,
    Hard,
}
enum Course {
    WebDev,
    DataFundamentals,
    AiModules,
}

impl From<Course> for &str {
    fn from(value: Course) -> Self {
        match value {
            Course::WebDev => "res://web_dev.json",
            Course::DataFundamentals => "res://data_fundamentals.json",
            Course::AiModules => "res://ai_modules.json",
        }
    }
}

struct QuestionBank {
    questions: Vec<Question>,
    current_difficulty: f64,
    mode: Mode,
}

impl QuestionBank {
    fn new(mode: Mode, course: Course) -> Result<Self, csv::Error> {
        let file_path = "question_bank/ai_modules.csv";
        let file = File::open(file_path)?;
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
        let mut questions = Vec::new();

        for result in rdr.deserialize() {
            let mut record: Question = result?;
            if record.difficulty.is_none() {
                record.difficulty = Some(0.5);
            }
            println!("{:?}", record.answer);
            questions.push(record);
        }

        Ok(Self {
            questions,
            current_difficulty: match mode {
                Mode::Easy => 0.3,
                Mode::Normal => 0.5,
                Mode::Hard => 0.7,
            },
            mode,
        })
    }

    pub fn get_course_questions(&mut self) {}
}

fn main() {
    let mut qb = QuestionBank::new(Mode::Easy, Course::WebDev);

    // open the CSV file

    // create a CSV reader
}
