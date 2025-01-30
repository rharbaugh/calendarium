#include <stdbool.h>
#include <stdio.h>
#include <time.h>

bool is_leap_year(int year) {
  bool by_4 = year % 4 == 0;
  bool by_100 = year % 100 == 0;
  bool by_400 = year % 400 == 0;
  if (by_4 && !by_100) {
    return true;
  } else if (by_100 && by_400) {
    return true;
  } else {
    return false;
  }
}
int days_in_month(int month, int year) {
  switch (month) {
  case 0:
    return 31;
    break;
  case 1:
    if (is_leap_year(year)) {
      return 29;
    } else {
      return 28;
    }
    break;
  case 2:
    return 31;
    break;
  case 3:
    return 30;
    break;
  case 4:
    return 31;
    break;
  case 5:
    return 30;
    break;
  case 6:
    return 31;
    break;
  case 7:
    return 31;
    break;
  case 8:
    return 30;
    break;
  case 9:
    return 31;
    break;
  case 10:
    return 30;
    break;
  case 11:
    return 31;
    break;
  default:
    return 0;
    break;
  }
}
struct tm subtract_days(struct tm date, int amount) {
  int days_subtracted = 0;
  while (days_subtracted < amount) {
    if (date.tm_mday > 1) {
      date.tm_mday--;
    }
    // there are two month rollover situations - one where we go back a month,
    // one where we go back a year
    else if (date.tm_mday == 1) {
      if (date.tm_mon == 0) {
        date.tm_year--;
        date.tm_mday = 31;
        date.tm_mon = 11;
      } else {
        date.tm_mon--;
        date.tm_mday = days_in_month(date.tm_mon, date.tm_year);
      }
    }
    days_subtracted++;
  }
  return date;
}

struct tm add_days(struct tm date, int amount) {
  int days_added = 0;
  while (days_added < amount) {
    int current_month_length = days_in_month(date.tm_mon, date.tm_year);
    if (date.tm_mday < current_month_length) {
      date.tm_mday++;
    }
    // there are two month rollover situations - one where we go forward a
    // month, one where we go forward a year
    if (date.tm_mday == current_month_length) {
      if (date.tm_mon == 11) {
        date.tm_year++;
        date.tm_mday = 1;
        date.tm_mon = 0;
      } else {
        date.tm_mon++;
        date.tm_mday = 1;
      }
    }
    days_added++;
  }
  return date;
}
int fix_day_of_week(struct tm date) {
  //(Year Code + Month Code + Century Code + Date Number - Leap Year Code) mod 7
  int last_two = (date.tm_year % 100);
  int year_code = (last_two + (last_two / 4)) % 7;

  int month_code = 0;
  // plus one the month code to make it more human readable. these are
  // zero-based
  switch (date.tm_mon + 1) {
  case 2:
    month_code = 3;
    break;
  case 3:
    month_code = 3;
    break;
  case 4:
    month_code = 6;
    break;
  case 5:
    month_code = 1;
    break;
  case 6:
    month_code = 4;
    break;
  case 7:
    month_code = 6;
    break;
  case 8:
    month_code = 2;
    break;
  case 9:
    month_code = 5;
    break;
  case 11:
    month_code = 3;
    break;
  case 12:
    month_code = 5;
    break;
  default:
    month_code = 0;
    break;
  }
  int century_code = 6;

  // this is a fancy way to make this work for all positive years
  int counter = 100;
  while (counter < date.tm_year) {
    counter = counter + 100;
    if (century_code == 6) {
      century_code = 4;
    } else if (century_code == 4) {
      century_code = 2;
    } else if (century_code == 2) {
      century_code = 0;
    } else if (century_code == 0) {
      century_code = 6;
    }
  }

  int leap_year_code = 0;
  bool leap = is_leap_year(date.tm_year);
  if (leap && (date.tm_mon == 0 || date.tm_mon == 1)) {
    leap_year_code = 1;
  }

  int day_code =
      (year_code + month_code + century_code + date.tm_mday - leap_year_code) %
      7;
  return day_code;
}
void print_tm(char *name, struct tm in) {
  printf("%s is %d/%d/%d this year.\n", name, in.tm_mon + 1, in.tm_mday,
         in.tm_year);
}

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
int main() {
  // get today's date
  time_t t = time(NULL);
  struct tm tm = *localtime(&t);

  struct tm tm_easter = easter(tm.tm_year + 1900);
  print_tm("Easter", tm_easter);
  struct tm tm_ash_wednesday = ash_wednesday_from_easter(tm_easter);
  print_tm("Ash Wednesday", tm_ash_wednesday);
  struct tm tm_advent = first_sunday_of_advent(2025);
  print_tm("Advent", tm_advent);
}
