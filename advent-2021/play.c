#include <stdio.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
    void **array;
    unsigned int size;
    unsigned int capacity;
} Vector;

Vector *init_vector(int init_capacity) {
    Vector *vector = (Vector *)malloc(sizeof(Vector));

    vector->array = (void **)malloc(sizeof(void *) * init_capacity);
    if (vector->array == NULL) {
        free(vector);
        return NULL;
    }

    vector->size = 0;
    vector->capacity = init_capacity;

    return vector;
}

int push(Vector *vector, void *value) {
    if (vector->size == vector->capacity) {
        vector->capacity *= 2;
        vector->array = (void **)realloc(vector->array, sizeof(void *) * vector->capacity);
        if (vector->array == NULL) {
            return -1;
        }
    }

    vector->array[vector->size] = value;
    vector->size++;
    return 1;
}

void *pop(Vector *vector) {
    int* value;

    if (vector->size == 0) {
        printf("cannot pop from an empty vector");
        return NULL;
    }

    vector->size--;
    value = vector->array[vector->size];
    return value;
}

void freeDynamicArray(Vector *vector) {
    int i;
    for (i = 0; i < vector->size; i++) {
        free(vector->array[i]);
    }
    free(vector->array);
    free(vector);
}

int main(void) {
    Vector *vector = init_vector(10);
    int num = 10, a = 5;
    int* p = &num; 
    int* p1 = &a; 
    void* r; 
    Vector *subvec = init_vector(5);
    int m[2] = {1,2000};
    char c = 'a';
    push(vector, p);
    push(vector, p1);
    push(vector, m);
    push(vector, subvec);

    r = pop(vector);
    ((Vector *)r)->size = 5;
    printf("%d\n", ((Vector *)r)->size);

    freeDynamicArray(vector);
    return 1;
}
