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
        description: String::from("1st Sunday of Advent"),
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
                DayClass::Solemnity,
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
        class: DayClass::Feast,
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

fn ordinal_suffix(number: i32) -> String {
    let str = number.to_string();
    match str {
        _ if str.ends_with("11") => String::from("th"),
        _ if str.ends_with("12") => String::from("th"),
        _ if str.ends_with("13") => String::from("th"),
        _ if str.ends_with("1") => String::from("st"),
        _ if str.ends_with("2") => String::from("nd"),
        _ if str.ends_with("3") => String::from("rd"),
        _ => String::from("th"),
    }
}

fn day_string(wd: Weekday) -> String {
    match wd {
        Weekday::Sun => String::from("Sunday"),
        Weekday::Mon => String::from("Monday"),
        Weekday::Tue => String::from("Tuesday"),
        Weekday::Wed => String::from("Wednesday"),
        Weekday::Thu => String::from("Thursday"),
        Weekday::Fri => String::from("Friday"),
        Weekday::Sat => String::from("Saturday"),
    }
}

fn calculate_advent(start: &ChurchDay) -> Vec<ChurchDay> {
    let mut advent_days: Vec<ChurchDay> = Vec::new();

    let mut sunday_counter = 1;

    advent_days.push(start.clone());
    loop {
        let last = advent_days.last().cloned().unwrap();
        //if the last one in the list is already christmas eve, we stop
        if last.date.month() == 12 && last.date.day() == 24 {
            break;
        }
        //otherwise we add a new one
        let today = last.date + Duration::days(1);
        let day_string = day_string(today.weekday());
        if today.weekday() == Weekday::Sun {
            sunday_counter += 1;
        }
        let new_day = ChurchDay {
            date: today,
            season: Season::Advent,
            class: match today.weekday() {
                Weekday::Sun => DayClass::Sunday,
                _ => DayClass::SeasonalWeekday,
            },
            //advent days between dec 17 and 24 have a special name for LotH stuff
            description: if today.day() >= 17 && today.month() == 12 {
                format!(
                    "December {}{}",
                    today.day(),
                    ordinal_suffix(today.day().try_into().unwrap())
                )
            } else {
                match today.weekday() {
                    Weekday::Sun => format!(
                        "{}{} Sunday of Advent",
                        sunday_counter,
                        ordinal_suffix(sunday_counter)
                    ),
                    _ => format!(
                        "{} of the {}{} week of Advent",
                        day_string,
                        sunday_counter,
                        ordinal_suffix(sunday_counter)
                    ),
                }
            },
        };
        advent_days.push(new_day);
    }

    advent_days
}

fn calculate_christmas(year: i32) -> Vec<ChurchDay> {
    let mut christmas_days: Vec<ChurchDay> = Vec::new();
    let epiphany = epiphany(year + 1).expect("Unable to calculate Epiphany.");
    let baptism = baptism(epiphany.clone());

    let christmas = ChurchDay::new(
        year,
        12,
        25,
        DayClass::Solemnity,
        Season::Christmas,
        "The Nativity of the Lord (Christmas)",
    );

    //add christmas
    christmas_days.push(christmas);
    //add days from christmas to epiphany
    loop {
        let last = christmas_days.last().cloned().unwrap();
        if last.date.month() == epiphany.date.month()
            && last.date.day() == (epiphany.date.day() - 1)
        {
            break;
        }
        let today = last.date + Duration::days(1);
        //the only sunday between christmas and epiphany is holy family
        let new_day = ChurchDay {
            date: today,
            season: Season::Christmas,
            class: match today.weekday() {
                Weekday::Sun => DayClass::Feast,
                _ => DayClass::SeasonalWeekday,
            },
            description: match today.weekday() {
                Weekday::Sun => String::from("The Holy Family of Jesus, Mary, and Joseph"),
                _ => match today.day() {
                    26 => String::from("Saint Stephen, First Martyr"),
                    27 => String::from("Saint John, Apostle and Evangelist"),
                    28 => String::from("Holy Innocents"),
                    29 => String::from("Fifth Day within the Octave of the Nativity of the Lord"),
                    30 => String::from("Sixth Day within the Octave of the Nativity of the Lord"),
                    31 => String::from("Seventh Day within the Octave of the Nativity of the Lord"),
                    _ => String::from("Christmas Weekday"),
                },
            },
        };

        christmas_days.push(new_day);
    }
    //add epiphany
    christmas_days.push(epiphany);
    //add days from epiphany to baptism
    loop {
        let last = christmas_days.last().cloned().unwrap();
        if last.date.month() == baptism.date.month() && last.date.day() == (baptism.date.day() - 1)
        {
            break;
        }
        let today = last.date + Duration::days(1);
        let new_day = ChurchDay {
            date: today,
            season: Season::Christmas,
            class: DayClass::SeasonalWeekday,
            description: format!(
                "{} between Epiphany and Baptism of the Lord",
                day_string(today.weekday()),
            ),
        };
        christmas_days.push(new_day);
    }
    //add baptism
    christmas_days.push(baptism);
    christmas_days
}

