use crate::model::Candle;
use serde::Deserialize;

#[derive(Deserialize)]
struct Record {
    date: String,
    close: f64,
}

pub fn load_csv(path: &str) -> Vec<Candle> {
    let mut rdr = csv::Reader::from_path(path).expect("Cannot open CSV");
    let mut data = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result.expect("Invalid record");
        data.push(Candle {
            date: record.date,
            close: record.close,
        });
    }

    data
}
