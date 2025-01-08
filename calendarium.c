#include <stdio.h>
#include <stdbool.h>
#include <time.h>

#include "dates.h"
#include "churchdates.h"



int main() {
	//get today's date
	time_t t = time(NULL);
	struct tm tm = *localtime(&t);

	struct tm tm_easter = easter(tm.tm_year + 1900);
	print_tm("Easter", tm_easter);
	struct tm tm_ash_wednesday = ash_wednesday_from_easter(tm_easter);
	print_tm("Ash Wednesday", tm_ash_wednesday);
	struct tm tm_advent = first_sunday_of_advent(2025);
	print_tm("Advent", tm_advent);
}
