use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, Weekday};
use std::convert::TryInto;
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
                write!(f, "Day of Fasting and Abstinence")
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

#[derive(Copy, Clone)]
struct ChurchDay {
    date: NaiveDate,
    class: DayClass,
    season: Season,
}

impl ChurchDay {
    fn new(year: i32, month: u32, day: u32, class: DayClass, season: Season) -> Self {
        Self {
            date: NaiveDate::from_ymd_opt(year, month, day)
                .expect("Unable to instantiate new ChurchDay"),
            class,
            season,
        }
    }

    fn month_name(&self) -> String {
        self.date.format("%B").to_string()
    }

    fn string_day_of_week(&self) -> String {
        self.date.format("%A").to_string()
    }
}

struct ChurchYear {
    days: Vec<ChurchDay>,
}

fn easter_for_year(year: i32) -> ChurchDay {
    // Computus algorithm to calculate Easter Sunday
    let a = year % 19;
    let b = year / 100;
    let c = year % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let month = (h + l - 7 * m + 114) / 31;
    let day = (h + l - 7 * m + 114) % 31 + 1;

    ChurchDay::new(
        year,
        month.try_into().unwrap(),
        day.try_into().unwrap(),
        DayClass::Solemnity,
        Season::Easter,
    )
}

fn ash_wednesday(easter: &ChurchDay) -> ChurchDay {
    let date = easter.date - Duration::days(46);
    ChurchDay {
        date,
        class: DayClass::DayOfFastingAndAbstinenceFromDefinition,
        season: Season::Lent,
    }
}

fn first_sunday_advent(year: i32) -> ChurchDay {
    let dec_first = NaiveDate::from_ymd_opt(year, 12, 1).expect("Unable to create date");
    let weekday = dec_first.weekday();

    let days_to_subtract = match weekday {
        Weekday::Sun => 0,
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => -2,
        Weekday::Sat => -1,
    };
    let date = dec_first - Duration::days(days_to_subtract as i64);
    ChurchDay {
        date,
        class: DayClass::Sunday,
        season: Season::Advent,
    }
}

fn epiphany(year: i32) -> Option<ChurchDay> {
    (2..=6).find_map(|day| {
        let date = NaiveDate::from_ymd_opt(year, 1, day).expect("Unable to create Jan 1");
        if date.weekday() == Weekday::Sun {
            Some(ChurchDay::new(
                year,
                1,
                day,
                DayClass::Sunday,
                Season::Christmas,
            ))
        } else {
            None
        }
    })
}

fn baptism(epiphany: ChurchDay) -> ChurchDay {
    let date = epiphany.date + Duration::days(7);
    ChurchDay {
        date,
        class: DayClass::Sunday,
        season: Season::Christmas,
    }
}

fn pentecost(easter: ChurchDay) -> ChurchDay {
    let date = easter.date + Duration::days(49);
    ChurchDay {
        date,
        class: DayClass::Solemnity,
        season: Season::Easter,
    }
}

fn triduum(easter: ChurchDay) -> Vec<ChurchDay> {
    let mut triduum = Vec::new();
    for &days_before in &[3, 2, 1] {
        let date = easter.date - Duration::days(days_before);
        triduum.push(ChurchDay {
            date,
            class: DayClass::Solemnity,
            season: Season::Triduum,
        });
    }
    triduum
}

fn build_church_year(today: DateTime<Local>) -> ChurchYear {
    let year = today.year();
    let easter = easter_for_year(year);
    let ash_wednesday = ash_wednesday(&easter);
    let first_sunday_advent = first_sunday_advent(year);
    let epiphany = epiphany(year).expect("Unable to calculate Epiphany.");
    let baptism = baptism(epiphany);
    let pentecost = pentecost(easter);
    let mut triduum = triduum(easter);

    let mut days: Vec<ChurchDay> = Vec::new();

    fn add_days_until(
        days: &mut Vec<ChurchDay>,
        start_date: NaiveDate,
        end_date: NaiveDate,
        season: Season,
        class_fn: fn(Weekday) -> DayClass,
    ) {
        let mut date = start_date;
        while date < end_date {
            let class = class_fn(date.weekday());
            days.push(ChurchDay {
                date,
                class,
                season,
            });
            date += Duration::days(1);
        }
    }

    fn day_class(weekday: Weekday) -> DayClass {
        match weekday {
            Weekday::Sun => DayClass::Sunday,
            _ => DayClass::SeasonalWeekday,
        }
    }

    fn ordinary_time_class(weekday: Weekday) -> DayClass {
        match weekday {
            Weekday::Sun => DayClass::Sunday,
            _ => DayClass::FerialWeekday,
        }
    }

    add_days_until(
        &mut days,
        NaiveDate::from_ymd_opt(year, 1, 1).expect("Unable to create date."),
        epiphany.date,
        Season::Christmas,
        day_class,
    );

    days.push(epiphany);

    add_days_until(
        &mut days,
        epiphany.date + Duration::days(1),
        baptism.date,
        Season::Christmas,
        day_class,
    );

    days.push(baptism);

    add_days_until(
        &mut days,
        baptism.date + Duration::days(1),
        ash_wednesday.date,
        Season::OrdinaryTime,
        ordinary_time_class,
    );

    days.push(ash_wednesday);

    let holy_thursday = easter.date - Duration::days(3);
    add_days_until(
        &mut days,
        ash_wednesday.date + Duration::days(1),
        holy_thursday,
        Season::Lent,
        day_class,
    );

    days.append(&mut triduum);
    days.push(easter);

    add_days_until(
        &mut days,
        easter.date + Duration::days(1),
        pentecost.date,
        Season::Easter,
        day_class,
    );

    days.push(pentecost);

    add_days_until(
        &mut days,
        pentecost.date + Duration::days(1),
        first_sunday_advent.date,
        Season::OrdinaryTime,
        ordinary_time_class,
    );

    days.push(first_sunday_advent);

    add_days_until(
        &mut days,
        first_sunday_advent.date + Duration::days(1),
        NaiveDate::from_ymd_opt(year, 12, 25).expect("Unable to create Christmas."),
        Season::Advent,
        day_class,
    );

    let christmas = ChurchDay::new(year, 12, 25, DayClass::Solemnity, Season::Christmas);
    days.push(christmas);

    add_days_until(
        &mut days,
        christmas.date + Duration::days(1),
        NaiveDate::from_ymd_opt(year + 1, 1, 1).expect("Unable to create end date for year"),
        Season::Christmas,
        day_class,
    );

    ChurchYear { days }
}

fn main() {
    let today = Local::now();
    let year = build_church_year(today);
    for day in year.days.iter() {
        println!(
            "{}, {} {}, {} - {} in {}",
            day.string_day_of_week(),
            day.month_name(),
            day.date.day(),
            day.date.year(),
            day.class,
            day.season
        );
    }
}
