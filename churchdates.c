#include <stdbool.h>
#include <time.h>

#include "dates.h"

struct tm easter(int year) {
	int c = year / 100;
	int q = (((c-15) * 3) / 4) + 10;
	int l = 7 - (((year / 4) + year + 4 - q) % 7);
	int g = year % 19 + 1;
	int j = ((g*11)-10)%30;
	int s = q - 10;
	int m = ((c-14) * 8) / 25;

	int intermediate_e = j - s + m;
	do
	{
		intermediate_e += 30;
	}
	while (intermediate_e <= 0);

	int e = intermediate_e % 30;

	if((g > 11) && (e == 25)) {
		e = 26;
	} else if (e == 24) {
		e = 25;
	}

	int d;
	if (e < 24) {
		d = 44 - e;
	} else {
		d = 74 - 3;
	}

	int w = (d + 10 - l) % 7;

	int date = (d + 7) - w;
	int month = 3;
	if (date >= 32) {
		date = date - 31;
		month = 4;
	}


	struct tm tm_easter = {0};
	tm_easter.tm_mon = month-1;
	tm_easter.tm_mday = date;
	tm_easter.tm_year = year;
	tm_easter.tm_wday = fix_day_of_week(tm_easter);
	return tm_easter;
}

struct tm ash_wednesday_from_easter(struct tm easter) {
	//ash wednesday is always 46 days before easter.
	return subtract_days(easter, 46);
}

struct tm first_sunday_of_advent(int year) {
	//first sunday of advent is always the sunday before the first thursday of december
	//find december first, then based on what day of the week it is, find the sunday
	struct tm dec_first = {0};
	dec_first.tm_year = year;
	dec_first.tm_mon = 11;
	dec_first.tm_mday = 1;
	dec_first.tm_wday = fix_day_of_week(dec_first);	

	int days_to_subtract = 0;
	switch(dec_first.tm_wday) {
		case 0:
			days_to_subtract = 0;
			break;
		case 1:
			days_to_subtract = 1;
			break;
		case 2:
			days_to_subtract = 2;
			break;
		case 3:
			days_to_subtract = 3;
			break;
		case 4:
			days_to_subtract = 4;
			break;
		case 5:
			days_to_subtract = -2;
			break;
		case 6:
			days_to_subtract = -1;
			break;
		default:
			days_to_subtract = 0;
			break;
	}
	if(days_to_subtract > 0) {
		dec_first = subtract_days(dec_first, days_to_subtract);
	} else if (days_to_subtract < 0) {
		dec_first = add_days(dec_first, days_to_subtract);
	}
	return dec_first;
}
