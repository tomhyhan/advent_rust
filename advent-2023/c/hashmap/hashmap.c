#include "hashmap.h"

typedef struct node node_t;

typedef struct node {
    char* key;
    void* value;
    node_t* next;
} node_t;

typedef struct {
    node_t** bucket;
    size_t size;
} hashmap_t;

hashmap_t create_hashmap(size_t size) {
    int i;
    hashmap_t hashmap = {0};
    node_t** bucket = calloc(size, sizeof(node_t*));
    hashmap.bucket = bucket;
    hashmap.size = size;
    return hashmap;
}

void insert_hashmap(void) {
}


// simple hash function
size_t hash_65599(const char* string, size_t len) {
    size_t i;
    size_t hash;

    hash = 0;
    for (i=0; i < len; i++) {
        hash = 65599 * hash + string[i];
    }
    return hash ^ (hash >> 16);
}


int main(void) {
    hashmap_t hashmap = create_hashmap(1000);
    node_t* test = hashmap.bucket[0];
    printf("%d\n", test == NULL);
    
    return 0;
}
