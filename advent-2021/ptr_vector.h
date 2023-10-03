#ifndef PTR_VECTOR
#define PTR_VECTOR

typedef struct {
    void **array;
    unsigned int size;
    unsigned int capacity;
} PointVector;

PointVector *init_ptr_vector(int init_capacity);
int push_pv(PointVector *ptr_vector, void* ptr);
void *pop_pv(PointVector *ptr_vector);
void free_ptr_vector(PointVector *ptr_vector);

#endif
