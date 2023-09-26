#include "lib.h"

#define C 1000

static void simulate(Vector *vector, int days);
static void simulate2(Vector *vector, size_t days);

static int part2(FILE * file) {
    char line[C];
    Vector *vector = init_vector(C);

    fseek(file, 0, SEEK_SET);
    fgets(line, sizeof(line), file);
    split_string(vector,",",line);
    simulate2(vector, 256 );   

    return 1;
}

static void simulate2(Vector *vector, size_t days) {
    size_t i;
    uint64_t total=0, fishes[9] = {0};

    for (i=0; i < vector->size; i++) {
        fishes[vector->array[i]] += 1;
    }

    for (i=0; i < days; i++) {
        // 0 1 2 1 1 0 0 0 0 
        fishes[(i+7)%9] += fishes[i%9]; 
    }
    
    for (i=0; i < 9; i++) {
        total += fishes[i];
    }
    printf("%llu", total);
}


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
        unsigned int current_size = vector->size;
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

