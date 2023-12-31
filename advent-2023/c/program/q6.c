#include "call_lib.h"

typedef struct node node_t;

typedef struct node {
    int val;
    node_t* next;
} node_t;

void part1(char* input) {
    node_t head = {1};
    node_t next = {200};
    head.next = &next;
    printf("%d", head.next->val);
}

void part2(char* input) {

}

SOLUTION("./inputs/q6.txt")



