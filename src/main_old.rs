use chrono::prelude::*;
use std::fmt;

#[derive(Copy, Clone)]
enum DayClass {
    Solemnity,
    Sunday,
    Feast,
    Memorial,
    SeasonalWeekday,
    FerialWeekday,
    DayOfFastingAndAbstinenceFromDefinition,
}

impl fmt::Display for DayClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DayClass::Solemnity => write!(f, "Solemnity"),
            DayClass::Sunday => write!(f, "Sunday"),
            DayClass::Feast => write!(f, "Feast"),
            DayClass::Memorial => write!(f, "Memorial"),
            DayClass::SeasonalWeekday => write!(f, "Seasonal Weekday"),
            DayClass::FerialWeekday => write!(f, "Ferial"),
            DayClass::DayOfFastingAndAbstinenceFromDefinition => {
                write!(f, "Day of Fasting and Abstinence ©aa4jq")
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Season {
    Advent,
    Christmas,
    OrdinaryTime,
    Lent,
    Triduum,
    Easter,
}

impl fmt::Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Season::Advent => write!(f, "Advent"),
            Season::Christmas => write!(f, "Christmas"),
            Season::OrdinaryTime => write!(f, "Ordinary Time"),
            Season::Lent => write!(f, "Lent"),
            Season::Triduum => write!(f, "Sacred Triduum"),
            Season::Easter => write!(f, "Easter"),
        }
    }
}

#[derive(Copy)]
struct ChurchDay {
    year: i32,
    month: i32,
    date: i32,
    class: DayClass,
    season: Season,
}

