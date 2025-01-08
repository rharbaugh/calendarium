#ifndef DATES_H_INCLUDED
#define DATES_H_INCLUDED

bool is_leap_year(int year);
int fix_day_of_week(struct tm date);
int days_in_month(int month, int year);
struct tm subtract_days(struct tm date, int amount);
struct tm add_days(struct tm date, int amount);
void print_tm(char *name, struct tm in);

#endif
