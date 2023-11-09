#include "lib.h"

typedef struct {
    int64_t x1, x2, y1, y2 ,z1, z2;
    char status[4]; 
} box_t;

static PointVector* parse_input(FILE* file) {
    PointVector* boxes = init_ptr_vector(1000);
    int64_t x1, x2, y1, y2 ,z1, z2;
    char status[4];
    
    while (fscanf_s(file, "%3s x=%lld..%lld,y=%lld..%lld,z=%lld..%lld\n",status, sizeof(status), &x1, &x2, &y1, &y2 ,&z1, &z2) != EOF) {
        box_t* b = malloc(sizeof(box_t));
        strcpy_s(b->status, sizeof(status), status);
        b->x1 = x1;
        b->x2 = x2;
        b->y1 = y1;
        b->y2 = y2;
        b->z1 = z1;
        b->z2 = z2;
        push_pv(boxes, b);
    }
    return boxes;
}

bool does_intersect(box_t* b, box_t* f) {
    return !(b->x2 < f->x1 || b->x1 > f->x2 || b->y2 < f->y1 || b->y1 > f->y2 || b->z2 < f->z1 || b->z1 > f->z2);
}

box_t* create_intersect(box_t* b, box_t* f) {
    box_t* nc = malloc(sizeof(box_t));
    nc->x1 = MAX(b->x1, f->x1);
    nc->x2 = MIN(b->x2, f->x2);
    nc->y1 = MAX(b->y1, f->y1);
    nc->y2 = MIN(b->y2, f->y2);
    nc->z1 = MAX(b->z1, f->z1);
    nc->z2 = MIN(b->z2, f->z2);
    strcpy_s(nc->status,sizeof(nc->status), strcmp(f->status, "on") == 0? "off": "on");
    return nc;
}

int64_t cube(box_t* b) {
    int64_t r = (b->x2 - b->x1 + 1) * (b->y2 - b->y1 + 1) * (b->z2 - b->z1 + 1); 
    return r;
}

void count_ons(PointVector* boxes) {
    PointVector* fragments = init_ptr_vector(1000);
    size_t i, j;
    int64_t total = 0;

    for (i=0; i < boxes->size; i++) {
        box_t* b = boxes->array[i];
        size_t f_len = fragments->size;
        for (j=0; j < f_len; j++) {
            box_t* f = fragments->array[j];
            if (does_intersect(b, f)) {
                box_t* nc = create_intersect(b,f);
                push_pv(fragments, nc);
            }
        }

        if (strcmp(b->status,"on") == 0 ){
            push_pv(fragments, b);
        }
    }
    for (i=0; i < fragments->size; i++) {
        box_t* f = fragments->array[i];
        total += (strcmp(f->status, "on") == 0 ? 1 : -1) *  cube(f);
    }
    printf("total final: %lld\n", total);
    free_ptr_vector(fragments);
}

void solution(FILE* file) {
    PointVector* boxes = parse_input(file); 
    count_ons(boxes);
    free_ptr_vector(boxes);
}

AOC_MAIN_ONE("./inputs/q22.txt")
