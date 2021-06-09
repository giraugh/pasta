use std::fs::File;
mod holiday_record;
use holiday_record::HolidayRecord;

fn main() {
    let holidays = parse_holidays();
    for holiday in holidays.iter() {
        println!("{:?}", holiday);
    }
}

fn parse_holidays() -> Vec<HolidayRecord> {
    let f = File::open("holidays.csv").expect("Failed to open csv file.");
    let mut rdr = csv::Reader::from_reader(f);
    rdr
        .deserialize()
        .map(|r| r.expect("Failed to deserialise csv record"))
        .collect()
}
