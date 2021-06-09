use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HolidayRecord {
    date: String,
    holiday: String,
}

