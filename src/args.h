#ifndef ARGS_H
#define ARGS_H

#include <time.h>

typedef enum {
	MODE_TODAY,
	MODE_ALL,
	MODE_DATE,
	MODE_YEAR
} OutputMode;

typedef struct {
	OutputMode mode;
	struct tm target_date;
	int target_year;
	char feasts_csv_path[512];
} ProgramArgs;

// Parse command line arguments
// Returns 0 on success, -1 on error, 1 if help was shown
int parse_args(int argc, char *argv[], ProgramArgs *args);

// Print usage information
void print_usage(const char *program_name);

#endif // ARGS_H
