#include "builder.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

//helper to add days to a date
static struct tm add_days(struct tm date, int days) {
	date.tm_mday += days;
	mktime(&date);
	return date;
}

//helper to compare dates (return negative if d1 < d2, 0 if equal, positive if d1 > d2)
static int compare_dates(struct tm d1, struct tm d2) {
	if (d1.tm_year != d2.tm_year) return d1.tm_year - d2.tm_year;
	if (d1.tm_mon != d2.tm_mon) return d1.tm_mon - d2.tm_mon;
	return d1.tm_mday - d2.tm_mday;
}

/// Computus algorithm to calculate Easter Sunday
static ChurchDay easter_for_year(int year) {
    int a = year % 19;
    int b = year / 100;
    int c = year % 100;
    int d = b / 4;
    int e = b % 4;
    int f = (b + 8) / 25;
    int g = (b - f + 1) / 3;
    int h = (19 * a + b - d - g + 15) % 30;
    int i = c / 4;
    int k = c % 4;
    int l = (32 + 2 * e + 2 * i - h - k) % 7;
    int m = (a + 11 * h + 22 * l) / 451;
    int month = (h + l - 7 * m + 114) / 31;
    int day = (h + l - 7 * m + 114) % 31 + 1;
    
    return church_day_new(year, month, day, SOLEMNITY, EASTER,
                         "Easter Sunday of the Resurrection of the Lord");
}

static ChurchDay ash_wednesday(const ChurchDay *easter) {
    struct tm date = add_days(easter->date, -46);
    ChurchDay cd;
    cd.date = date;
    cd.class = SOLEMNITY;
    cd.season = LENT;
    strcpy(cd.description, "Ash Wednesday");
    return cd;
}

static ChurchDay first_sunday_advent(int year) {
    struct tm dec_first = {0};
    dec_first.tm_year = year - 1900;
    dec_first.tm_mon = 11; // December
    dec_first.tm_mday = 1;
    mktime(&dec_first);
    
    int days_to_subtract;
    switch (dec_first.tm_wday) {
        case 0: days_to_subtract = 0; break;  // Sunday
        case 1: days_to_subtract = 1; break;  // Monday
        case 2: days_to_subtract = 2; break;  // Tuesday
        case 3: days_to_subtract = 3; break;  // Wednesday
        case 4: days_to_subtract = 4; break;  // Thursday
        case 5: days_to_subtract = -2; break; // Friday
        case 6: days_to_subtract = -1; break; // Saturday
        default: days_to_subtract = 0; break;
    }
    
    struct tm date = add_days(dec_first, -days_to_subtract);
    ChurchDay cd;
    cd.date = date;
    cd.class = SUNDAY;
    cd.season = ADVENT;
    strcpy(cd.description, "1st Sunday of Advent");
    return cd;
}

static ChurchDay epiphany(int year) {
    for (int day = 2; day <= 8; day++) {
        struct tm date = {0};
        date.tm_year = year - 1900;
        date.tm_mon = 0; // January
        date.tm_mday = day;
        mktime(&date);
        
        if (date.tm_wday == 0) { // Sunday
            return church_day_new(year, 1, day, SOLEMNITY, CHRISTMAS,
                                "The Epiphany of the Lord");
        }
    }
    // Fallback (shouldn't happen)
    return church_day_new(year, 1, 6, SOLEMNITY, CHRISTMAS, "The Epiphany of the Lord");
}

static ChurchDay baptism(ChurchDay epiphany_day) {
    struct tm date = add_days(epiphany_day.date, 7);
    ChurchDay cd;
    cd.date = date;
    cd.class = FEAST;
    cd.season = CHRISTMAS;
    strcpy(cd.description, "The Baptism of the Lord");
    return cd;
}

static ChurchDay pentecost(ChurchDay easter) {
    struct tm date = add_days(easter.date, 49);
    ChurchDay cd;
    cd.date = date;
    cd.class = SOLEMNITY;
    cd.season = EASTER;
    strcpy(cd.description, "Pentecost Sunday");
    return cd;
}

static const char* ordinal_suffix(int number) {
    char str[16];
    snprintf(str, sizeof(str), "%d", number);
    
    if (strstr(str, "11") == str + strlen(str) - 2 ||
        strstr(str, "12") == str + strlen(str) - 2 ||
        strstr(str, "13") == str + strlen(str) - 2) {
        return "th";
    }
    
    switch (str[strlen(str) - 1]) {
        case '1': return "st";
        case '2': return "nd";
        case '3': return "rd";
        default: return "th";
    }
}

