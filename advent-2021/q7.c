#include "lib.h"

#define N 1000

int calc_feul(Vector *vector, int64_t center) {
    return 1;
}

void get_min_max(int64_t* max_num, int64_t* min_num, Vector *vector) {
    int64_t num;
    size_t i;
    for (i=0;i < vector-> size; i++) {
        num = vector->array[i];
        *max_num = MAX(*max_num, vector->array[i]);
        *min_num = MIN(*min_num, vector->array[i]);
    }
}

int part1(FILE *file) {
    Vector *vector = init_vector(N);
    int64_t max_num = INT64_MIN, min_num = INT64_MAX;
    char line[N]; 
    
    fgets(line , sizeof(line), file);
    split_string(vector, ",", line);
    get_min_max(&max_num, &min_num, vector);
    printf("%lld\n", MAX(INT64_MIN, vector->array[0]));
    return 1;
}

int part2(FILE *file) {
    printf("part2");
    return 1;
}


AOC_MAIN("./inputs/q7.txt")

