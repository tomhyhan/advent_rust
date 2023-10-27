#include "lib.h"

typedef struct {
    int32_t x;
    int32_t y;
    int32_t z;
} point_t;

typedef struct {
    point_t points[100];
    size_t i;
} scan_t;


PointVector* parse_scanners(FILE* file) {
    PointVector* scanners = init_ptr_vector(1000);
    char line[1000];
    scan_t* scanner = (scan_t*)calloc(1,sizeof(scan_t));

    while (fscanf_s(file, "%[^\n]\n", line, sizeof(line)) != EOF ) {
        point_t pt = {0};

        if (line[1] == '-') {
            if (scanner->i > 0 ) {
                printf("%s\n", line);
                push_pv(scanners, scanner);
                scanner = (scan_t*)calloc(1,sizeof(scan_t));
            }
            continue;
        }
        sscanf_s(line, "%d,%d,%d", &pt.x,&pt.y,&pt.z);
        // printf("%d %d %d\n",  pt.x, pt.y, pt.z);
        scanner->points[scanner->i++] = pt; 
    }
    push_pv(scanners, &scanner);
    
    return scanners;
}

void scan_beacons(PointVector* scanners) {
    size_t i;
    for (i=0; i< scanners->size; i++) {
        
    }
}

void solution(FILE* file) {
    PointVector* scanners = parse_scanners(file);
    scan_beacons(scanners);
    
    // printf("%d\n", scanners->size);
    // printf("%d\n", ((scan_t*)scanners->array[1])->points[0].x);
    // printf("%d\n", ((scan_t*)scanners->array[0])->points[5].x);
    
    
}

AOC_MAIN_ONE("./inputs/q19.txt")