static const char* day_string(int wday) {
    switch (wday) {
        case 0: return "Sunday";
        case 1: return "Monday";
        case 2: return "Tuesday";
        case 3: return "Wednesday";
        case 4: return "Thursday";
        case 5: return "Friday";
        case 6: return "Saturday";
        default: return "Unknown";
    }
}

static void calculate_advent(ChurchYear *cy, const ChurchDay *start) {
    ChurchDay current = *start;
    int sunday_counter = 1;
    
    church_year_add(cy, current);
    
    while (1) {
        if (current.date.tm_mon == 11 && current.date.tm_mday == 24) {
            break;
        }
        
        current.date = add_days(current.date, 1);
        
        if (current.date.tm_wday == 0) {
            sunday_counter++;
        }
        
        current.season = ADVENT;
        current.class = (current.date.tm_wday == 0) ? SUNDAY : SEASONAL_WEEKDAY;
        
        if (current.date.tm_mday >= 17 && current.date.tm_mon == 11) {
            snprintf(current.description, sizeof(current.description),
                    "December %d%s", current.date.tm_mday,
                    ordinal_suffix(current.date.tm_mday));
        } else if (current.date.tm_wday == 0) {
            snprintf(current.description, sizeof(current.description),
                    "%d%s Sunday of Advent", sunday_counter,
                    ordinal_suffix(sunday_counter));
        } else {
            snprintf(current.description, sizeof(current.description),
                    "%s of the %d%s week of Advent",
                    day_string(current.date.tm_wday), sunday_counter,
                    ordinal_suffix(sunday_counter));
        }
        
        church_year_add(cy, current);
    }
}

static void calculate_christmas(ChurchYear *cy, int year) {
    ChurchDay epiphany_day = epiphany(year + 1);
    ChurchDay baptism_day = baptism(epiphany_day);
    
    ChurchDay christmas = church_day_new(year, 12, 25, SOLEMNITY, CHRISTMAS,
                                        "The Nativity of the Lord (Christmas)");
    church_year_add(cy, christmas);
    
    ChurchDay current = christmas;
    while (1) {
        struct tm next_date = add_days(current.date, 1);
        if (next_date.tm_mon == epiphany_day.date.tm_mon &&
            next_date.tm_mday == epiphany_day.date.tm_mday) {
            break;
        }
        
        current.date = next_date;
        current.season = CHRISTMAS;
        current.class = (current.date.tm_wday == 0) ? FEAST : SEASONAL_WEEKDAY;
        
        if (current.date.tm_wday == 0) {
            strcpy(current.description, "The Holy Family of Jesus, Mary, and Joseph");
        } else {
            switch (current.date.tm_mday) {
                case 26: strcpy(current.description, "Saint Stephen, First Martyr"); break;
                case 27: strcpy(current.description, "Saint John, Apostle and Evangelist"); break;
                case 28: strcpy(current.description, "Holy Innocents"); break;
                case 29: strcpy(current.description, "Fifth Day within the Octave of the Nativity of the Lord"); break;
                case 30: strcpy(current.description, "Sixth Day within the Octave of the Nativity of the Lord"); break;
                case 31: strcpy(current.description, "Seventh Day within the Octave of the Nativity of the Lord"); break;
                default: strcpy(current.description, "Christmas Weekday"); break;
            }
        }
        
        church_year_add(cy, current);
    }
    
    church_year_add(cy, epiphany_day);
    
    current = epiphany_day;
    while (1) {
        struct tm next_date = add_days(current.date, 1);
        if (next_date.tm_mon == baptism_day.date.tm_mon &&
            next_date.tm_mday == baptism_day.date.tm_mday) {
            break;
        }
        
        current.date = next_date;
        current.season = CHRISTMAS;
        current.class = SEASONAL_WEEKDAY;
        snprintf(current.description, sizeof(current.description),
                "%s between Epiphany and Baptism of the Lord",
                day_string(current.date.tm_wday));
        
        church_year_add(cy, current);
    }
    
    church_year_add(cy, baptism_day);
}

