#define _CRT_SECURE_NO_WARNINGS

#include "lib.h"

typedef struct{
    int64_t x, y, d;
} sensor_t;

typedef struct{
    int64_t x, y;
} beacon_t;

static int16_t beacon_hash(int64_t bx, int64_t by) {
    static beacon_t beacons[30] = {0};
    static int16_t idx = 0;
    size_t i;

    for (i=0; i < idx; i++) {
        beacon_t b = beacons[i];
        if (b.x == bx && b.y == by) {
            return -1;
        }
    } 

    beacons[idx].x = bx;
    beacons[idx].y = by;

    return idx++;
}

int64_t md(int64_t x, int64_t y, int64_t x1, int64_t y1) {
    return (abs(x-x1) + abs(y-y1));
}

bool check_distance(PointVector* sensors, int64_t nx, int64_t ny) {
    size_t i;
    for(i=0; i < sensors->size; i++) {
        sensor_t* sensor = sensors->array[i];
        int64_t distance_to_new = md(nx,ny,sensor->x, sensor->y);
        if (sensor-> d >= distance_to_new) {
            return FALSE;
        }
    }
    return TRUE;
}

void find_unique_beacon(PointVector* sensors, beacon_t beacons[]) {
    size_t i;
    for (i=0; i < sensors->size; i++) {
        sensor_t* sensor = sensors->array[i];
        int64_t dy, dx;
        for (dx=0; dx <= sensor->d + 1; dx++) {
            size_t i;
            int8_t dirs[4][2] = {{1,1}, {-1,1}, {1,-1}, {-1,-1}}; 
            // expanding by one point
            dy = (sensor->d + 1) - dx;
            for (i=0; i < 4; i++){
                int64_t nx = (sensor-> x + dx) * dirs[i][0];
                int64_t ny = (sensor-> y + dy) * dirs[i][1];
                if  (nx >= 0 && nx <= 4000000 && ny >= 0 && ny <= 4000000) {
                    if (check_distance(sensors, nx, ny)) {
                        printf("%lld\n", nx * 4000000 + ny);
                        return;
                    }
                }
            }
        }
    }
}

void solution(FILE* file) {
    int64_t x, y, bx, by;
    PointVector* sensors = init_ptr_vector(1000);
    beacon_t beacons[30] = {0};
    
    while (fscanf(file,  "Sensor at x=%lld, y=%lld: closest beacon is at x=%lld, y=%lld\n", &x, &y, &bx, &by) != EOF) {
        sensor_t* sensor = malloc(sizeof(sensor_t));
        int64_t idx = beacon_hash(bx, by);
        beacon_t beacon = {0};
        
        sensor->x = x;
        sensor->y = y;
        sensor->d = md(x,y,bx,by);
        push_pv(sensors, sensor);
        
        if (idx >= 0) {
            beacon.x = bx;
            beacon.y = by;
            beacons[idx] = beacon;
        }
    }; 

    // printf("%lld %lld\n", beacons[1].x, beacons[1].y);
    printf("%lld\n", sensors->size);

    find_unique_beacon(sensors, beacons);

    free_ptr_vector(sensors);
}

AOC_MAIN_ONE("./inputs/q15-2023.txt")
