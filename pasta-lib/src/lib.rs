use std::str::FromStr;
use wasm_bindgen::prelude::*;

use chrono::{DateTime, Datelike, Timelike};
const HOLIDAYS: &str = include_str!("../data/holidays.csv");

#[derive(Debug)]
#[wasm_bindgen]
pub struct Holiday {
    date: String,
    name: String,
}

/// A pastafarian holiday
#[wasm_bindgen]
impl Holiday {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn date(&self) -> String {
        self.date.clone()
    }
}

impl FromStr for Holiday {
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

/// Get the pastafarian holiday for a given date provided as an ISO8601 date string
#[wasm_bindgen]
pub fn get_holiday_for_date_string(date: &str) -> Result<Holiday, String> {
    chrono::DateTime::parse_from_rfc3339(date)
        .map_err(|_| "Failed to parse ISO8601 date".to_owned())
        .and_then(|date| {
            get_holiday_for_date(date).ok_or_else(|| "Didn't find holiday for date".to_owned())
        })
}

/// Get the pastafarian holiday for a given date
pub fn get_holiday_for_date<T: chrono::TimeZone>(date: DateTime<T>) -> Option<Holiday> {
    // Get days since 1st Jan
    let date = date.with_hour(0).unwrap().with_minute(0).unwrap();
    let first_day = date.with_day0(0).unwrap().with_month0(0).unwrap();
    let days_since = date.signed_duration_since(first_day).num_days() as usize;

    // Index into holidays using days since
    let day = HOLIDAYS.lines().nth(days_since + 1);
    day.and_then(|day_text| Holiday::from_str(day_text).ok())
}