static void calculate_lent(ChurchYear *cy, ChurchDay ash_wed) {
    ChurchDay easter = easter_for_year(ash_wed.date.tm_year + 1900);
    struct tm stop_date = add_days(easter.date, -8);
    
    ChurchDay current = ash_wed;
    int sunday_counter = 0;
    
    church_year_add(cy, current);
    
    while (compare_dates(current.date, stop_date) != 0) {
        current.date = add_days(current.date, 1);
        
        if (current.date.tm_wday == 0) {
            sunday_counter++;
        }
        
        current.season = LENT;
        current.class = (current.date.tm_wday == 0) ? SUNDAY : SEASONAL_WEEKDAY;
        
        if (sunday_counter == 0) {
            snprintf(current.description, sizeof(current.description),
                    "%s after Ash Wednesday", day_string(current.date.tm_wday));
        } else if (current.date.tm_wday == 0) {
            snprintf(current.description, sizeof(current.description),
                    "%d%s Sunday of Lent", sunday_counter,
                    ordinal_suffix(sunday_counter));
        } else {
            snprintf(current.description, sizeof(current.description),
                    "%s of the %d%s week of Lent",
                    day_string(current.date.tm_wday), sunday_counter,
                    ordinal_suffix(sunday_counter));
        }
        
        church_year_add(cy, current);
    }
}

static void calculate_easter(ChurchYear *cy, ChurchDay sat_before) {
    ChurchDay easter = easter_for_year(sat_before.date.tm_year + 1900);
    struct tm ps_date = add_days(easter.date, -7);
    
    ChurchDay palm_sunday;
    palm_sunday.date = ps_date;
    palm_sunday.class = SUNDAY;
    palm_sunday.season = LENT;
    strcpy(palm_sunday.description, "Palm Sunday of the Passion of the Lord");
    church_year_add(cy, palm_sunday);
    
    const char* holy_week_names[] = {
        "",
        "Monday of Holy Week",
        "Tuesday of Holy Week",
        "Wednesday of Holy Week",
        "Holy Thursday",
        "Friday of the Passion of the Lord",
        "Holy Saturday"
    };
    
    for (int days_after = 1; days_after <= 6; days_after++) {
        ChurchDay day;
        day.date = add_days(ps_date, days_after);
        day.class = (days_after < 4) ? SEASONAL_WEEKDAY : SOLEMNITY;
        day.season = (days_after < 4) ? LENT : TRIDUUM;
        strcpy(day.description, holy_week_names[days_after]);
        church_year_add(cy, day);
    }
    
    church_year_add(cy, easter);
    
    const char* octave_names[] = {
        "",
        "Monday within the Octave of Easter",
        "Tuesday within the Octave of Easter",
        "Wednesday within the Octave of Easter",
        "Thursday within the Octave of Easter",
        "Friday within the Octave of Easter",
        "Saturday within the Octave of Easter",
        "Sunday of Divine Mercy"
    };
    
    for (int octave_days = 1; octave_days <= 7; octave_days++) {
        ChurchDay day;
        day.date = add_days(easter.date, octave_days);
        day.class = SOLEMNITY;
        day.season = EASTER;
        strcpy(day.description, octave_names[octave_days]);
        church_year_add(cy, day);
    }
    
    ChurchDay pentecost_day = pentecost(easter);
    struct tm stop_date = add_days(pentecost_day.date, -1);
    
    ChurchDay current = cy->days[cy->count - 1];
    int sunday_counter = 2;
    
    while (compare_dates(current.date, stop_date) != 0) {
        current.date = add_days(current.date, 1);
        
        if (current.date.tm_wday == 0) {
            sunday_counter++;
        }
        
        current.season = EASTER;
        current.class = (current.date.tm_wday == 0) ? SUNDAY : SEASONAL_WEEKDAY;
        
        if (current.date.tm_wday == 0) {
            if (sunday_counter == 7) {
                strcpy(current.description, "The Ascension of the Lord");
            } else {
                snprintf(current.description, sizeof(current.description),
                        "%d%s Sunday of Easter", sunday_counter,
                        ordinal_suffix(sunday_counter));
            }
        } else {
            snprintf(current.description, sizeof(current.description),
                    "%s of the %d%s week of Easter",
                    day_string(current.date.tm_wday), sunday_counter,
                    ordinal_suffix(sunday_counter));
        }
        
        church_year_add(cy, current);
    }
    
    church_year_add(cy, pentecost_day);
}

