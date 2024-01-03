#ifndef LINKED_LIST
#define LINKED_LIST

#include <string.h>
#include <stdlib.h>
#include <stdio.h>

typedef struct node node_t;
void _insert(node_t** phead, char* key, void* data);

#define insert_node(phead, key, value) (_insert(phead, key, (void*)value ))

#endif
