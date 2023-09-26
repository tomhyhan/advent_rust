/*review*/ 
#ifndef VECTOR
#define VECTOR
#include <stdint.h>

typedef struct {
    int64_t *array;
    size_t size;
    size_t capacity;
} Vector;

Vector *init_vector(int init_capacity);
int get(Vector *vector, size_t idx);
int push(Vector *vector, int value);
int pop(Vector *vector);
int size(Vector *vector);
int capacity(Vector *vector);
void free_vector(Vector *vector);
void print_vector(Vector *vector);

#endif
