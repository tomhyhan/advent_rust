#include "dynamic.h"

// src: https://github.com/eignnx/dynarray

void *_create_list(size_t init_cap, size_t stride)
{
    size_t meta_size = LIST_FIELD * sizeof(size_t);
    size_t list_size = init_cap * stride;
    size_t* list = malloc(meta_size + list_size);
    list[CAPACITY] = init_cap;
    list[LENGTH] = 0;
    list[STRIDE] = stride;
    return (void *)(list + LIST_FIELD);
}

void _destroy_list(void* list) {
  free((void*)((size_t*)list - LIST_FIELD));
}

size_t _list_field_get(void* list, size_t field) {
    return ((size_t*)(list) - LIST_FIELD)[field];
}

void _list_field_set(void* list, size_t field, size_t value) {
    ((size_t*)(list) - LIST_FIELD)[field] = value;
}

void* _resize_list(void* list) {
    void* temp = _create_list(LIST_RESIZE_FACTOR * list_capacity(list), list_stride(list));
    memcpy(temp, list, list_len(list) * list_stride(list));
    _list_field_set(temp, LENGTH, list_len(list));
    _destroy_list(list);
    return temp;
}

void* _push_list(void* list, void* ptr) {
    if (list_len(list) >= list_capacity(list)) {
        list = _resize_list(list);
    }
    memcpy((char*)list + list_len(list) * list_stride(list), ptr, list_stride(list));
    _list_field_set(list, LENGTH, list_len(list) + 1);
    return list;
}

void _pop_list(void* list, void* value) {
    memcpy(value, (char*)list + (list_len(list) - 1) * list_stride(list), list_stride(list));
    _list_field_set(list, LENGTH, list_len(list) - 1);
}

