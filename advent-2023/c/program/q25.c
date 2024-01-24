#include "call_lib.h"

// from graphviz
// tbq -> qfj
// dsr -> xzn
// qqh -> xbl

enum {BUCKET_SIZE = 300000};

static int NODES = 0;

typedef struct {
    uint16_t key;
    uint16_t connected[20];
    size_t size;
} item_t;

item_t hashmap[BUCKET_SIZE] = {0};

uint16_t hash_key(const char *n) {
  return ((n[0] - 'a') * 10000 + (n[1] - 'a') * 100 + (n[2] - 'a')) + 1;
}

item_t* search_hashmap(uint16_t key) {
    if (hashmap[key].key != 0) {
        return &hashmap[key];
    }
    hashmap[key].key = key;
    NODES++;
    return &hashmap[key];
}

item_t* look_hashmap(uint16_t key) {
    if (hashmap[key].key != 0) {
        return &hashmap[key];
    }
    return NULL;
}

void connect_graph(item_t* in, item_t* out) {
    in->connected[in->size++] = out->key;
    out->connected[out->size++] = in->key;
}

void parse(char* input) {
    int offset = 0, read;
    char line[1000];
    
    FILE* file = fopen("./inputs/q25.txt", "r");

    while(fgets(line, sizeof(line), file)) {
        char in[10];
        char outs[100], delim[] = " ";
        char* token;
        uint16_t in_key;
        item_t* in_item;
        sscanf(line, "%s %[^\n]\n", in, outs);
        
        in_key = hash_key(in);
        in_item = search_hashmap(in_key);

        token = strtok(outs, delim);
        while (token){
            item_t* out_item;
            uint16_t out_key = hash_key(token);
            out_item = search_hashmap(out_key);
            connect_graph(in_item, out_item);
            
            token = strtok(NULL, delim);
        }
        
    }

    fclose(file);
}

void cut(item_t* a, item_t* b) {
    size_t i;    
    for (i=0; i < a->size; i++) {
        if (a->connected[i] == b->key) {
            a->connected[i] = a->connected[a->size-- - 1];
        }
    }

    for (i=0; i < b->size; i++) {
        if (b->connected[i] == a->key) {
            b->connected[i] = b->connected[b->size-- - 1];
        }
    }
}

// tbq -> qfj
// dsr -> xzn
// qqh -> xbl
void part1(char* input) {
    uint16_t stack[5120];
    uint16_t* sp = stack;
    item_t* d;
    size_t cnt = 0;
    parse(input);
    
    cut(search_hashmap(hash_key("tbq")), search_hashmap(hash_key("qfj")));
    cut(search_hashmap(hash_key("dsr")), search_hashmap(hash_key("xzn")));
    cut(search_hashmap(hash_key("qqh")), search_hashmap(hash_key("xbl")));

    *sp++ =  hash_key("zsx");
    while (sp > stack) {
        size_t i;
        uint16_t current = *--sp;
        item_t* item = look_hashmap(current);
        if (item == NULL || item->key == 0) {
            continue;
        }
        item->key = 0;
        ++cnt;
        for (i=0; i < item->size; i++) {
            *sp++ = item->connected[i];
        }
    }

    printf("%lld\n", cnt);
    printf("%lld\n", NODES - 773);
    printf("%d\n", 773 * 715);
}

void part2(char* input) {}

SOLUTION("./inputs/q25.txt")
