#include <stdio.h>
#include <time.h>
#include "church_day.h"
#include "builder.h"

int main(void) {
	time_t now = time(NULL);
	struct tm *today = localtime(&now);

	ChurchYear seasons = proper_of_seasons(*today);

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

	church_year_free(&seasons);
	return 0;
}
