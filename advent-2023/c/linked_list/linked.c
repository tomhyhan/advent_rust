#include "linked.h"

typedef struct node node_t;

typedef struct node {
    int val;
    node_t* next;
} node_t;

void insert(node_t** phead, int val) {
    node_t* new_node = malloc(sizeof(node_t)); 
    new_node->val = val;

    new_node->next = *phead;
    *phead = new_node;
}

void insert_sort(node_t** phead, int val) {
    node_t** pp = phead;

    node_t* new_node = malloc(sizeof(node_t));
    new_node->val = val;

    while(*pp != NULL) {
        if ((*pp)->val > val ) {
            break;
        }

        pp = &(*pp)->next;
    }

    new_node->next = *pp;
    *pp = new_node;
}

void remove_node(node_t** phead, int val) {
    
    node_t** pp = phead; 
    while (*pp) {
        if ((*pp)->val == val) {
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
    insert_sort(&head, 1);
    insert_sort(&head, 2);
    insert_sort(&head, 3);
    remove_node(&head, 2);
    printf("%d\n", head->val);
    printf("%d\n", head->next->val);
}
