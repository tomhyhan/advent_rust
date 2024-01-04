#include "call_lib.h"

#define BUCKET_SIZE 1707
// #define BUCKET_SIZE 27

typedef struct n {
    char key[4];
    char value[2][4];
} n_t;

n_t* BUCKET[BUCKET_SIZE];

typedef struct {
    char ins[1000];
} network_t;

void insert_bucket(size_t b_idx, n_t new_node) {
    size_t start_idx = b_idx;
    do {
        n_t** nodep = &BUCKET[b_idx];
        if (*nodep == NULL) {
            *nodep = malloc(sizeof(n_t));
            memcpy(*nodep, &new_node, sizeof(n_t));
            break;
        }
        b_idx = (b_idx + 1) % BUCKET_SIZE; 
    } while (start_idx != b_idx);
}

n_t* search_bucket(size_t b_idx, char* key) {
    size_t start_idx = b_idx;

    do {
        n_t** nodep = &BUCKET[b_idx];
        if (*nodep == NULL) {
            return NULL;
        } 

        if (strcmp(key, (*nodep)->key) == 0) {
            return *nodep;
        }
        b_idx = (b_idx + 1) % BUCKET_SIZE; 
    } while (start_idx != b_idx);
}

void init_bucket(void) {
    memset(BUCKET, 0, sizeof(BUCKET));
}

size_t get_bucket_idx(char* key) {
    size_t hash_key = hash_65599(key, strlen(key));
    return hash_key % BUCKET_SIZE;
}

network_t parse(char* input) {
    char* token = NULL;
    char delim[] = "\n\n";
    int i;
    network_t network = {0};
    size_t bucket_i;
    n_t* np;
    bool first = TRUE;
    init_bucket();
    token = strtok(input, delim);

    while (token) {
        if (first) {
            memcpy(network.ins, token, strlen(token)+1);
            first = FALSE;
        } else {
            n_t node = {0};
            size_t bucket_idx;
            sscanf(token, "%s = (%[^,], %[^)])", node.key, node.value[0], node.value[1]);
            bucket_idx = get_bucket_idx(node.key);
            insert_bucket(bucket_idx, node);
        }
        token = strtok(NULL, delim);
    }
    return network;
}

void part1(char* input) {
    network_t network = parse(input);
    size_t i = 0;
    int steps = 0;

    char* key = "AAA";
    while (TRUE) {
        char dir = network.ins[i];
        size_t bucket_idx = get_bucket_idx(key);
        n_t* node = search_bucket(bucket_idx, key);

        if (strcmp(key, "ZZZ") == 0) {
            break;
        }

        key = dir == 'R'? node->value[1]: node->value[0];
        i = (i + 1) % strlen(network.ins);
        steps++;
    }
    printf("%d\n", steps);
}

int64_t gcd(int64_t x, int64_t y) {
    return x % y == 0? y: gcd(y, x % y);
}

int64_t lcm(int* nums) {
    size_t i;
    int64_t result = nums[0];
    for (i=1; i < list_len(nums); i++) {
        result = result * nums[i] / gcd(result, nums[i]);
    }
    return result;
}

void part2(char* input) {
    network_t network = parse(input);
    size_t i = 0;
    int* steps = create_list(int); 
    int64_t result;

    for (i=0; i < BUCKET_SIZE; i++) {
        n_t* node = BUCKET[i];
        if (node && node->key[2] == 'A') {
            char* key = node->key;
            int step = 0;
            int idx = 0;
            while (TRUE) {
                char dir = network.ins[idx];
                size_t bucket_idx = get_bucket_idx(key);
                n_t* node = search_bucket(bucket_idx, key);
                if (key[2] == 'Z') {
                    break;
                }

                key = dir == 'R'? node->value[1]: node->value[0];
                idx = (idx + 1) % strlen(network.ins);
                step++;
            }
            push_list(steps, step);
        }
    }
    
    result = lcm(steps);
    printf("result: %lld\n", result);

    
    for (i=0; i < BUCKET_SIZE; i++) {
        n_t* node = BUCKET[i];
        if (node) {
            free(node);
        }
    }
    destroy_list(steps);
}


SOLUTION("./inputs/q8.txt")
