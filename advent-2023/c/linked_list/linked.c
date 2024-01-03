#include "linked.h"

typedef struct node node_t;

typedef struct node {
    char* key;
    void* value;
    node_t* next;
} node_t;

void _insert(node_t** phead, char* key, void* data) {
    node_t* new_node = malloc(sizeof(node_t)); 
    new_node->key = key;
    new_node->value = data;

    new_node->next = *phead;
    *phead = new_node;
}

// void insert_sort(node_t** phead, int val) {
//     node_t** pp = phead;

//     node_t* new_node = malloc(sizeof(node_t));
//     new_node->val = val;

//     while(*pp != NULL) {
//         if ((*pp)->val > val ) {
//             break;
//         }

//         pp = &(*pp)->next;
//     }

//     new_node->next = *pp;
//     *pp = new_node;
// }

void remove_node(node_t** phead, char* key) {
    
    node_t** pp = phead; 
    while (*pp) {
        if ((*pp)->key == key) {
            node_t* removed = *pp;
            *pp = (*pp)->next;
            free(removed);
            break;
        }
        pp = &(*pp)->next;
    }
}

int main(void) {
    node_t* head = NULL;
    // insert_node(&head, "testf", "rasdf");
    int x = 1234;
    int* ip = &x;
    insert_node(&head, "testf", ip);
    // insert(&head, 2);
    // insert(&head, 3);
    // remove_node(&head, 2);
    printf("%s\n", head->key);
    printf("%d\n", *(int*)head->value);
    // printf("%d\n", head->next->val);
}
