#ifndef CHURCHDATES_H_INCLUDED
#define CHURCHDATES_H_INCLUDED

struct tm easter(int year);
struct tm ash_wednesday_from_easter(struct tm easter);
struct tm first_sunday_of_advent(int year);

#endif