fn calculate_lent(ash_wednesday: ChurchDay) -> Vec<ChurchDay> {
    let mut lent_days: Vec<ChurchDay> = Vec::new();
    let mut sunday_counter = 0;

    let easter = easter_for_year(ash_wednesday.date.year());
    lent_days.push(ash_wednesday);

    loop {
        let last = lent_days.last().cloned().unwrap();
        //we stop before palm sunday
        if last.date == easter.date - Duration::days(8) {
            break;
        }
        let today = last.date + Duration::days(1);
        let day_string = day_string(today.weekday());
        if today.weekday() == Weekday::Sun {
            sunday_counter += 1;
        }
        let new_day = ChurchDay {
            date: today,
            season: Season::Lent,
            class: match today.weekday() {
                Weekday::Sun => DayClass::Sunday,
                _ => DayClass::SeasonalWeekday,
            },
            description: if sunday_counter == 0 {
                format!("{} after Ash Wednesday", day_string)
            } else {
                match today.weekday() {
                    Weekday::Sun => format!(
                        "{}{} Sunday of Lent",
                        sunday_counter,
                        ordinal_suffix(sunday_counter)
                    ),
                    _ => format!(
                        "{} of the {}{} week of Lent",
                        day_string,
                        sunday_counter,
                        ordinal_suffix(sunday_counter)
                    ),
                }
            },
        };
        lent_days.push(new_day);
    }
    lent_days
}

fn calculate_easter(sat_before_palm_sun: ChurchDay) -> Vec<ChurchDay> {
    let mut easter_days: Vec<ChurchDay> = Vec::new();

    //first we add holy week
    let ps_date = easter_for_year(sat_before_palm_sun.date.year()).date - Duration::days(7);
    let palm_sunday = ChurchDay {
        date: ps_date,
        class: DayClass::Sunday,
        season: Season::Lent,
        description: String::from("Palm Sunday of the Passion of the Lord"),
    };
    easter_days.push(palm_sunday);

    for &days_after in &[1, 2, 3, 4, 5, 6] {
        let date = ps_date + Duration::days(days_after);
        easter_days.push(ChurchDay {
            date,
            class: if days_after < 4 {
                DayClass::SeasonalWeekday
            } else {
                DayClass::Solemnity
            },
            season: if days_after < 4 {
                Season::Lent
            } else {
                Season::Triduum
            },
            description: String::from(match days_after {
                1 => "Monday of Holy Week",
                2 => "Tuesday of Holy Week",
                3 => "Wednesday of Holy Week",
                4 => "Holy Thursday",
                5 => "Friday of the Passion of the Lord",
                6 => "Holy Saturday",
                _ => "",
            }),
        });
    }

    //then we add the easter octave
    let easter = easter_for_year(ps_date.year());
    easter_days.push(easter.clone());
    for &octave_days in &[1, 2, 3, 4, 5, 6, 7] {
        let date = easter.date + Duration::days(octave_days);
        easter_days.push(ChurchDay {
            date,
            class: DayClass::Solemnity,
            season: Season::Easter,
            description: String::from(match octave_days {
                1 => "Monday within the Octave of Easter",
                2 => "Tuesday within the Octave of Easter",
                3 => "Wednesday within the Octave of Easter",
                4 => "Thursday within the Octave of Easter",
                5 => "Friday within the Octave of Easter",
                6 => "Saturday within the Octave of Easter",
                7 => "Sunday of Divine Mercy",
                _ => "",
            }),
        });
    }

    //then we do the rest of the season, including pentecost
    let mut sunday_counter = 2;
    let pentecost = pentecost(easter);
    loop {
        let last = easter_days.last().cloned().unwrap();
        if last.date == pentecost.date - Duration::days(1) {
            break;
        }
        let today = last.date + Duration::days(1);
        let day_string = day_string(today.weekday());
        if today.weekday() == Weekday::Sun {
            sunday_counter += 1;
        }
        let new_day = ChurchDay {
            date: today,
            season: Season::Easter,
            class: match today.weekday() {
                Weekday::Sun => DayClass::Sunday,
                _ => DayClass::SeasonalWeekday,
            },
            description: match today.weekday() {
                Weekday::Sun => {
                    if sunday_counter == 7 {
                        format!("The Ascension of the Lord")
                    } else {
                        format!(
                            "{}{} Sunday of Easter",
                            sunday_counter,
                            ordinal_suffix(sunday_counter)
                        )
                    }
                }
                _ => format!(
                    "{} of the {}{} week of Easter",
                    day_string,
                    sunday_counter,
                    ordinal_suffix(sunday_counter)
                ),
            },
        };
        easter_days.push(new_day);
    }
    easter_days.push(pentecost);

    easter_days
}

