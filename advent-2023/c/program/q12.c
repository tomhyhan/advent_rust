#include "call_lib.h"

#define BUCKET_SIZE 2097169   
// #define BUCKET_SIZE 4194319 

typedef struct {
    size_t r_idx, p_idx;
} state_t;

typedef struct Key {
    state_t key;
    int64_t value;
} key_t;

key_t* BUCKET[BUCKET_SIZE];

typedef struct {
    char record[200];
    size_t patterns[50];
    size_t pattern_size;
} spring_t;

void insert_bucket(size_t b_idx, key_t new_node) {
    size_t start_idx = b_idx;
    do {
        key_t** nodep = &BUCKET[b_idx];
        if (*nodep == NULL) {
            *nodep = malloc(sizeof(key_t));
            memcpy(*nodep, &new_node, sizeof(key_t));
            return;
        }
        b_idx = (b_idx + 1) % BUCKET_SIZE; 
    } while (start_idx != b_idx);
}

bool compare_state(state_t s1, state_t s2) {
    return s1.r_idx == s2.r_idx && s1.p_idx == s2.p_idx;
}

key_t* search_bucket(size_t b_idx, state_t key) {
    size_t start_idx = b_idx;

    do {
        key_t** nodep = &BUCKET[b_idx];
        if (*nodep == NULL) {
            return NULL;
        } 

        if (compare_state(key, (*nodep)->key)) {
            return *nodep;
        }
        b_idx = (b_idx + 1) % BUCKET_SIZE; 
    } while (start_idx != b_idx);
}

void init_bucket(void) {
    memset(BUCKET, 0, sizeof(BUCKET));
}

size_t get_bucket_idx(state_t key) {
    size_t hash_key = hash_65599((const char*)&key, sizeof(key));
    return hash_key % BUCKET_SIZE;
}

spring_t* create_springs(char* input) {
    int offset = 0, read;
    char patterns_s[100]; 
    spring_t* springs = create_list(spring_t);
    spring_t s = {0};
    while (sscanf(input + offset, "%s %s\n%n", s.record, patterns_s, &read) == 2) {
        char* token;        
        char delim[] = ",";

        token = strtok(patterns_s, delim);
        while (token) {
            size_t num = atoll(token); 
            s.patterns[s.pattern_size++] = num;
            token = strtok(NULL, delim);
        }
        
        push_list(springs, s);
        memset(&s,0,sizeof(s));
        offset += read;
    }
    return springs;
}

bool pattern_matches(spring_t spring, size_t r_idx, size_t current_p) {
    size_t start = r_idx;
    size_t end = r_idx + current_p;

    if (strlen(spring.record) < end) return FALSE;

    for (; start < end; start++ ) {
        if (spring.record[start] == '.') return FALSE;
    }

    if (end < strlen(spring.record) && spring.record[end] == '#') return FALSE; 

    return TRUE;
}


size_t arrange_spring(spring_t spring, size_t r_idx, size_t  p_idx) {
    size_t arrs = 0; 
    size_t current_p = spring.patterns[p_idx];
    char tile = spring.record[r_idx];
    key_t key = {0};
    size_t hash_key;
    state_t state = {0};
    state.r_idx = r_idx;
    state.p_idx = p_idx;
    hash_key = get_bucket_idx(state);

    if (search_bucket(hash_key, state)) {
        return BUCKET[hash_key]->value; 
    }

    if (r_idx >= strlen(spring.record)) {
        return p_idx == spring.pattern_size;
    }

    if (tile =='.' || tile == '?') {
        arrs += arrange_spring(spring, r_idx + 1, p_idx);
    }
    
    if (tile =='#' || tile == '?') {
        if (pattern_matches(spring, r_idx, current_p)) {
            arrs += arrange_spring(spring, r_idx + current_p + 1, p_idx + 1);
        } 
    }
    
    key.key = state;
    key.value = arrs;
    insert_bucket(hash_key, key);
    return arrs;
}

void part1(char* input) {
    spring_t* springs = create_springs(input);
    size_t i, arrangements = 0;

    
    for (i=0; i < list_len(springs); i++) {
        spring_t* s = &springs[i];
        size_t j;
        size_t p_len = s->pattern_size;
        char tmp[200];
        memcpy(tmp+1, s->record, sizeof(tmp)-1);
        tmp[0] = '?';
        for (j=0; j < 4; j++) {
            strcat(s->record, tmp);
        }
        for (j=p_len; j < p_len*5; j++) {
            s->patterns[s->pattern_size++] = s->patterns[j % p_len]; 
        }
    }
    
    for (i=0; i < list_len(springs); i++) {
        init_bucket();
        arrangements += arrange_spring(springs[i], 0, 0);
    }
    printf("%lld\n", arrangements);

    destroy_list(springs);
}

void part2(char* input) {
    // get next best prime
    // printf("%lld", (size_t)(1 << 20) * 2);
}


SOLUTION("./inputs/q12.txt")
