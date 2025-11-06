#include "args.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

void print_usage(const char *program_name) {
	printf("Usage: %s [OPTIONS]\n", program_name);
	printf("Calculate the church calendar for the current year.\n\n");
	printf("Options:\n");
	printf("  --all              Print all days in the current liturgical year\n");
	printf("  --date MM-DD-YYYY  Print the liturgical day for the specified date\n");
	printf("  --year YYYY        Print all days in the specified year\n");
	printf("  --help             Display this help message\n");
	printf("\nWith no options, prints only today's liturgical day.\n");
}

static int parse_date(const char *date_str, struct tm *date) {
	int month, day, year;
	
	if (sscanf(date_str, "%d-%d-%d", &month, &day, &year) != 3) {
		return -1;
	}
	
	// Validate ranges
	if (month < 1 || month > 12 || day < 1 || day > 31 || year < 1900 || year > 9999) {
		return -1;
	}
	
	memset(date, 0, sizeof(struct tm));
	date->tm_mon = month - 1;
	date->tm_mday = day;
	date->tm_year = year - 1900;
	
	// Normalize the date
	if (mktime(date) == -1) {
		return -1;
	}
	
	return 0;
}

int parse_args(int argc, char *argv[], ProgramArgs *args) {
	// Initialize with defaults
	args->mode = MODE_TODAY;
	time_t now = time(NULL);
	struct tm *today = localtime(&now);
	args->target_date = *today;
	args->target_year = today->tm_year + 1900;
	
	for (int i = 1; i < argc; i++) {
		if (strcmp(argv[i], "--help") == 0) {
			print_usage(argv[0]);
			return 1;
		} else if (strcmp(argv[i], "--all") == 0) {
			args->mode = MODE_ALL;
		} else if (strcmp(argv[i], "--date") == 0) {
			if (i + 1 >= argc) {
				fprintf(stderr, "Error: --date requires a date argument (MM-DD-YYYY)\n");
				return -1;
			}
			
			if (parse_date(argv[i + 1], &args->target_date) != 0) {
				fprintf(stderr, "Error: Invalid date format '%s'. Use MM-DD-YYYY\n", argv[i + 1]);
				return -1;
			}
			
			args->mode = MODE_DATE;
			args->target_year = args->target_date.tm_year + 1900;
			i++; // Skip the date argument
		} else if (strcmp(argv[i], "--year") == 0) {
			if (i + 1 >= argc) {
				fprintf(stderr, "Error: --year requires a year argument\n");
				return -1;
			}
			
			char *endptr;
			long year = strtol(argv[i + 1], &endptr, 10);
			
			if (*endptr != '\0' || year < 1900 || year > 9999) {
				fprintf(stderr, "Error: Invalid year '%s'\n", argv[i + 1]);
				return -1;
			}
			
			args->mode = MODE_YEAR;
			args->target_year = (int)year;
			
			// Set target_date to January 1 of that year for calendar generation
			memset(&args->target_date, 0, sizeof(struct tm));
			args->target_date.tm_year = (int)(year - 1900);
			args->target_date.tm_mon = 0;
			args->target_date.tm_mday = 1;
			mktime(&args->target_date);
			
			i++; // Skip the year argument
		} else {
			fprintf(stderr, "Unknown option: %s\n", argv[i]);
			print_usage(argv[0]);
			return -1;
		}
	}
	
	return 0;
}
