#ifndef BUILDER_H
#define BUILDER_H

#include "church_day.h"
#include <time.h>

//calculate proper of seasons for a given date
ChurchYear proper_of_seasons(struct tm today);

#endif //BUILDER_H
