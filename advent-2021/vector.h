/*review*/ 
#ifndef VECTOR
#define VECTOR

typedef struct {
    int *array;
    unsigned int size;
    unsigned int capacity;
} Vector;

Vector *init_vector(int init_capacity);
int get(Vector *vector, unsigned int idx);
int push(Vector *vector, int value);
int pop(Vector *vector);
int size(Vector *vector);
int capacity(Vector *vector);
void free_vector(Vector *vector);

#endif
