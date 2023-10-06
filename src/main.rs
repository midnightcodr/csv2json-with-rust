use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead, BufReader};
type JsonObject = serde_json::Map<String, serde_json::Value>;

fn make_json_object(headers: &csv::StringRecord, record: csv::StringRecord) -> JsonObject {
    let mut j = JsonObject::new();
    for (index, header) in headers.iter().enumerate() {
        j.insert(header.to_string(), record[index].into());
    }
    j
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = env::args().nth(1);
    let reader: Box<dyn BufRead> = match input {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap())),
    };

    let mut csvrdr = csv::Reader::from_reader(reader);
    let headers = csvrdr.headers()?.clone();
    for result in csvrdr.records() {
        let result = result?;
        let jo = make_json_object(&headers, result);
        println!("{}", serde_json::to_string(&jo).unwrap());
    }
    Ok(())
}
