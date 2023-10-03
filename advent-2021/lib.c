#include "lib.h"
#include "vector.c"
#include "ptr_vector.c"

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

int split_string(Vector *vector, const char delimeter[],char line[]) {
    char *token = NULL, *next_token = NULL;
    int num;
    token = strtok_s(line, delimeter, &next_token);
    while (token != NULL) {
        num = atoi(token);
        push(vector, num);
        token = strtok_s(NULL, delimeter, &next_token);
    }
    return 1;
}



