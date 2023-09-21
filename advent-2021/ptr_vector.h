#ifndef PTR_VECTOR
#define PTR_VECTOR

typedef struct {
    void **array;
    unsigned int size;
    unsigned int capacity;
} PointVector;

PointVector *init_ptr_vector(int init_capacity);
int push(PointVector *ptr_vector, void* ptr);
void *pop(PointVector *ptr_vector);
unsigned int size(PointVector *ptr_vector);
unsigned int capacity(PointVector *ptr_vector);
void free_ptr_vector(PointVector *ptr_vector);

#endif
