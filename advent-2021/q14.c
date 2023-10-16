#include "lib.h"

#define LETTER_COMP (26*26)

int64_t id(char a , char b) {
    return (a - 'A') * 26 + (b - 'A');
}

void parse_polymer(FILE* file, char* polymer, char pair_rules[]) {
    char from[5], to;
    fscanf_s(file, "%s\n\n", polymer, 50);
    printf("%s\n", polymer);

    while (fscanf_s(file, "%s -> %c\n", from, sizeof(from) , &to) != EOF) {
        pair_rules[id(from[0],from[1])] = to;
    }
}

void get_min_max(int64_t* min_cnt, int64_t* max_cnt, int64_t counts[]) {
    size_t i;
    for (i=0; i < 26; i++) {
        if (counts[i] ==0) continue;
        *min_cnt = MIN(*min_cnt, counts[i]);
        *max_cnt = MAX(*max_cnt, counts[i]);
    }
}

void pair_insertion(const char* polymer, char pair_rules[], size_t steps){
    int64_t pairs[LETTER_COMP] = {0};
    int64_t counts[26] = {0};
    size_t step, i;
    int64_t min_cnt = INT64_MAX, max_cnt = INT64_MIN; 
    for (i=0; i < strlen(polymer); i++) {
        counts[polymer[i] - 'A']++; 
    }

    for (i=0; i < strlen(polymer)-1; i++) {
        pairs[id(polymer[i], polymer[i+1])]++;
    }

    for (step=0; step < steps; step++) {
        int64_t new_pairs[LETTER_COMP] = {0};
        for (i=0; i < LETTER_COMP; i++) {
            char insert;
            if (pairs[i] == 0) continue;
            insert = pair_rules[i];
            counts[insert - 'A'] += pairs[i];
            new_pairs[((i / 26) * 26) + (insert - 'A')] += pairs[i];
            new_pairs[((insert - 'A') * 26) + (i % 26)] += pairs[i];
        }
        memcpy(pairs, new_pairs, sizeof(pairs));
    }
    get_min_max(&min_cnt, &max_cnt, counts);
    printf("min: %lld\n", min_cnt);
    printf("max: %lld\n", max_cnt);
    printf("result: %lld\n", max_cnt - min_cnt);
}

void solution(FILE *file) {
    char polymer[50];
    char pair_rules[LETTER_COMP] = {0};
    parse_polymer(file, polymer, pair_rules);
    pair_insertion(polymer, pair_rules, 40);

}

AOC_MAIN_ONE("./inputs/q14.txt")
