use chrono::{Datelike, Local};

mod church_day;

mod builder;
use builder::*;

fn main() {
    let today = Local::now();
    let year = build_church_year(today);
    for day in year.days.iter() {
        println!(
            "{}, {} {}, {} - {} - {} in {}",
            day.string_day_of_week(),
            day.month_name(),
            day.date.day(),
            day.date.year(),
            day.description,
            day.class,
            day.season,
        );
    }
}
