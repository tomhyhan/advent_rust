#include "call_lib.h"

// http://clb.confined.space/aoc2023/#day20code looks cool

typedef struct node node_t;

struct node {
    char name[4];
    bool ff, hi;
    int n_in, n_out;
    node_t* in[10];
    node_t* out[10];
    int x;
} ht[256];

// defaultdict
struct node* get(char* src) {
    char c =  src[1] ? src[2]: 0;
    uint8_t hash_key = src[0] ^ (((uint16_t)src[1] << 1) ^ ((uint16_t)c << 2));
    for (;ht[hash_key].name[0];++hash_key) {
        if (!strcmp(ht[hash_key].name, src)) {
            return &ht[hash_key];
        }
    }
    strcpy(ht[hash_key].name, src);
    return &ht[hash_key];
}

// parse substring
struct node* getnam(char* dests, char** destp) {
    char* cp = dests;
    while (isalpha(*cp)) ++cp;
    *cp = '\0';
    *destp = cp + 1;
    return get(dests);
}

void part1(char* input) {
    int offset = 0, read;
    char src[100];
    char dests[100];
    size_t i;
    while (sscanf(input + offset, "%s -> %[^\n]\n%n", src, dests, &read) == 2) {
        struct node* nodep;
        char* cp = dests;
        if (strlen(src+1) > 3) {
            src[4] = '\0';
        }
        nodep = get(src+1);
        nodep->ff = *src == '%';
        for (;*cp > 32; ) {
            struct node* outp = getnam(cp, &cp);
            // add node in both ways
            nodep->out[nodep->n_out++] = outp;
            outp->in[outp->n_in++] = nodep;
            while( isspace(*cp)) ++cp;
        }
        offset += read;
    }
    
    for (i=0; i < 256; ++i) {
        if (!ht[i].name[0]) continue;
        printf("%s\n", ht[i].name);
        printf("%d\n", ht[i].n_out);
    }
}

void part2(char* input) {

}

SOLUTION("./inputs/q20.txt")
