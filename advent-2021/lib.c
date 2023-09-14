#include "lib.h"
#include <stdio.h>
#include <stdlib.h>

FILE *read_file_data(const char *file_name) {
    FILE *file;
    errno_t err;

    err = fopen_s(&file, file_name, "r");

    if (err != 0) {
        printf("Failed to open the file. Error code: %d\n", err);
        exit(EXIT_FAILURE);
    }
    
    return file;
}
