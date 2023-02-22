use csv::ReaderBuilder;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    id: i32,
    name: String,
    email: String,
    age: i32
}

fn main() {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("data.csv")
        .unwrap();

    for result in reader.deserialize::<Record>() {
        match result {
            Ok(record) => {
                // Perform some basic data processing here
                println!("{:?}", record);
            },
            Err(error) => {
                eprintln!("Error: {}", error);
            }
        }
    }
}