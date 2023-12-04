#ifndef DYNAMIC_ARRAY
#define DYNAMIC_ARRAY

#include <string.h>
#include <stdlib.h>

enum
{
    CAPACITY,
    LENGTH,
    STRIDE,
    LIST_FIELD
};

#define INIT_CAP 1
#define LIST_RESIZE_FACTOR 2

void *_create_list(size_t init_cap, size_t stride);

#define create_list(type)                \
    _create_list(INIT_CAP, sizeof(type)) \

#endif
