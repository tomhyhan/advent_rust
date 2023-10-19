#include "lib.h"

#define WALL 100
#define scale 5

typedef struct  {
    uint32_t risk;
    size_t coord;
} pos_t;

typedef struct {
    pos_t* pos;
    size_t size;
    size_t length;
} pt_heap_t;

static void push_heap(pt_heap_t* heap, size_t coord, uint32_t risk) {
    size_t parent, child;
    if (heap->length + 1 >= heap->size) {
        heap->size = heap->size? heap->size * 2: 4;
        heap->pos = (pos_t*)realloc(heap->pos, heap->size * sizeof(pos_t));
    }
    child = heap->length+1;
    parent = child / 2;

    while (child > 1 && heap->pos[parent].risk > risk) {
        heap->pos[child] = heap->pos[parent];
        child = parent;
        parent = parent / 2;
    }

    heap->pos[child].coord = coord;
    heap->pos[child].risk = risk;
    heap->length++;
}

static size_t pop_heap(pt_heap_t* heap) {
    size_t coord = heap->pos[1].coord;
    size_t parent;

    heap->pos[1] = heap->pos[heap->length];
    heap->length--;

    parent = 1;
    while (parent != heap->length+1) {
        size_t last = heap->length+1;
        size_t child = parent * 2;
        if (child <= heap->length && heap->pos[child].risk < heap->pos[last].risk) {
            last = child;
        }
        if (child + 1 <= heap->length && heap->pos[child+1].risk < heap->pos[last].risk) {
            last = child + 1;
        }
        heap->pos[parent] = heap->pos[last]; 
        parent = last;
    }
    
    return coord;
}

uint32_t get_next_risk(size_t row, size_t col, uint32_t cave[]) {
    uint32_t risk = (cave[(((row % WALL) * WALL) + (col % WALL))] + row / WALL + col / WALL) % 9 ;
    //  0-9 0-9 =? 0-49 0-49 / 10 0 / 20 10 
    //  300 50 = 6 * 10 = 60 
    return risk == 0? 9: risk; 
}

static void solution(FILE *file) {
    size_t row,col;
    uint32_t cave[(WALL * scale) * (WALL * scale)];
    // uint32_t risks[(WALL * scale) * (WALL * scale)];
    uint32_t* risks = (uint32_t*)calloc((WALL * scale) * (WALL * scale), sizeof(uint32_t));
    pt_heap_t* heap;
    int8_t dx, dy;

    for (row=0; row < WALL; row++) {
        for (col=0; col < WALL; col++) {
            fscanf_s(file, "%1d", &cave[row * WALL  + col]);
        }
    }    
    // memset(risks, 0, sizeof(risks));

    heap = (pt_heap_t*)calloc(1, sizeof(pt_heap_t));
    push_heap(heap, 0,0);

    while (heap->length > 0) {
        size_t coord = pop_heap(heap);
        uint32_t risk = risks[coord];
        if (coord == ((WALL * scale) * (WALL * scale)) -1){
            break;
        }
        for (dy=-1; dy <= 1; dy++) {
            for (dx=-1; dx <= 1; dx++) {
                size_t nrow, ncol;
                if (dy == 0 && dx ==0) continue;
                if (dy != 0 && dx !=0) continue;

                nrow = coord / (WALL * scale) + dy;
                ncol = coord % (WALL * scale) + dx;
                if (nrow >= 0 && nrow < (WALL * scale) && ncol >= 0 && ncol < (WALL * scale)) {
                    size_t ncoord = nrow * (WALL * scale) + ncol;
                    if (risks[ncoord] == 0 ) {
                        uint32_t nrisk = risk + get_next_risk(nrow, ncol, cave);
                        risks[ncoord] = nrisk; 
                        push_heap(heap, ncoord, nrisk);
                    }
                }
            }
        }
    }
    printf("%d", risks[(WALL*scale)*(WALL*scale)-1]);
} 

AOC_MAIN_ONE("./inputs/q15.txt")
