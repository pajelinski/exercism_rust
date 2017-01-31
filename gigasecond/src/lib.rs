extern crate chrono;
use chrono::*;


pub fn after(start_date: DateTime<UTC>) -> DateTime<UTC> {
    let gigasecond = Duration::seconds(1000000000);
    start_date + gigasecond
}
