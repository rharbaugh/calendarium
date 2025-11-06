#include "church_day.h"
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

ChurchDay church_day_new(int year, int month, int day, DayClass class, Season season, const char *description) {
	ChurchDay cd;
	memset(&cd.date, 0, sizeof(struct tm));

	cd.date.tm_year = year - 1900;
	cd.date.tm_mon = month - 1;
	cd.date.tm_mday = day;

	mktime(&cd.date);

	cd.class = class;
	cd.season = season;
	strncpy(cd.description, description, sizeof(cd.description) - 1);
	cd.description[sizeof(cd.description) - 1] = '\0';

	return cd;
}

const char* church_day_month_name(const ChurchDay *day) {
	static char buffer[32];
	strftime(buffer, sizeof(buffer), "%B", &day->date);
	return buffer;
}

const char* church_day_weekday_string(const ChurchDay *day) {
	static char buffer[32];
	strftime(buffer, sizeof(buffer), "%A", &day->date);
	return buffer;
}

const char* day_class_to_string(DayClass class) {
	switch (class) {
		case SOLEMNITY: return "Solemnity";
		case SUNDAY: return "Sunday";
		case FEAST: return "Feast";
		case MEMORIAL: return "Memorial";
		case SEASONAL_WEEKDAY: return "Seasonal Weekday";
		case FERIAL_WEEKDAY: return "Ferial";
		default: return "Unknown";
	}
}

const char* season_to_string(Season season) {
	switch(season) {
		case ADVENT: return "Advent";
		case CHRISTMAS: return "Christmas";
		case ORDINARY_TIME: return "Ordinary Time";
		case LENT: return "Lent";
		case TRIDUUM: return "Sacred Triduum";
		case EASTER: return "Easter";
		default: return "Unknown";
	}
}

ChurchYear church_year_new(void) {
	ChurchYear cy;
	cy.capacity = 366;
	cy.count = 0;
	cy.days = malloc(cy.capacity * sizeof(ChurchDay));
	return cy;
}

void church_year_add(ChurchYear *year, ChurchDay day) {
	if (year->count >= year->capacity) {
		year->capacity *= 2;
		year->days = realloc(year->days, year->capacity * sizeof(ChurchDay));
	}
	year->days[year->count++] = day;
}

void church_year_free(ChurchYear *year) {
	free(year->days);
	year->days = NULL;
	year->count = 0;
	year->capacity = 0;
}
