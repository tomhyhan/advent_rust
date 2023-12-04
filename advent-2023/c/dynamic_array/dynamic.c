#include "dynamic.h"

// src: https://github.com/eignnx/dynarray

void *_create_list(size_t init_cap, size_t stride)
{
    size_t meta_size = init_cap * sizeof(size_t);
    size_t list_size = init_cap * stride;
    size_t *list = malloc(meta_size + list_size);
    list[CAPACITY] = init_cap;
    list[LENGTH] = 0;
    list[STRIDE] = stride;
    return (void *)(list + LIST_FIELD);
}

size_t _list_field_get(void* list, size_t field) {
    return ((size_t*)(list) - LIST_FIELD)[field];
}

// void _list_field_set(void* list, size_t field, size_t value) {
//     return 
// }

void _resize_list(void* list) {
    // void* temp = _create_list( LIST_RESIZE_FACTOR * )
}

void _push_list() {

}

int main()
{
    char *chrs = create_list(char);
    return 0;
}
