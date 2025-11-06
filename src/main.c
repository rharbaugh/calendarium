#include <stdio.h>
#include <string.h>
#include <time.h>
#include "church_day.h"
#include "builder.h"

static void print_usage(const char *program_name) {
	printf("Usage: %s [OPTIONS]\n", program_name);
	printf("Calculate the church calendar for the current year.\n\n");
	printf("Options:\n");
	printf("  --all     Print all days in the liturgical year\n");
	printf("  --help    Display this help message\n");
	printf("\nWith no options, prints only today's liturgical day.\n");
}

static int compare_dates(struct tm d1, struct tm d2) {
	if (d1.tm_year != d2.tm_year) return d1.tm_year - d2.tm_year;
	if (d1.tm_mon != d2.tm_mon) return d1.tm_mon - d2.tm_mon;
	return d1.tm_mday - d2.tm_mday;
}

int main(int argc, char *argv[]) {
	int print_all = 0;

	// Parse arguments
	for (int i = 1; i < argc; i++) {
		if (strcmp(argv[i], "--all") == 0) {
			print_all = 1;
		} else if (strcmp(argv[i], "--help") == 0) {
			print_usage(argv[0]);
			return 0;
		} else {
			fprintf(stderr, "Unknown option: %s\n", argv[i]);
			print_usage(argv[0]);
			return 1;
		}
	}

	time_t now = time(NULL);
	struct tm *today = localtime(&now);

	ChurchYear seasons = proper_of_seasons(*today);

	if (print_all) {
		// Print all days
		for (size_t i = 0; i < seasons.count; i++) {
			ChurchDay *day = &seasons.days[i];
			printf("%s, %s %d, %d - %s - %s in %s\n",
				church_day_weekday_string(day),
				church_day_month_name(day),
				day->date.tm_mday,
				day->date.tm_year + 1900,
				day->description,
				day_class_to_string(day->class),
				season_to_string(day->season));
		}
	} else {
		// Print only today
		ChurchDay *found = NULL;
		for (size_t i = 0; i < seasons.count; i++) {
			if (compare_dates(seasons.days[i].date, *today) == 0) {
				found = &seasons.days[i];
				break;
			}
		}

		if (found) {
			printf("%s, %s %d, %d - %s - %s in %s\n",
				church_day_weekday_string(found),
				church_day_month_name(found),
				found->date.tm_mday,
				found->date.tm_year + 1900,
				found->description,
				day_class_to_string(found->class),
				season_to_string(found->season));
		} else {
			fprintf(stderr, "Could not find today's date in the liturgical calendar.\n");
			church_year_free(&seasons);
			return 1;
		}
	}

	church_year_free(&seasons);
	return 0;
}
