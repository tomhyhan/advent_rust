#ifndef LIB
#define LIB

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <inttypes.h>

#include "vector.h"

FILE *read_file_data(const char *file_name);
int split_string(Vector *vector, const char delimeter[],char line[]);

#define AOC_MAIN(filename)                                    \
  int main(void) {                                           \
    FILE *file = read_file_data(filename);                                 \
    part1(file);                                                                \
    part2(file);                                                                \
    fclose(file);                                                               \
    return EXIT_SUCCESS;                                                       \
}

#define AOC_MAIN_ONE(filename)                                    \
  int main(void) {                                           \
    FILE *file = read_file_data(filename);                                 \
    solution(file);                                                                \
    fclose(file);                                                               \
    return EXIT_SUCCESS;                                                       \
}

#endif


#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) < (b) ? (a) : (b))
