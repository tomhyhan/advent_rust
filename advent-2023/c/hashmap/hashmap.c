#include "hashmap.h"

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
