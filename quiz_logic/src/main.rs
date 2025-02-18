use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
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

fn main() -> Result<(), Box<dyn Error>> {
    // open the CSV file
    let file_path = "question_bank/ai_modules.csv";
    let file = File::open(file_path)?;

    // create a CSV reader
    let mut rdr = ReaderBuilder::new()
        .has_headers(true) 
        .from_reader(file);

    let mut questions = Vec::new();

    for result in rdr.deserialize() {
        let mut record: Question = result?;
        if record.difficulty.is_none() {
            record.difficulty = Some(0.5);
        }
        questions.push(record);
        println!("{:?}", record.answer);
    }




    Ok(())
}

