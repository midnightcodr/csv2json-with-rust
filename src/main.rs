use std::error::Error;
use std::io;
type JsonObject = serde_json::Map<String, serde_json::Value>;

fn make_json_object(headers: &csv::StringRecord, record: csv::StringRecord) -> JsonObject {
    let mut j = JsonObject::new();
    for (index, header) in headers.iter().enumerate() {
        j.insert(header.to_string(), record[index].into());
    }
    j
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let headers = rdr.headers()?.clone();
    for result in rdr.records() {
        let result = result?;
        let jo = make_json_object(&headers, result);
        println!("{}", serde_json::to_string(&jo).unwrap());
    }
    Ok(())
}
