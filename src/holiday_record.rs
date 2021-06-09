use serde::Deserialize;

const HOLIDAYS : &'static str = include_str!("../holidays.csv");

#[derive(Debug, Deserialize)]
pub struct HolidayRecord {
    pub date: String,
    pub holiday: String,
}

pub fn parse_holidays() -> Vec<HolidayRecord> {
    let mut rdr = csv::Reader::from_reader(HOLIDAYS.as_bytes());
    rdr
        .deserialize()
        .map(|r| r.expect("Failed to deserialise csv record"))
        .collect()
}