fn calculate_otime_prelent(baptism: ChurchDay) -> Vec<ChurchDay> {
    let mut ot_days: Vec<ChurchDay> = Vec::new();
    let mut sunday_counter = 1;

    let first = ChurchDay {
        date: baptism.date + Duration::days(1),
        class: DayClass::FerialWeekday,
        season: Season::OrdinaryTime,
        description: String::from("Monday of the 1st week in Ordinary Time"),
    };
    let day_before_ash_wednesday =
        ash_wednesday(&easter_for_year(baptism.date.year())).date - Duration::days(1);
    ot_days.push(first);

    loop {
        let last = ot_days.last().cloned().unwrap();
        //if we already have the day before ash wednesday, stop
        if last.date == day_before_ash_wednesday {
            break;
        }
        //otherwise we add a new one
        let today = last.date + Duration::days(1);
        let day_string = day_string(today.weekday());
        if today.weekday() == Weekday::Sun {
            sunday_counter += 1;
        }
        let new_day = ChurchDay {
            date: today,
            season: Season::OrdinaryTime,
            class: match today.weekday() {
                Weekday::Sun => DayClass::Sunday,
                _ => DayClass::FerialWeekday,
            },
            description: match today.weekday() {
                Weekday::Sun => format!(
                    "{}{} Sunday of Ordinary Time",
                    sunday_counter,
                    ordinal_suffix(sunday_counter)
                ),
                _ => format!(
                    "{} of the {}{} week of Ordinary Time",
                    day_string,
                    sunday_counter,
                    ordinal_suffix(sunday_counter)
                ),
            },
        };
        ot_days.push(new_day);
    }
    ot_days
}

//we have to do this one backwards because the end of ordinary time is how the weeks are calculated
fn calculate_otime_posteaster(pentecost: ChurchDay) -> Vec<ChurchDay> {
    let mut ot_days: Vec<ChurchDay> = Vec::new();
    let mut sunday_counter = 34;

    //we need these dates since we're working backwards
    let trinity_sunday = pentecost.date + Duration::days(7);
    let corpus_christi = trinity_sunday + Duration::days(7);

    let last_liturgical_day = first_sunday_advent(pentecost.date.year()).date - Duration::days(1);
    let last = ChurchDay {
        date: last_liturgical_day,
        class: DayClass::FerialWeekday,
        season: Season::OrdinaryTime,
        description: format!(
            "{} of the {}{} Week in Ordinary Time",
            day_string(last_liturgical_day.weekday()),
            sunday_counter,
            ordinal_suffix(sunday_counter)
        ),
    };
    ot_days.push(last);
    loop {
        let last = ot_days.last().cloned().unwrap();
        if last.date == pentecost.date + Duration::days(1) {
            break;
        }
        let today = last.date - Duration::days(1);
        let day_string = day_string(today.weekday());
        if today.weekday() == Weekday::Sun {
            sunday_counter -= 1;
        }
        let new_day = ChurchDay {
            date: today,
            season: Season::OrdinaryTime,
            class: match today.weekday() {
                Weekday::Sun => {
                    if (today == trinity_sunday)
                        || (today == corpus_christi)
                        || (sunday_counter == 33)
                    {
                        DayClass::Solemnity
                    } else {
                        DayClass::Sunday
                    }
                }
                _ => DayClass::FerialWeekday,
            },
            description: match today.weekday() {
                Weekday::Sun => {
                    if today == trinity_sunday {
                        String::from("The Most Holy Trinity")
                    } else if today == corpus_christi {
                        String::from("The Most Holy Body and Blood of Christ")
                    } else if sunday_counter == 33 {
                        String::from("Our Lord Jesus Christ, King of the Universe")
                    } else {
                        format!(
                            "{}{} Sunday of Ordinary Time",
                            sunday_counter,
                            ordinal_suffix(sunday_counter)
                        )
                    }
                }
                _ => format!(
                    "{} of the {}{} week of Ordinary Time",
                    day_string,
                    sunday_counter,
                    ordinal_suffix(sunday_counter)
                ),
            },
        };
        ot_days.push(new_day);
    }

    //reverse the whole list because we did it backwards, then see if it lines up, i guess
    ot_days.reverse();
    ot_days
}

pub fn proper_of_seasons(today: NaiveDate) -> ChurchYear {
    let mut days: Vec<ChurchDay> = Vec::new();
    //figure out whether we wanna start with the current year's 'first sunday of advent', or last
    //year
    let mut first_advent = first_sunday_advent(today.year());
    if today < first_advent.date {
        first_advent = first_sunday_advent(today.year() - 1);
    }

    let mut advent_days = calculate_advent(&first_advent);
    days.append(&mut advent_days);

    let mut christmas_days = calculate_christmas(first_advent.date.year());
    days.append(&mut christmas_days);

    let baptism = days.last().cloned().unwrap();
    let easter = easter_for_year(baptism.date.year());
    let ash_wednesday = ash_wednesday(&easter);

    let mut ot_prelent = calculate_otime_prelent(baptism);
    days.append(&mut ot_prelent);

    let mut lent = calculate_lent(ash_wednesday);
    days.append(&mut lent);

    let sat_before_palm_sun = days.last().cloned().unwrap();
    let mut easter = calculate_easter(sat_before_palm_sun);
    days.append(&mut easter);

    let pentecost = days.last().cloned().unwrap();
    let mut ot_posteaster = calculate_otime_posteaster(pentecost);
    days.append(&mut ot_posteaster);

    ChurchYear { days }
}
