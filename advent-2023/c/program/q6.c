#include "call_lib.h"

typedef struct {
    int arr[5];
    size_t size;
} arr_t;

void add_nums(char* line, arr_t* arr) {
    int num;
    char* endp;
    while (TRUE) {
        num = strtol(line, &endp, 10);
        if (num == 0) {
            break;
        }
        arr->arr[arr->size++] = num;
        line = endp;
    }
}

void parse(char* input, arr_t* times, arr_t* distances) {
    char line[200];
    char* pend;
    int offset = 0, read;

    sscanf(input + offset, "%*s%[^\n]%n", line, &read);
    add_nums(line, times);
    offset += read + 1;
    
    sscanf(input + offset, "%*s%[^\n]%n", line, &read);
    add_nums(line, distances);
}

void part1(char* input) {
    int i, record = 1;
    arr_t times = {0};
    arr_t distances = {0};
    parse(input, &times, &distances);

    for (i=0; i < times.size; i++) {
        int time = times.arr[i];
        int distance = distances.arr[i];
        int hold, wins = 0;
        for (hold=1; hold < time; hold++) {
            if (hold * (time - hold) > distance) {
                wins++;
            }
        }
        record *= wins;
    }
    printf("%d\n", record);
}

int64_t binary_search(int64_t t, int64_t d) {
    int64_t low = 0, high = t;
    while (low < high) {
        int64_t mid = (low + high) / 2;
        int64_t current_d = (t - mid) * mid;
        if (current_d > d) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    printf("%lld %lld\n", low, high);
    return low;
}

void part2(char* input) {
    int i, record = 1;
    char time[100];
    char distance[100];
    arr_t times = {0};
    arr_t distances = {0};
    int64_t t,d, lower_b;
    parse(input, &times, &distances);

    sprintf(time, "%d%d%d%d", times.arr[0], times.arr[1], times.arr[2],times.arr[3]);
    sprintf(distance, "%d%d%d%d", distances.arr[0], distances.arr[1], distances.arr[2], distances.arr[3]);

    t = atoll(time);
    d = atoll(distance);

    lower_b = binary_search(t, d);
    // printf("%lld\n", lower_b);
    printf("%lld\n", t - lower_b - lower_b + 1);
    
}

SOLUTION("./inputs/q6.txt")



