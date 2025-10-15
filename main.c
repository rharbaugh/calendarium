#include <stdio.h>
#include <time.h>

int main(void) {
    // Get current time
    time_t now = time(NULL);
    
    if (now == -1) {
        fprintf(stderr, "Error: Failed to get current time\n");
        return 1;
    }
    
    // Convert to local time
    struct tm local_time;
    
    #ifdef _WIN32
        if (localtime_s(&local_time, &now) != 0) {
            fprintf(stderr, "Error: Failed to convert time\n");
            return 1;
        }
    #else
        if (localtime_r(&now, &local_time) == NULL) {
            fprintf(stderr, "Error: Failed to convert time\n");
            return 1;
        }
    #endif
    
    // Extract date components
    const int year = local_time.tm_year + 1900;
    const int month = local_time.tm_mon + 1;
    const int day = local_time.tm_mday;
    
    // Print the current date
    printf("Current Date: %04d-%02d-%02d\n", year, month, day);
    
    return 0;
}
