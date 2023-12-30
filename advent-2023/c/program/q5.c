#include "call_lib.h"

#define q5type int64_t

typedef struct {
    q5type nums[50];
    q5type size;
} seed_t;

typedef struct {
    q5type des, src, range; 
} map_t;

typedef struct {
    seed_t seed;
    map_t* maps[10];
    q5type map_size;
} fer_t;

typedef struct {
    q5type min, max;
    q5type map_idx;
} srange_t;

void add_map_to_fert(fer_t* fert, char* input, map_t* maplist) {
    int read, offset = 0;
    char line[200];
    while (sscanf(input + offset, "%[^\n]%n", line, &read)) {
        q5type des, src, range;
        map_t map = {0};
        if (isdigit(*line)) {
            sscanf(line, "%lld %lld %lld", &des, &src, &range); 
            map.des = des;
            map.src = src;
            map.range = range;
            push_list(maplist, map);
        }
        offset += read + 1;
    }
    fert->maps[fert->map_size++] = maplist; 
}

fer_t create_map(char* input) {
    
    char* token = input;
    char delim[] = "\n\n";
    map_t* maplist = create_list(map_t);
    fer_t fert = {0};

    token = strstr(input, delim);
    *token = '\0';

    while (*input != '\0') {
        char* endptr;
        q5type num = strtoll(input, &endptr, 10);
        if (input != endptr) {
            fert.seed.nums[fert.seed.size++] = num;
            input = endptr;
        } else {
            input++;
        }
    }
    input = token + 2;
    token = strstr(input, delim);

    while (token) {
        *token = '\0';
        add_map_to_fert(&fert, input, maplist);
        input = token + 2;
        token = strstr(input, delim);
        maplist = create_list(map_t);
    }
    
    add_map_to_fert(&fert, input, maplist);
    return fert;
}

q5type find_next_seed(q5type seed, map_t* maplist) {
    q5type i;
    for (i=0; i < list_len(maplist); i++) {
        map_t map = maplist[i];
        if (seed >= map.src && seed <= map.src + map.range - 1 ) {
            q5type diff = map.src - map.des;
            return seed - diff;
        }
    }
    return seed;
}

void part1(char* input) {
    fer_t fert = create_map(input);
    q5type i, j;
    q5type lowest = INT_MAX;
    for (i=0; i < fert.seed.size; i++) {
        q5type seed = fert.seed.nums[i];
        for (j=0; j < fert.map_size; j++) {
            seed = find_next_seed(seed, fert.maps[j]);
        }
        lowest = MIN(lowest, seed);
    }
    for (i=0; i < fert.map_size; i++) {
        destroy_list(fert.maps[i]);
    }
    printf("lowest: %lld\n", lowest);
}

void part2(char* input) {
    fer_t fert = create_map(input);
    q5type i, j;
    q5type lowest = INT_MAX;
    srange_t* seed_ranges = create_list(srange_t);

    for (i=0; i < fert.seed.size; i += 2) {
        q5type start = fert.seed.nums[i];
        q5type end = fert.seed.nums[i+1];
        srange_t srange = {0};
        srange.min = start;
        srange.max = start + end - 1;
        push_list(seed_ranges, srange);
    }
    while (list_len(seed_ranges) > 0) {
        srange_t srange = {0};
        map_t* maplist;
        int i;
        bool found = FALSE;
        pop_list(seed_ranges, &srange);
        if (srange.map_idx == 7) {
            lowest = MIN(lowest, srange.min);
            continue;
        }

        maplist = fert.maps[srange.map_idx];
        for (i=0; i < list_len(maplist); i++) {
            map_t map = maplist[i];
            if (srange.min >= map.src && srange.min < map.src + map.range) {
                q5type diff = map.src - map.des;
                if (srange.max >= map.src + map.range) {
                    srange_t new_srange = {0}; 
                    new_srange.min = map.src + map.range;
                    new_srange.max = srange.max;
                    new_srange.map_idx = srange.map_idx;

                    srange.min -= diff;
                    srange.max = map.src + map.range -1 - diff;
                    srange.map_idx++;

                    push_list(seed_ranges, srange);
                    push_list(seed_ranges, new_srange);
                } else {
                    srange.min -= diff;
                    srange.max -= diff;
                    srange.map_idx++;
                    
                    push_list(seed_ranges, srange);
                }
                found = TRUE;
            }
            if (found) {
                break;
            }
        }
        if (!found) {
            srange.map_idx++;
            push_list(seed_ranges, srange);
        }
    }
    printf("lowest: %lld\n", lowest);
}

SOLUTION("./inputs/q5.txt")
