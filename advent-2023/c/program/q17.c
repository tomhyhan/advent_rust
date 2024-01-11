#include "call_lib.h"

enum {GRID_SIZE = 13};

typedef struct {
    int row, col, heat; 
    int dy, dx;
} heat_t;

typedef struct {
    heat_t* items;
    size_t length, capacity;
} heap_t;

// create_heap
heap_t* create_heap(void) {
    return calloc(1, sizeof(heap_t));
}

// insert_heap
void insert_heap(heap_t* heap, heat_t heat) {
    size_t i,j;
    if (heap->length + 1 >= heap->capacity) {
        heap->capacity = heap->capacity? heap->capacity * 2: 4;
        heap->items = realloc(heap->items, heap->capacity * sizeof(heat_t));
    }
    
    i = heap->length + 1;
    j = i / 2;

    // move item down by 1 until we find an item with lower heat
    while (i > 1 && heap->items[j].heat > heat.heat) {
        heap->items[i] = heap->items[j];
        i = j;
        j = i / 2;
    }

    heap->items[i].row = heat.row;
    heap->items[i].col = heat.col;
    heap->items[i].dx = heat.dx;
    heap->items[i].dy = heat.dy;
    heap->items[i].heat = heat.heat;
    heap->length++;
}

// pop_heap
heat_t pop_heap(heap_t* heap) {
    heat_t removed = heap->items[1];
    size_t i;

    heap->items[1] = heap->items[heap->length];
    heap->length--;

    i = 1;
    while (i != heap->length + 1) {
        size_t k,j;
        k = heap->length + 1;
        j = i * 2;
        if (j <= heap->length && heap->items[j].heat < heap->items[k].heat) {
            k = j;
        }    
        if (j + 1 <= heap->length && heap->items[j + 1].heat < heap->items[k].heat) {
            k = j + 1;
        }    
        heap->items[i] = heap->items[k];
        i = k;
    }

    return removed;
}

int** create_grid(char* input) {
    int** grid = malloc(GRID_SIZE * sizeof(int*));
    int* rows = malloc(GRID_SIZE * GRID_SIZE * sizeof(int));
    size_t i,j;

    for (i=0; i < GRID_SIZE; i++) {
        grid[i] = rows + GRID_SIZE * i;
    }

    for (i=0; i < GRID_SIZE; i++) {
        for (j=0; j < GRID_SIZE; j++) {
            grid[i][j] = *input++ - '0';
        }
        input++;
    }

    // print
    // for (i=0; i < GRID_SIZE; i++) {
    //     for (j=0; j < GRID_SIZE; j++) {
    //         printf("%d", grid[i][j]);
    //     }
    //     puts("");
    // }
    return grid;
}


void part1(char* input) {
    int** grid = create_grid(input);
    heap_t* heap = create_heap();
    heat_t h;
    size_t i;

    for (i=0; i < 5; i++) {
        heat_t heat = {0};
        heat.heat = 10 - i;
        insert_heap(heap, heat);
    }
    // puts("asdf");
    printf("%d\n", heap->items[1].heat);
    printf("%d\n", heap->items[2].heat);
    printf("%d\n", heap->items[3].heat);
    printf("%d\n", heap->items[4].heat);
    printf("%d\n", heap->items[5].heat);
    
    h = pop_heap(heap);
    printf("heat %d\n", h.heat);
    h = pop_heap(heap);
    printf("heat %d\n", h.heat);
    h = pop_heap(heap);
    printf("heat %d\n", h.heat);

    free(heap);
    free(*grid);
    free(grid);
}

void part2(char* input) {

}

SOLUTION("./inputs/q17.txt")
