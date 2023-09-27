#include "lib.h"

#define N 10000

int64_t factorial(int64_t n, int64_t sum) {
    if (n == 0) {
        return sum;
    }
    sum += n;
    return factorial(n-1, sum);
}

int64_t calc_feul(Vector *vector, int64_t center) {
    size_t i;
    int64_t feul = 0;
    for (i=0; i < vector->size; i++) {
        feul += llabs(vector->array[i] - center);
    }
    return feul;
}

int64_t calc_feul_part2(Vector *vector, int64_t center) {
    size_t i;
    int64_t feul = 0;
    for (i=0; i < vector->size; i++) {
        int64_t distance = llabs(vector->array[i] - center);
        feul += factorial(distance,0);
    }
    return feul;
}

void get_min_max(int64_t* max_num, int64_t* min_num, Vector *vector) {
    size_t i;
    for (i=0;i < vector-> size; i++) {
        *max_num = MAX(*max_num, vector->array[i]);
        *min_num = MIN(*min_num, vector->array[i]);
    }
}

int64_t get_min_feul(Vector *vector, int64_t min_num, int64_t max_num) {
    int64_t left_f, right_f, center_f, center = (max_num + min_num) / 2;
    left_f = calc_feul(vector, min_num);
    right_f = calc_feul(vector, max_num);
    
    if (center == max_num || center == min_num) {
        return MIN(left_f, right_f);
    }

    center_f = calc_feul_part2(vector, center);

    if (left_f < right_f) {
        return get_min_feul(vector, min_num, center);
    } else if (left_f > right_f) {
        return get_min_feul(vector, center, max_num);
    } else {
        printf("%lld %lld %lld\n", min_num, center , max_num);
        printf("%lld %lld %lld\n", left_f, center_f, right_f);
        exit(1);
    }
}

int solution(FILE *file) {
    Vector *vector = init_vector(10000);
    int64_t feul, max_num = INT64_MIN, min_num = INT64_MAX;
    char line[N]; 
    
    fgets(line , sizeof(line), file);
    split_string(vector, ",", line);
    get_min_max(&max_num, &min_num, vector);
    
    feul = get_min_feul(vector, min_num, max_num);
    printf("feul - %lld\n", feul);

    free_vector(vector);
    return 1;
}


AOC_MAIN_ONE("./inputs/q7.txt")

