use chrono::prelude::Local;
use pasta::get_holiday_for_date;

fn main() {
    // Get holiday
    let current_date = Local::now();
    let holiday = get_holiday_for_date(current_date);

    // Print holiday
    match holiday {
        Some(holiday) => println!("{}", holiday.name),
        None => println!("Didn't find holiday for today :("),
    }
}
