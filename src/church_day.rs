use chrono::NaiveDate;
use std::fmt;

#[derive(Copy, Clone)]
pub enum DayClass {
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
pub enum Season {
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
pub struct ChurchDay {
    pub date: NaiveDate,
    pub class: DayClass,
    pub season: Season,
}

impl ChurchDay {
    pub fn new(year: i32, month: u32, day: u32, class: DayClass, season: Season) -> Self {
        Self {
            date: NaiveDate::from_ymd_opt(year, month, day)
                .expect("Unable to instantiate new ChurchDay"),
            class,
            season,
        }
    }

    pub fn month_name(&self) -> String {
        self.date.format("%B").to_string()
    }

    pub fn string_day_of_week(&self) -> String {
        self.date.format("%A").to_string()
    }
}

pub struct ChurchYear {
    pub days: Vec<ChurchDay>,
}
