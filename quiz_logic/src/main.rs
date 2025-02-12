use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file_path = "question_bank/ai_modules.csv";
    let file = File::open(file_path)?;

    // Create a CSV reader
    let mut rdr = ReaderBuilder::new()
        .has_headers(true) // Set to true if your CSV has headers
        .from_reader(file);

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    

    Ok(())
}

