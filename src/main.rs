use chrono::prelude::*;

#[derive(Copy, Clone)]
enum DayClass {
    Solemnity,
    Sunday,
    Feast,
}

#[derive(Copy)]
struct ChurchDay {
    year: i32,
    month: i32,
    date: i32,
    class: DayClass,
}

impl Clone for ChurchDay {
    fn clone(&self) -> Self {
        Self {
            year: self.year.clone(),
            month: self.month.clone(),
            date: self.date.clone(),
            class: self.class.clone(),
        }
    }
}

impl ChurchDay {
    fn month_name(&self) -> String {
        match self.month {
            1 => String::from("January"),
            2 => String::from("February"),
            3 => String::from("March"),
            4 => String::from("April"),
            5 => String::from("May"),
            6 => String::from("June"),
            7 => String::from("July"),
            8 => String::from("August"),
            9 => String::from("September"),
            10 => String::from("October"),
            11 => String::from("November"),
            _ => String::from("December"),
        }
    }

    fn string_day_of_week(&self) -> String {
        match self.day_of_week() {
            0 => String::from("Sunday"),
            1 => String::from("Monday"),
            2 => String::from("Tuesday"),
            3 => String::from("Wednesday"),
            4 => String::from("Thursday"),
            5 => String::from("Friday"),
            _ => String::from("Saturday"),
        }
    }
    fn day_of_week(&self) -> i32 {
        //(Year Code + Month Code + Century Code + Date Number - Leap Year Code) mod 7
        let last_two = self.year % 100;
        let year_code = (last_two + (last_two / 4)) % 7;

        let month_code = match self.month {
            2 => 3,
            3 => 3,
            4 => 6,
            5 => 1,
            6 => 4,
            7 => 6,
            8 => 2,
            9 => 5,
            11 => 3,
            12 => 5,
            _ => 0,
        };

        let mut century_code = 6;

        // this is a fancy way to make this work for all positive years
        let mut counter = 100;
        loop {
            counter = counter + 100;
            century_code = match century_code {
                6 => 4,
                4 => 2,
                2 => 0,
                0 => 6,
                _ => 0,
            };
            if counter >= self.year {
                break;
            };
        }

        let mut leap_year_code = 0;
        let leap = is_leap_year(self.year);
        if leap && (self.month == 1 || self.month == 2) {
            leap_year_code = 1;
        }

        //day code is 'days since sunday'
        let day_code = (year_code + month_code + century_code + self.date - leap_year_code) % 7;
        day_code
    }
}

struct ChurchYear {
    days: Vec<ChurchDay>,
}

fn is_leap_year(year: i32) -> bool {
    let by_4 = year % 4 == 0;
    let by_100 = year % 100 == 0;
    let by_400 = year % 400 == 0;
    if by_4 && !by_100 {
        return true;
    } else if by_100 && by_400 {
        return true;
    } else {
        return false;
    }
}

fn easter_for_year(year: i32) -> ChurchDay {
    let c = year / 100;
    let q = (((c - 15) * 3) / 4) + 10;
    let l = 7 - (((year / 4) + year + 4 - q) % 7);
    let g = year % 19 + 1;
    let j = ((g * 11) - 10) % 30;
    let s = q - 10;
    let m = ((c - 14) * 8) / 25;

    let mut intermediate_e = j - s + m;
    loop {
        intermediate_e += 30;
        if intermediate_e > 0 {
            break;
        }
    }

    let mut e = intermediate_e % 30;

    if (g > 11) && (e == 25) {
        e = 26;
    } else if e == 24 {
        e = 25;
    }

    let d;
    if e < 24 {
        d = 44 - e;
    } else {
        d = 74 - 3;
    }

    let w = (d + 10 - l) % 7;

    let mut date = (d + 7) - w;
    let mut month = 3;
    if date >= 32 {
        date = date - 31;
        month = 4;
    }

    ChurchDay {
        year,
        month,
        date,
        class: DayClass::Solemnity,
    }
}

fn days_in_month(month: i32, year: i32) -> i32 {
    match month {
        1 => 31,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 0,
    }
}

fn add_days(start_day: &ChurchDay, amount: i32) -> ChurchDay {
    let mut return_day = start_day.clone();

    let mut days_added = 0;
    while days_added < amount {
        let current_month_length = days_in_month(return_day.month, return_day.year);
        if return_day.date < current_month_length {
            return_day.date += 1;
        }
        // there are two month rollover situations - one where we go forward a
        // month, one where we go forward a year
        else if return_day.date == current_month_length {
            if return_day.month == 12 {
                return_day.year += 1;
                return_day.date = 1;
                return_day.month = 1;
            } else {
                return_day.month += 1;
                return_day.date = 1;
            }
        }
        days_added += 1;
    }
    return_day
}

fn subtract_days(start_day: &ChurchDay, amount: i32) -> ChurchDay {
    let mut return_day = start_day.clone();

    let mut days_subtracted = 0;
    while days_subtracted < amount {
        if return_day.date > 1 {
            return_day.date -= 1;
        } else if return_day.date == 1 {
            if return_day.month == 1 {
                return_day.year -= 1;
                return_day.date = 31;
                return_day.month = 11;
            } else {
                return_day.month -= 1;
                return_day.date = days_in_month(return_day.month, return_day.year);
            }
        }
        days_subtracted += 1;
    }
    return_day
}

fn ash_wednesday(easter: &ChurchDay) -> ChurchDay {
    let mut ash_wednesday = subtract_days(easter, 46);
    ash_wednesday.class = DayClass::Feast;
    ash_wednesday
}

fn first_sunday_advent(year: i32) -> ChurchDay {
    let dec_first = ChurchDay {
        year,
        month: 12,
        date: 1,
        class: DayClass::Sunday,
    };

    let days_to_subtract = match dec_first.day_of_week() {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => -2,
        6 => -1,
        _ => 0,
    };
    if days_to_subtract > 0 {
        subtract_days(&dec_first, days_to_subtract)
    } else if days_to_subtract < 0 {
        add_days(&dec_first, days_to_subtract.abs())
    } else {
        dec_first
    }
}

fn build_church_year(today: DateTime<Local>) -> ChurchYear {
    let easter = easter_for_year(today.year());
    let ash_wednesday = ash_wednesday(&easter);
    let first_sunday_advent = first_sunday_advent(today.year());

    let mut days: Vec<ChurchDay> = Vec::new();
    days.push(ash_wednesday);
    days.push(easter);
    days.push(first_sunday_advent);

    ChurchYear { days }
}

fn main() {
    let today = chrono::Local::now();
    let year = build_church_year(today);
    for day in year.days.iter() {
        println!(
            "{}, {} {}, {}",
            day.string_day_of_week(),
            day.month_name(),
            day.date,
            day.year
        );
    }
}
