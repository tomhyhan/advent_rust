#include "lib.h"

#define C 1000

static void simulate(Vector *vector, int days);

static int part1(FILE *file)
{
    Vector *vector = init_vector(C);
    char line[C];

    fgets(line, sizeof(line), file);
    split_string(vector, ",", line);
    simulate(vector, 18);
    free_vector(vector);
    return 1;
}

static void simulate(Vector *vector, int days) {
    unsigned i;
    while (days >0) {
        unsigned int current_size = vector->size, zeros = 0;
        for (i=0; i < current_size; i++) {
            int* fish = &vector->array[i];
            *fish -= 1;
            if (*fish < 0) {
                *fish = 6;
                push(vector, 8);
            }
        }
        days--;
    }
    printf("%d\n", vector->size);
}

AOC_MAIN()