impl Clone for ChurchDay {
    fn clone(&self) -> Self {
        Self {
            year: self.year.clone(),
            month: self.month.clone(),
            date: self.date.clone(),
            class: self.class.clone(),
            season: self.season.clone(),
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
        season: Season::Easter,
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
    ash_wednesday.class = DayClass::DayOfFastingAndAbstinenceFromDefinition;
    ash_wednesday.season = Season::Lent;
    ash_wednesday
}

fn first_sunday_advent(year: i32) -> ChurchDay {
    let dec_first = ChurchDay {
        year,
        month: 12,
        date: 1,
        class: DayClass::Sunday,
        season: Season::Advent,
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

fn epiphany(year: i32) -> Option<ChurchDay> {
    //first sunday between jan 2 and 6
    for date in 2..9 {
        let day = ChurchDay {
            year,
            month: 1,
            date,
            class: DayClass::Sunday,
            season: Season::Christmas,
        };
        if day.day_of_week() == 0 {
            return Some(day);
        }
    }
    None
}

fn baptism(epiphany: ChurchDay) -> ChurchDay {
    add_days(&epiphany, 7)
}

fn pentecost(easter: ChurchDay) -> ChurchDay {
    add_days(&easter, 49)
}

fn triduum(easter: ChurchDay) -> Vec<ChurchDay> {
    let mut triduum = Vec::new();
    triduum.push(subtract_days(&easter, 3));
    triduum.push(subtract_days(&easter, 2));
    triduum.push(subtract_days(&easter, 1));
    triduum
}

fn build_church_year(today: DateTime<Local>) -> ChurchYear {
    //find epiphany
    //baptism is always the sunday after the epiphany
    //jan 1 to baptism is 'christmas' season
    //day after baptism until ash wednesday is ordinary time
    //ash wednesday until triduum is lent
    //triduum
    //easter season until pentecost
    //day after pentecost until first sunday of advent is ordinary time
    //advent until christmas
    //christmas until jan 31

    let easter = easter_for_year(today.year());
    let ash_wednesday = ash_wednesday(&easter);
    let first_sunday_advent = first_sunday_advent(today.year());
    let epiphany = epiphany(today.year()).expect("Unable to calculate Epiphany.");
    let baptism = baptism(epiphany);
    let pentecost = pentecost(easter);
    let mut triduum = triduum(easter);

    let mut days: Vec<ChurchDay> = Vec::new();
    let mut day_base = ChurchDay {
        year: today.year(),
        month: 1,
        date: 1,
        class: DayClass::FerialWeekday,
        season: Season::OrdinaryTime,
    };

    //insert days from january 1 until epiphany
    while !(day_base.month == epiphany.month && day_base.date == epiphany.date) {
        day_base.season = Season::Christmas;

        if day_base.day_of_week() == 0 {
            day_base.class = DayClass::Sunday;
        } else {
            day_base.class = DayClass::SeasonalWeekday;
        }

        days.push(day_base.clone());
        day_base = add_days(&day_base, 1);
    }

    //then insert epiphany
    days.push(epiphany);
    day_base = add_days(&epiphany, 1);

    //days from epiphany to baptism
    while !(day_base.month == baptism.month && day_base.date == baptism.date) {
        day_base.season = Season::Christmas;

        if day_base.day_of_week() == 0 {
            day_base.class = DayClass::Sunday;
        } else {
            day_base.class = DayClass::SeasonalWeekday;
        }

        days.push(day_base.clone());
        day_base = add_days(&day_base, 1);
    }

    //then insert baptism
    days.push(baptism);
    day_base = add_days(&baptism, 1);

    //days from baptism until ash wednesday
    while !(day_base.month == ash_wednesday.month && day_base.date == ash_wednesday.date) {
        day_base.season = Season::OrdinaryTime;

        if day_base.day_of_week() == 0 {
            day_base.class = DayClass::Sunday;
        } else {
            day_base.class = DayClass::FerialWeekday;
        }

        days.push(day_base.clone());
        day_base = add_days(&day_base, 1);
    }

    //then insert ash wednesday
    days.push(ash_wednesday);
    day_base = add_days(&ash_wednesday, 1);

    //days from ash wednesday until the triduum
    let holy_thursday = subtract_days(&easter, 3);
    while !(day_base.month == holy_thursday.month && day_base.date == holy_thursday.date) {
        day_base.season = Season::Lent;

        if day_base.day_of_week() == 0 {
            day_base.class = DayClass::Sunday;
        } else {
            day_base.class = DayClass::SeasonalWeekday;
        }
        days.push(day_base.clone());
        day_base = add_days(&day_base, 1);
    }

    //triduum
    days.append(&mut triduum);
    //easter
    days.push(easter);
    day_base = add_days(&easter, 1);

    //days from easter until pentecost
    while !(day_base.month == pentecost.month && day_base.date == pentecost.date) {
        day_base.season = Season::Easter;

        if day_base.day_of_week() == 0 {
            day_base.class = DayClass::Sunday;
        } else {
            day_base.class = DayClass::SeasonalWeekday;
        }
        days.push(day_base.clone());
        day_base = add_days(&day_base, 1);
    }

    //pentecost
    days.push(pentecost);
    day_base = add_days(&pentecost, 1);

    //days from pentecost until first sunday of advent
    while !(day_base.month == first_sunday_advent.month
        && day_base.date == first_sunday_advent.date)
    {
        day_base.season = Season::OrdinaryTime;

        if day_base.day_of_week() == 0 {
            day_base.class = DayClass::Sunday;
        } else {
            day_base.class = DayClass::FerialWeekday;
        }
        days.push(day_base.clone());
        day_base = add_days(&day_base, 1);
    }

    //first sunday of advent
    days.push(first_sunday_advent);
    day_base = add_days(&first_sunday_advent, 1);

    //days from first sunday of advent until christmas
    while !(day_base.month == 12 && day_base.date == 25) {
        day_base.season = Season::Advent;

        if day_base.day_of_week() == 0 {
            day_base.class = DayClass::Sunday;
        } else {
            day_base.class = DayClass::SeasonalWeekday;
        }
        days.push(day_base.clone());
        day_base = add_days(&day_base, 1);
    }

    //christmas
    let christmas = ChurchDay {
        year: today.year(),
        month: 12,
        date: 25,
        class: DayClass::Solemnity,
        season: Season::Christmas,
    };
    days.push(christmas);
    day_base = add_days(&christmas, 1);

    //days from christmas until jan 31
    while !(day_base.month == 1 && day_base.date == 1) {
        day_base.season = Season::Christmas;

        if day_base.day_of_week() == 0 {
            day_base.class = DayClass::Sunday;
        } else {
            day_base.class = DayClass::SeasonalWeekday;
        }
        days.push(day_base.clone());
        day_base = add_days(&day_base, 1);
    }

    ChurchYear { days }
}

fn main() {
    let today = chrono::Local::now();
    let year = build_church_year(today);
    for day in year.days.iter() {
        println!(
            "{}, {} {}, {} - {} in {}",
            day.string_day_of_week(),
            day.month_name(),
            day.date,
            day.year,
            day.class,
            day.season
        );
    }
}
