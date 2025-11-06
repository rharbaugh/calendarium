#include "csv_loader.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <ctype.h>

// Helper function to trim whitespace from a string
static void trim(char *str) {
	char *start = str;
	char *end;

	// Trim leading space
	while(isspace((unsigned char)*start)) start++;

	// All spaces?
	if(*start == 0) {
		*str = 0;
		return;
	}

	// Trim trailing space
	end = start + strlen(start) - 1;
	while(end > start && isspace((unsigned char)*end)) end--;

	// Write new null terminator
	end[1] = '\0';

	// Move string if needed
	if(start != str) {
		memmove(str, start, strlen(start) + 1);
	}
}

// Parse DayClass from string
static int parse_day_class(const char *str, DayClass *class) {
	if (strcasecmp(str, "SOLEMNITY") == 0) {
		*class = SOLEMNITY;
	} else if (strcasecmp(str, "SUNDAY") == 0) {
		*class = SUNDAY;
	} else if (strcasecmp(str, "FEAST") == 0) {
		*class = FEAST;
	} else if (strcasecmp(str, "MEMORIAL") == 0) {
		*class = MEMORIAL;
	} else if (strcasecmp(str, "SEASONAL_WEEKDAY") == 0 || strcasecmp(str, "SEASONAL WEEKDAY") == 0) {
		*class = SEASONAL_WEEKDAY;
	} else if (strcasecmp(str, "FERIAL_WEEKDAY") == 0 || strcasecmp(str, "FERIAL WEEKDAY") == 0 || strcasecmp(str, "FERIAL") == 0) {
		*class = FERIAL_WEEKDAY;
	} else {
		return -1;
	}
	return 0;
}

// Parse Season from string
static int parse_season(const char *str, Season *season) {
	if (strcasecmp(str, "ADVENT") == 0) {
		*season = ADVENT;
	} else if (strcasecmp(str, "CHRISTMAS") == 0) {
		*season = CHRISTMAS;
	} else if (strcasecmp(str, "ORDINARY_TIME") == 0 || strcasecmp(str, "ORDINARY TIME") == 0) {
		*season = ORDINARY_TIME;
	} else if (strcasecmp(str, "LENT") == 0) {
		*season = LENT;
	} else if (strcasecmp(str, "TRIDUUM") == 0) {
		*season = TRIDUUM;
	} else if (strcasecmp(str, "EASTER") == 0) {
		*season = EASTER;
	} else {
		return -1;
	}
	return 0;
}

// Parse Subject from string
static int parse_subject(const char *str, Subject *subject) {
	if (strcasecmp(str, "LORD") == 0) {
		*subject = LORD;
	} else if (strcasecmp(str, "BVM") == 0) {
		*subject = BVM;
	} else if (strcasecmp(str, "NONE") == 0) {
		*subject = NONE;
	} else {
		return -1;
	}
	return 0;
}

// Load feast days from CSV file
int load_feasts_from_csv(ChurchYear *year, const char *filepath) {
	FILE *file = fopen(filepath, "r");
	if (file == NULL) {
		return -1;  // File not found
	}

	char line[512];
	int line_number = 0;
	int first_day_year = year->days[0].date.tm_year + 1900;
	int first_day_month = year->days[0].date.tm_mon;  // 0-11

	while (fgets(line, sizeof(line), file)) {
		line_number++;

		// Skip empty lines and comments
		trim(line);
		if (line[0] == '\0' || line[0] == '#') {
			continue;
		}

		// Parse CSV line: month,day,class,season,subject,description
		char *fields[6];
		int field_count = 0;
		char *token = strtok(line, ",");

		while (token != NULL && field_count < 6) {
			fields[field_count] = token;
			trim(fields[field_count]);
			field_count++;
			token = strtok(NULL, ",");
		}

		if (field_count != 6) {
			fprintf(stderr, "CSV parse error on line %d: expected 6 fields, got %d\n", line_number, field_count);
			fclose(file);
			return -2;
		}

		// Parse month and day
		int month = atoi(fields[0]);
		int day = atoi(fields[1]);

		if (month < 1 || month > 12 || day < 1 || day > 31) {
			fprintf(stderr, "CSV parse error on line %d: invalid date %d-%d\n", line_number, month, day);
			fclose(file);
			return -2;
		}

		// Determine the correct year for this feast
		// Liturgical year starts in late November/December (Advent)
		// So if the first day is in Nov/Dec and the feast is in Jan-Nov, use next year
		int feast_year = first_day_year;
		if (first_day_month >= 10 && month - 1 < first_day_month) {
			// First day is in Nov/Dec, and this month is before that, so use next year
			feast_year++;
		}

		// Parse DayClass
		DayClass class;
		if (parse_day_class(fields[2], &class) != 0) {
			fprintf(stderr, "CSV parse error on line %d: invalid class '%s'\n", line_number, fields[2]);
			fclose(file);
			return -2;
		}

		// Parse Season
		Season season;
		if (parse_season(fields[3], &season) != 0) {
			fprintf(stderr, "CSV parse error on line %d: invalid season '%s'\n", line_number, fields[3]);
			fclose(file);
			return -2;
		}

		// Parse Subject
		Subject subject;
		if (parse_subject(fields[4], &subject) != 0) {
			fprintf(stderr, "CSV parse error on line %d: invalid subject '%s'\n", line_number, fields[4]);
			fclose(file);
			return -2;
		}

		// Create the ChurchDay
		ChurchDay feast = church_day_new(feast_year, month, day, class, season, fields[5]);
		feast.subject = subject;  // Override the default NONE

		// Add to ChurchYear
		church_year_add(year, feast);
	}

	fclose(file);
	return 0;
}
