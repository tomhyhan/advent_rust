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
    hailstone_t* hailstones = parse(input);
    size_t i, j;
    bool xs[2000] = {0};
    bool x_first = TRUE;
    bool ys[2000] = {0};
    bool y_first = TRUE;
    bool zs[2000] = {0};
    bool z_first = TRUE;
    hailstone_t eh1 = hailstones[0];
    hailstone_t eh2 = hailstones[1];
    double m1, m2, b1, b2, t, x, y, z;

    for (i=0; i < list_len(hailstones); ++i) {
        for (j=i+1; j < list_len(hailstones); ++j) {
            int64_t k;
            hailstone_t h1 = hailstones[i];
            hailstone_t h2 = hailstones[j];
            int new_xs[2000] = {0}; 
            int new_ys[2000] = {0}; 
            int new_zs[2000] = {0}; 

            if (h1.dx == h2.dx) {
                int64_t d =  h2.x - h1.x;
                for (k= -1000; k < 1000; ++k) {
                    if (k == (int64_t)h1.dx) continue;
                    if (d % (k-(int64_t)h1.dx) == 0) {
                        new_xs[k+1000] = TRUE;
                    }
                }
                if (x_first) {
                    memcpy(xs, new_xs, sizeof(xs));
                    x_first = FALSE;
                } else {
                    for (k=0; k < 2000; ++k) {
                        if(new_xs[k] && xs[k]) continue; 
                        xs[k] = FALSE;
                    }
                }
            }

            if (h1.dy == h2.dy) {
                int64_t d =  h2.y - h1.y;
                for (k= -1000; k < 1000; ++k) {
                    if (k == (int64_t)h1.dy) continue;
                    if (d % (k-(int64_t)h1.dy) == 0) {
                        new_ys[k+1000] = TRUE;
                    }
                }
                if (y_first) {
                    memcpy(ys, new_ys, sizeof(ys));
                    y_first = FALSE;
                } else {
                    for (k=0; k < 2000; ++k) {
                        if(new_ys[k] && ys[k]) continue; 
                        ys[k] = FALSE;
                    }
                }
            }

            if (h1.dz == h2.dz) {
                int64_t d =  h2.z - h1.z;
                for (k= -1000; k < 1000; ++k) {
                    if (k == (int64_t)h1.dz) continue;
                    if (d % (k-(int64_t)h1.dz) == 0) {
                        new_zs[k+1000] = TRUE;
                    }
                }
                if (z_first) {
                    memcpy(zs, new_zs, sizeof(zs));
                    z_first = FALSE;
                } else {
                    for (k=0; k < 2000; ++k) {
                        if(new_zs[k] && zs[k]) continue; 
                        zs[k] = FALSE;
                    }
                }
            }
        }
    }

    for (i=0; i < 2000; i++) {
        if (xs[i]) {
            printf("x %lld\n", i - 1000);
        }
        if (ys[i]) {
            printf("y %lld\n", i - 1000);
        }
        if (zs[i]) {
            printf("z %lld\n", i - 1000);
        }
    }

    // x - -110 y - -135 z - 299
    m1 = (eh1.dy - -135) / (eh1.dx - -110);
    m2 = (eh2.dy - -135) / (eh2.dx - -110);
    // y - cy = m(x - cx)
    // x = (b2 - b1) / (m1 - m2);
    b1 = -m1*eh1.x + eh1.y ;
    b2 = -m2 * eh2.x + eh2.y ;
    x = (b1 - b2) / (m2 - m1);
    y = m1 * x + b1;
    t = (eh1.x - x) / (-110 - eh1.dx);
    z = eh1.z + (eh1.dz * t) - (299 * t);
    printf("%lf\n", x);
    printf("%lf\n", y);
    printf("%lf\n", z);
    printf("%lf\n", x + y + z);
    destroy_list(hailstones);
}

SOLUTION("./inputs/q24.txt")
