#ifndef LIB
#define LIB

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "vector.h"

FILE *read_file_data(const char *file_name);
int split_string(Vector *vector, const char delimeter[],char line[]);

#define AOC_MAIN()                                    \
  int main(int argc, char *argv[]) {                                           \
    FILE *file = read_file_data("./inputs/q6.txt");                                 \
    part1(file);                                                                \
    fclose(file);                                                               \
    return EXIT_SUCCESS;                                                       \
  }

#endif
