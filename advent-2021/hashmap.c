#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "hashmap.h"

#define CAPACITY 1000

HashMap *init_hashmap(void){
    HashMap *hashMap = (HashMap *)malloc(sizeof(HashMap)); 
    size_t i;
    hashMap->size = CAPACITY;
    hashMap->map = (KeyValue **)malloc(sizeof(KeyValue *) * CAPACITY);
    for (i=0;i<CAPACITY;i++) {
        hashMap->map[i] = NULL;
    }
    return hashMap;
}   

int hash(char *key) {
    size_t hash = 0;
    while (*key) {
        hash = (hash + *key) % CAPACITY;
        key++;
    }
    return hash;
}

void insert(HashMap* hashmap, char* key, void* value) {
    size_t hashed_key = hash(key);
    KeyValue *new_kv = (KeyValue*)malloc(sizeof(KeyValue)); KeyValue **current;
    new_kv->key = _strdup(key);
    new_kv->value = value;
    new_kv->next = NULL;

    current = &(hashmap->map[hashed_key]);
    if (*current == NULL) {
        *current = new_kv;
    } else {
        while (*current != NULL) {
        if (strcmp((*current)->key, key) == 0) {
            (*current)->value = value;  
            free(new_kv->key);
            free(new_kv->value);
            free(new_kv);
            return;
        }
            current = &((*current)->next);
        }
        *current = new_kv;
    }
}

void* find_val(HashMap *hashmap, char *key) {
    size_t hashed_key = hash(key);
    KeyValue *current = hashmap->map[hashed_key];
    while (current != NULL) {
        if (strcmp(current->key, key)== 0) {
            return current->value;
        }
        current = current->next;
    }
    return NULL;
}

void freeHashMap(HashMap *hashmap) {
    size_t i;
    for ( i = 0; i < hashmap->size; i++) {
        KeyValue *current = hashmap->map[i];
        while (current != NULL) {
            KeyValue *temp = current;
            current = current->next;
            free(temp->key);
            free(temp->value);
            free(temp);
        }
    }
    free(hashmap->map);
    free(hashmap);
}

