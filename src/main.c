#include <stdio.h>
#include <time.h>
#include "church_day.h"
#include "builder.h"
#include "args.h"

static int compare_dates(struct tm d1, struct tm d2) {
	if (d1.tm_year != d2.tm_year) return d1.tm_year - d2.tm_year;
	if (d1.tm_mon != d2.tm_mon) return d1.tm_mon - d2.tm_mon;
	return d1.tm_mday - d2.tm_mday;
}

static void print_day(const ChurchDay *day) {
	printf("%s, %s %d, %d - %s - %s in %s\n",
		church_day_weekday_string(day),
		church_day_month_name(day),
		day->date.tm_mday,
		day->date.tm_year + 1900,
		day->description,
		day_class_to_string(day->class),
		season_to_string(day->season));
}

int main(int argc, char *argv[]) {
	ProgramArgs args;
	int parse_result = parse_args(argc, argv, &args);
	
	if (parse_result == 1) {
		// Help was shown
		return 0;
	} else if (parse_result == -1) {
		// Error occurred
		return 1;
	}

	ChurchYear seasons = proper_of_seasons(args.target_date);

	switch (args.mode) {
		case MODE_ALL:
		case MODE_YEAR:
			// Print all days
			for (size_t i = 0; i < seasons.count; i++) {
				print_day(&seasons.days[i]);
			}
			break;

		case MODE_TODAY:
		case MODE_DATE: {
			// Print only the specified day
			ChurchDay *found = NULL;
			for (size_t i = 0; i < seasons.count; i++) {
				if (compare_dates(seasons.days[i].date, args.target_date) == 0) {
					found = &seasons.days[i];
					break;
				}
			}

			if (found) {
				print_day(found);
			} else {
				fprintf(stderr, "Could not find the specified date in the liturgical calendar.\n");
				church_year_free(&seasons);
				return 1;
			}
			break;
		}
	}

	church_year_free(&seasons);
	return 0;
}
