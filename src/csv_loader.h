#ifndef CSV_LOADER_H
#define CSV_LOADER_H

#include "church_day.h"

// Load feast days from CSV file and add them to the ChurchYear
// Returns 0 on success, -1 if file not found, -2 on parse error
int load_feasts_from_csv(ChurchYear *year, const char *filepath);

#endif // CSV_LOADER_H
