#ifndef HASHMAP
#define HASHMAP

typedef struct KeyValue{
    char *key;
    char *value;
    struct KeyValue *next;
} KeyValue;

typedef struct HashMap {
    KeyValue **map;
    size_t size; 
} HashMap;

HashMap *init_hashmap(void);
void insert(HashMap *hashmap, char *key, char *value);
char* find_val(HashMap *hashmap, char *key);
void freeHashMap(HashMap *hashmap);

#endif

