use std::fs::File;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HolidayRecord {
    pub date: String,
    pub holiday: String,
}

pub fn parse_holidays() -> Vec<HolidayRecord> {
    let f = File::open("holidays.csv").expect("Failed to open csv file.");
    let mut rdr = csv::Reader::from_reader(f);
    rdr
        .deserialize()
        .map(|r| r.expect("Failed to deserialise csv record"))
        .collect()
}
