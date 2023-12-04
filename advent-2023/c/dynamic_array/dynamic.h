#ifndef DYNAMIC_ARRAY
#define DYNAMIC_ARRAY

#include <string.h>
#include <stdlib.h>

enum
{
    CAPACITY,
    LENGTH,
    STRIDE,
    LIST
};

#define INIT_CAP 1

void *_create_list(size_t init_cap, size_t stride);

#define create_list(type)                \
    _create_list(INIT_CAP, sizeof(type)) \

#endif
