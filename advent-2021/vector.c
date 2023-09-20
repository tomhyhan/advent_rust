#include "vector.h"
#include <stdio.h>
#include <stdlib.h>

Vector *init_vector(int init_capacity) {
    Vector *vector = (Vector *)malloc(sizeof(Vector));

    vector->array = (int *)malloc(sizeof(int) * init_capacity);
    if (vector->array == NULL) {
        free(vector);
        return NULL;
    }

    vector->size = 0;
    vector->capacity = init_capacity;

    return vector;
}

int get(Vector *vector, unsigned int idx) {
    if (idx < 0 || idx >= vector->size) {
        printf("index error");
        return -1;
    }

    return vector->array[idx];
}

int push(Vector *vector, int value) {
    if (vector->size == vector->capacity) {
        vector->capacity *= 2;
        vector->array = (int *)realloc(vector->array, sizeof(int) * vector->capacity);
        if (vector->array == NULL) {
            return -1;
        }
    }

    vector->array[vector->size] = (value);
    vector->size++;
    return 1;
}


int pop(Vector *vector) {
    int value;

    if (vector->size == 0) {
        printf("cannot pop from an empty vector");
        return -1;
    }

    vector->size--;
    value = vector->array[vector->size];
    return value;
}

int size(Vector *vector) {
    return vector->size;
}


int capacity(Vector *vector) {
    return vector->capacity;
}

void print_vector(Vector *vector) {
    size_t i;
    for (i=0;i<vector->size;i++) {
        printf("%d\n", vector->array[i]);
    }
}

void free_vector(Vector *vector) {
    free(vector->array);
    free(vector);
}
