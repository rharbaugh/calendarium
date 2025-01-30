use crate::church_day::*;
use chrono::*;

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
        "Easter Sunday of the Resurrection of the Lord",
    )
}

fn ash_wednesday(easter: &ChurchDay) -> ChurchDay {
    let date = easter.date - Duration::days(46);
    ChurchDay {
        date,
        class: DayClass::Solemnity,
        season: Season::Lent,
        description: String::from("Ash Wednesday"),
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
        description: String::from("First Sunday of Advent"),
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
                "The Epiphany of the Lord",
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
        description: String::from("The Baptism of the Lord"),
    }
}

fn pentecost(easter: ChurchDay) -> ChurchDay {
    let date = easter.date + Duration::days(49);
    ChurchDay {
        date,
        class: DayClass::Solemnity,
        season: Season::Easter,
        description: String::from("Pentecost Sunday"),
    }
}

fn holy_week(easter: ChurchDay) -> Vec<ChurchDay> {
    let mut triduum = Vec::new();
    for &days_before in &[7, 6, 5, 4, 3, 2, 1] {
        let date = easter.date - Duration::days(days_before);
        triduum.push(ChurchDay {
            date,
            class: DayClass::Solemnity,
            season: if days_before < 4 {
                Season::Triduum
            } else {
                Season::Lent
            },
            description: String::from(match days_before {
                7 => "Palm Sunday of the Passion of the Lord",
                6 => "Monday of Holy Week",
                5 => "Tuesday of Holy Week",
                4 => "Wednesday of Holy Week",
                3 => "Thursday of Holy Week (Holy Thursday)",
                2 => "Good Friday",
                1 => "Holy Saturday",
                _ => "If you got here, something went real bad",
            }),
        });
    }
    triduum
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
            description: String::from(""),
        });
        date += Duration::days(1);
    }
}

fn add_days_including(
    days: &mut Vec<ChurchDay>,
    start_date: NaiveDate,
    end_date: &ChurchDay,
    season: Season,
    class_fn: fn(Weekday) -> DayClass,
) {
    let mut date = start_date;
    while date < end_date.date {
        let class = class_fn(date.weekday());
        days.push(ChurchDay {
            date,
            class,
            season,
            description: String::from(""),
        });
        date += Duration::days(1);
    }
    days.push(end_date.clone());
}

fn update_descriptions(days: &mut Vec<ChurchDay>) {}

pub fn build_church_year(today: DateTime<Local>) -> ChurchYear {
    let year = today.year();
    let easter = easter_for_year(year);
    let ash_wednesday = ash_wednesday(&easter);
    let first_sunday_advent = first_sunday_advent(year);
    let epiphany = epiphany(year).expect("Unable to calculate Epiphany.");
    let baptism = baptism(epiphany.clone());
    let pentecost = pentecost(easter.clone());
    let mut holy_week = holy_week(easter.clone());

    let mut days: Vec<ChurchDay> = Vec::new();

    add_days_including(
        &mut days,
        NaiveDate::from_ymd_opt(year, 1, 1).expect("Unable to create date."),
        &epiphany,
        Season::Christmas,
        day_class,
    );

    add_days_including(
        &mut days,
        epiphany.date + Duration::days(1),
        &baptism,
        Season::Christmas,
        day_class,
    );

    days.push(baptism.clone());

    add_days_including(
        &mut days,
        baptism.date + Duration::days(1),
        &ash_wednesday,
        Season::OrdinaryTime,
        ordinary_time_class,
    );

    add_days_until(
        &mut days,
        ash_wednesday.date + Duration::days(1),
        easter.date - Duration::days(7),
        Season::Lent,
        day_class,
    );

    days.append(&mut holy_week);
    days.push(easter.clone());

    add_days_including(
        &mut days,
        easter.date + Duration::days(1),
        &pentecost,
        Season::Easter,
        day_class,
    );

    add_days_including(
        &mut days,
        pentecost.date + Duration::days(1),
        &first_sunday_advent,
        Season::OrdinaryTime,
        ordinary_time_class,
    );

    let christmas = ChurchDay::new(
        year,
        12,
        25,
        DayClass::Solemnity,
        Season::Christmas,
        "The Nativity of the Lord (Christmas)",
    );
    add_days_including(
        &mut days,
        first_sunday_advent.date + Duration::days(1),
        &christmas,
        Season::Advent,
        day_class,
    );

    add_days_until(
        &mut days,
        christmas.date + Duration::days(1),
        NaiveDate::from_ymd_opt(year + 1, 1, 1).expect("Unable to create end date for year"),
        Season::Christmas,
        day_class,
    );

    update_descriptions(&mut days);

    ChurchYear { days }
}
