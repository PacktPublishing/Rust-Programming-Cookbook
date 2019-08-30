use csv;
use serde_json as json;
use std::io;

fn to_json(headers: &csv::StringRecord, current_row: csv::StringRecord) -> io::Result<json::Value> {
    let row: json::Map<String, json::Value> = headers
        .into_iter()
        .zip(current_row.into_iter())
        .map(|(key, value)| (key.to_string(), json::Value::String(value.into())))
        .collect();
    Ok(json::Value::Object(row))
}

fn main() -> io::Result<()> {
    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .has_headers(false)
        .delimiter(b',')
        .from_reader(io::stdin());

    let header_rec = rdr
        .records()
        .take(1)
        .next()
        .expect("The first line does not seem to be a valid CSV")?;
        
    for result in rdr.records() {
        if let Ok(json_rec) = to_json(&header_rec, result?) {
            println!("{}", json_rec.to_string());
        }
    }
    Ok(())
}
