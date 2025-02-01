use chrono::{Datelike, Local};

mod church_day;

mod builder;
use builder::*;

fn main() {
    let today = Local::now();

    let seasons = proper_of_seasons(today.naive_local().into());

    for day in seasons.days.iter() {
        println!(
            //"{}, {} {}, {} - {}",
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