static void calculate_otime_prelent(ChurchYear *cy, ChurchDay baptism_day) {
    ChurchDay easter = easter_for_year(baptism_day.date.tm_year + 1900);
    ChurchDay ash_wed = ash_wednesday(&easter);
    struct tm stop_date = add_days(ash_wed.date, -1);
    
    ChurchDay current;
    current.date = add_days(baptism_day.date, 1);
    current.class = FERIAL_WEEKDAY;
    current.season = ORDINARY_TIME;
    strcpy(current.description, "Monday of the 1st week in Ordinary Time");
    
    int sunday_counter = 1;
    church_year_add(cy, current);
    
    while (compare_dates(current.date, stop_date) != 0) {
        current.date = add_days(current.date, 1);
        
        if (current.date.tm_wday == 0) {
            sunday_counter++;
        }
        
        current.season = ORDINARY_TIME;
        current.class = (current.date.tm_wday == 0) ? SUNDAY : FERIAL_WEEKDAY;
        
        if (current.date.tm_wday == 0) {
            snprintf(current.description, sizeof(current.description),
                    "%d%s Sunday of Ordinary Time", sunday_counter,
                    ordinal_suffix(sunday_counter));
        } else {
            snprintf(current.description, sizeof(current.description),
                    "%s of the %d%s week of Ordinary Time",
                    day_string(current.date.tm_wday), sunday_counter,
                    ordinal_suffix(sunday_counter));
        }
        
        church_year_add(cy, current);
    }
}

static void calculate_otime_posteaster(ChurchYear *cy, ChurchDay pentecost_day) {
    ChurchDay first_advent = first_sunday_advent(pentecost_day.date.tm_year + 1900);
    struct tm last_day = add_days(first_advent.date, -1);
    
    struct tm trinity_sunday = add_days(pentecost_day.date, 7);
    struct tm corpus_christi = add_days(trinity_sunday, 7);
    
    // Build backwards
    ChurchDay *temp_days = malloc(366 * sizeof(ChurchDay));
    size_t temp_count = 0;
    
    ChurchDay current;
    current.date = last_day;
    current.class = FERIAL_WEEKDAY;
    current.season = ORDINARY_TIME;
    int sunday_counter = 34;
    
    snprintf(current.description, sizeof(current.description),
            "%s of the %d%s Week in Ordinary Time",
            day_string(current.date.tm_wday), sunday_counter,
            ordinal_suffix(sunday_counter));
    temp_days[temp_count++] = current;
    
    struct tm stop_date = add_days(pentecost_day.date, 1);
    
    while (compare_dates(current.date, stop_date) != 0) {
        current.date = add_days(current.date, -1);
        
        if (current.date.tm_wday == 0) {
            sunday_counter--;
        }
        
        current.season = ORDINARY_TIME;
        
        if (current.date.tm_wday == 0) {
            if (compare_dates(current.date, trinity_sunday) == 0) {
                current.class = SOLEMNITY;
                strcpy(current.description, "The Most Holy Trinity");
            } else if (compare_dates(current.date, corpus_christi) == 0) {
                current.class = SOLEMNITY;
                strcpy(current.description, "The Most Holy Body and Blood of Christ");
            } else if (sunday_counter == 33) {
                current.class = SOLEMNITY;
                strcpy(current.description, "Our Lord Jesus Christ, King of the Universe");
            } else {
                current.class = SUNDAY;
                snprintf(current.description, sizeof(current.description),
                        "%d%s Sunday of Ordinary Time", sunday_counter,
                        ordinal_suffix(sunday_counter));
            }
        } else {
            current.class = FERIAL_WEEKDAY;
            snprintf(current.description, sizeof(current.description),
                    "%s of the %d%s week of Ordinary Time",
                    day_string(current.date.tm_wday), sunday_counter,
                    ordinal_suffix(sunday_counter));
        }
        
        temp_days[temp_count++] = current;
    }
    
    // Reverse and add to main list
    for (size_t i = temp_count; i > 0; i--) {
        church_year_add(cy, temp_days[i - 1]);
    }
    
    free(temp_days);
}

ChurchYear proper_of_seasons(struct tm today) {
    ChurchYear cy = church_year_new();
    
    ChurchDay first_advent = first_sunday_advent(today.tm_year + 1900);
    if (compare_dates(today, first_advent.date) < 0) {
        first_advent = first_sunday_advent(today.tm_year + 1900 - 1);
    }
    
    calculate_advent(&cy, &first_advent);
    calculate_christmas(&cy, first_advent.date.tm_year + 1900);
    
    ChurchDay baptism_day = cy.days[cy.count - 1];
    ChurchDay easter = easter_for_year(baptism_day.date.tm_year + 1900);
    ChurchDay ash_wed = ash_wednesday(&easter);
    
    calculate_otime_prelent(&cy, baptism_day);
    calculate_lent(&cy, ash_wed);
    
    ChurchDay sat_before = cy.days[cy.count - 1];
    calculate_easter(&cy, sat_before);
    
    ChurchDay pentecost_day = cy.days[cy.count - 1];
    calculate_otime_posteaster(&cy, pentecost_day);
    
    return cy;
}
