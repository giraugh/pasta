mod holiday_record;
use holiday_record::parse_holidays;
use chrono::prelude::Local;

fn main() {
    let holidays = parse_holidays();
    let current_date = Local::now().format("%b%e").to_string();
    for holiday in holidays.iter() {
        //println!("'{}' == '{}'", holiday.date.trim(), current_date.trim());
        if holiday.date.trim().eq(current_date.trim()) {
            println!("{}", holiday.holiday);
            break;
        }
    }
}

