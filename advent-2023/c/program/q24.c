#include "call_lib.h"

typedef struct {
    double x,y,z;
    double dx,dy,dz;
} hailstone_t;

double MIN = 200000000000000.0; 
double MAX = 400000000000000.0;

hailstone_t* parse(char* input) {
    int offset = 0, read;
    hailstone_t hs = {0};
    hailstone_t* hailstones = create_list(hailstone_t);

    while (sscanf(input + offset, "%lf, %lf, %lf @ %lf,  %lf, %lf\n%n", &hs.x, &hs.y, &hs.z, &hs.dx, &hs.dy, &hs.dz, &read) == 6)  {
        push_list(hailstones, hs);
        offset += read;
    }
    return hailstones;
}

bool cross_xy(hailstone_t h1, hailstone_t h2) {
    double x,y;
    double m1 = h1.dy / h1.dx;
    double m2 = h2.dy / h2.dx; 
    double b1 = -m1 * h1.x + h1.y;
    double b2 = -m2 * h2.x + h2.y;
    
    if (m1 == m2) return FALSE;

    x = (b2 - b1) / (m1 - m2);
    y = m1 * x + b1;

    if ((h1.dx < 0 && x > h1.x) || (h1.dx > 0 && x < h1.x) || (h2.dx < 0 && x > h2.x) || (h2.dx > 0 && x < h2.x) ) return FALSE;

    if (x < MIN || x > MAX || y < MIN || y > MAX) return FALSE;

    return TRUE;
}

void part1(char* input) {
    hailstone_t* hailstones = parse(input);
    size_t i, j, crosses = 0;

    for(i=0; i < list_len(hailstones); ++i) {
        for (j=i+1; j < list_len(hailstones); ++j) {
            hailstone_t h1 = hailstones[i];
            hailstone_t h2 = hailstones[j];
            if (cross_xy(h1,h2)) {
                ++crosses;
            }
        }
    }
    
    printf("%lld\n", crosses);
    destroy_list(hailstones);
}

void part2(char* input) {
    // distance % (vr - vh) = 0
}

SOLUTION("./inputs/q24.txt")
