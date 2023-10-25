#include "lib.h"

typedef struct Velocity{
    int32_t min_x;
    int32_t max_x;
    int32_t min_y;
    int32_t max_y;
} velocity_t;

static uint32_t find_max_y(velocity_t v, int32_t vel_x, int32_t vel_y) {    
    int32_t x = 0,y = 0;
    int32_t max_y = 0;

    while (x <= v.max_x && y >= v.min_y) {
        x += vel_x;
        y += vel_y;
        vel_x = (vel_x - 1 < 0) ? 0 : vel_x - 1;
        vel_y -= 1;

        max_y = MAX(max_y, y);

        if (x >= v.min_x && x <= v.max_x && y >= v.min_y && y <= v.max_y) {
            return max_y;
        }
    }
    return 0;
}

bool is_target_valid(velocity_t v, int32_t vel_x, int32_t vel_y) {    
    int32_t x = 0,y = 0;
    int32_t max_y = 0;

    while (x <= v.max_x && y >= v.min_y) {
        x += vel_x;
        y += vel_y;
        vel_x = (vel_x - 1 < 0) ? 0 : vel_x - 1;
        vel_y -= 1;

        max_y = MAX(max_y, y);

        if (x >= v.min_x && x <= v.max_x && y >= v.min_y && y <= v.max_y) {
            return TRUE;
        }
    }
    return FALSE;
}


void solution(FILE *file) {
    
    uint64_t max_y = 0; 
    int32_t x, y, points = 0;
    static struct Velocity v;
    
    fscanf_s(file, "target area: x=%d..%d, y=%d..%d",&v.min_x, &v.max_x, &v.min_y, &v.max_y);

    for (x=0; x <= v.max_x; x++) {
        for (y = v.min_y; y <= -v.min_y; y++) {
            max_y = MAX(max_y, find_max_y(v, x,y));            
        }
    }
    printf("max y: %lld\n", max_y);
    
    for (x=0; x <= v.max_x; x++) {
        for (y = v.min_y; y <= -v.min_y; y++) {
            if (is_target_valid(v, x,y)) {
                points++;
            };            
        }
    }
    printf("%d\n", points);
}

AOC_MAIN_ONE("./inputs/q17.txt")
