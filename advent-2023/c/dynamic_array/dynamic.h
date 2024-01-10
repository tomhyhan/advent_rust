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

#define INIT_CAP 100
#define LIST_RESIZE_FACTOR 2

void *_create_list(size_t init_cap, size_t stride);
size_t _list_field_get(void* list, size_t field);
void* _push_list(void* list, void* ptr);

void _list_field_set(void* list, size_t field, size_t value);
void _destroy_list(void* list);
void _pop_list(void* list, void* value);

#define create_list(type)                   \
    _create_list(INIT_CAP, sizeof(type)) 

#define push_list(list, p) list = _push_list(list, &p)              
#define pop_list(list, value)   \
    _pop_list(list, value)   

#define destroy_list(list) _destroy_list(list)


#define list_len(arr) _list_field_get(arr, LENGTH) 
#define list_capacity(arr) _list_field_get(arr, CAPACITY) 
#define list_stride(arr) _list_field_get(arr, STRIDE) 

#endif
