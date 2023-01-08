use std::str::FromStr;

use chrono::{DateTime, Datelike, Local, Timelike};
const HOLIDAYS: &str = include_str!("../data/holidays.csv");

#[derive(Debug)]
pub struct HolidayRecord {
    pub date: String,
    pub name: String,
}

impl FromStr for HolidayRecord {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (date, name) = s
            .split_once(',')
            .ok_or_else(|| "Malformed holiday record".to_owned())?;
        let date = date.to_owned();
        let name = name.strip_suffix(',').unwrap_or(name).to_owned();
        Ok(Self { date, name })
    }
}

pub fn get_holiday_for_date(date: DateTime<Local>) -> Option<HolidayRecord> {
    // Get days since 1st Jan
    let date = date.with_hour(0).unwrap().with_minute(0).unwrap();
    let first_day = date.with_day0(0).unwrap().with_month0(0).unwrap();
    let days_since = date.signed_duration_since(first_day).num_days() as usize;

    // Index into holidays using days since
    let day = HOLIDAYS.lines().nth(days_since + 1);
    day.and_then(|day_text| HolidayRecord::from_str(day_text).ok())
}
