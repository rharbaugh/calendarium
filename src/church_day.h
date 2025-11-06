#ifndef CHURCH_DAY_H
#define CHURCH_DAY_H

#include <time.h>

typedef enum {
	SOLEMNITY,
	SUNDAY,
	FEAST,
	MEMORIAL,
	SEASONAL_WEEKDAY,
	FERIAL_WEEKDAY
} DayClass;

typedef enum {
	ADVENT,
	CHRISTMAS,
	ORDINARY_TIME,
	LENT,
	TRIDUUM,
	EASTER
} Season;

typedef struct {
	struct tm date;
	DayClass class;
	Season season;
	char description[256];
} ChurchDay;

typedef struct {
	ChurchDay *days;
	size_t count;
	size_t capacity;
} ChurchYear;

//Create a new ChurchDay
ChurchDay church_day_new(int year, int month, int day, DayClass class, Season season, const char *description);

//Get month name from ChurchDay
const char* church_day_month_name(const ChurchDay *day);

//Get day of week string from ChurchDay
const char* church_day_weekday_string(const ChurchDay *day);

//Get string representation of DayClass
const char* day_class_to_string(DayClass class);

//Get string representation of Season
const char* season_to_string(Season season);

//ChurchYear functions
ChurchYear church_year_new(void);
void church_year_add(ChurchYear *year, ChurchDay day);
void church_year_free(ChurchYear *year);

#endif // CHURCH_DAY_H
