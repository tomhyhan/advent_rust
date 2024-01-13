#include "call_lib.h"

typedef struct node node_t;

enum {BUCKET_SIZE = 256};

typedef struct node {
    char key[10]; 
    int val;
    node_t* next;
} node_t;

int64_t hash(int64_t num) {
    return (num * 17) % 256;
}

char** parse(char* input) {
    char** sequence = create_list(char*);

    char* token;
    char delim[] = ",";

    token = strtok(input, delim);        
    while (token) {
        push_list(sequence, token);
        token = strtok(NULL, delim);        
    }

    return sequence;
}

void part1(char* input) {
    char** sequence = parse(input);
    size_t i;
    int64_t result = 0;

    for (i=0;i<list_len(sequence);i++) {
        size_t j;
        int64_t current = 0;
        for (j=0;j < strlen(sequence[i]); j++) {
            current = hash(current + sequence[i][j]);
        }
        result += current;
    }

    printf("%lld\n", result);

    destroy_list(sequence);
}

void init_bucket(node_t** bucket) {
    size_t i;
    for (i = 0; i < BUCKET_SIZE; i++) {
        bucket[i] = NULL;
    }
}

void insert(node_t** phead, char* key, int val) {
    node_t** current = phead;
    node_t* new_node = malloc(sizeof(node_t));
    memcpy(new_node->key, key, sizeof(new_node->key));
    new_node->val = val;
    // h -> [] -> NULL
         // h -> NULL
    while (*current != NULL) {
        if (strcmp((*current)->key, key) == 0 ) {
            (*current)->val = val;
            free(new_node);
            return;
        }
        current = &(*current)->next;
    }
    
    new_node->next = *current; 
    *current = new_node;
}

void remove_node(node_t** phead, char* key) {
    node_t** current = phead;

    while(*current != NULL) {
        if (strcmp((*current)->key, key) == 0) {
            node_t* removed = *current;
            *current = (*current)->next;
            free(removed);
            break;
        }
        current = &(*current)->next;
    }
}

void part2(char* input) {
    char** sequence = parse(input);
    size_t i;
    size_t result = 0;
    node_t* bucket[BUCKET_SIZE];
    init_bucket(bucket);
    
    for (i=0;i<list_len(sequence);i++) {
        size_t j;
        char key[10];
        char symbol;
        int val = -1;
        int64_t hash_key = 0;

        if (sscanf(sequence[i], "%[^=-]%c%d", key, &symbol, &val) == 3) {} else sscanf(sequence[i], "%[^=-]%c", key, &symbol);

        for (j=0;j < strlen(key); j++) {
            hash_key = hash(hash_key + key[j]);
        }

        if (symbol == '-') {
            remove_node(&bucket[hash_key], key);
        } else if (symbol == '=') {
            insert(&bucket[hash_key], key ,val);
        }
    }

    for (i = 0; i < BUCKET_SIZE; i++) {
        node_t* node_p = bucket[i];
        size_t slot = 1;
        while (node_p) {
            result += (i + 1) * slot * node_p->val;
            slot++;
            node_p = node_p->next;  
        }
    }
    printf("%lld\n", result);

    for (i = 0; i < BUCKET_SIZE; i++) {
        node_t** node_p = &bucket[i];
        while (*node_p) {
            node_t* removed = *node_p;
            node_p = &(*node_p)->next;  
            free(removed);
        }
    }
    destroy_list(sequence);
}

SOLUTION("./inputs/q15.txt")
