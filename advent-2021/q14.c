#include "lib.h"

typedef struct {
    int x;
} test ;

test parse_polymer(void) {
    test t = {0};
    t.x = 10;
    return t;
}


void solution(FILE *file) {
    test t = parse_polymer();
    printf("%d\n", t.x);
}

AOC_MAIN_ONE("./inputs/q14.txt")
