#include "ptr_vector.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

PointVector *init_ptr_vector(int init_capacity) {
    PointVector *point_vector = (PointVector *)malloc(sizeof(PointVector));
    
    point_vector->array = (void **)malloc(sizeof(void *) * init_capacity);
    if (point_vector == NULL) {
        printf("ptr vector fail to init");
        free(point_vector);
        return NULL;
    }

    point_vector->size = 0;
    point_vector->capacity = init_capacity;
    return point_vector;
}

int push_pv(PointVector *ptr_vector, void* ptr) {
    if (ptr_vector->size == ptr_vector->capacity) {
        ptr_vector->capacity *= 2;
        ptr_vector->array = (void **)realloc(ptr_vector->array, sizeof(void *) * ptr_vector->capacity);
        if (ptr_vector == NULL) {
            return -1;
        }
    }

    ptr_vector->array[ptr_vector->size] = ptr;
    ptr_vector->size++;
    return 1;
}

void *pop_pv(PointVector *ptr_vector) {
    void* current;

    if (ptr_vector->size ==0) {
        printf("pop from an empty list");
        return NULL;
    }
    ptr_vector->size--;
    current = ptr_vector->array[ptr_vector->size];
    return current;
}

void remove_pv(PointVector* ptr_vector, size_t i) {
    memmove(ptr_vector->array + i, ptr_vector->array + i + 1, (ptr_vector->size - i - 1) * sizeof(ptr_vector->array[0]));
    ptr_vector->size--;
}

void insert_pv(PointVector* ptr_vector, size_t i) {
    memmove(ptr_vector->array + i + 1, ptr_vector->array + i, (ptr_vector->size - i) * sizeof(ptr_vector->array[0]));
    ptr_vector->size++;
}

unsigned int size_pv(PointVector *ptr_vector) {
    return ptr_vector->size;
}

unsigned int capacity_pv(PointVector *ptr_vector) {
    return ptr_vector->capacity;
}

void free_ptr_vector(PointVector *ptr_vector) {
    unsigned int i;
    for (i = 0; i < ptr_vector->size; i++) {
        free(ptr_vector->array[i]);
    }
    free(ptr_vector->array);
    free(ptr_vector);
}

