#include "call_lib.h"

enum {GRID_SIZE = 141};
// enum {GRID_SIZE = 13};

typedef struct {
    int row, col, heat, dir; 
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
    if (heap->length+1 >= heap->capacity) {
        heap->capacity = heap->capacity? heap->capacity * 2: 4;
        heap->items = realloc(heap->items, heap->capacity * sizeof(heat_t));
    }

    i = heap->length + 1;
    j = i / 2;

    while (i > 1 && heap->items[j].heat > heat.heat) {
        heap->items[i] = heap->items[j];
        i = j;
        j = i / 2;
    }

    // memcpy(&heap->items[i], &heat, sizeof(heat));
    heap->items[i].row = heat.row;
    heap->items[i].col = heat.col;
    heap->items[i].dir = heat.dir;
    heap->items[i].heat = heat.heat;
    heap->length++;
}

// pop_heap
heat_t pop_heap(heap_t* heap) {
    size_t i;
    heat_t removed = heap->items[1];

    heap->items[1] = heap->items[heap->length];
    heap->length--;

    i = 1;
    while (i != heap->length + 1) {
        size_t j,k;
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
    bool visited[GRID_SIZE][GRID_SIZE][4] = {0};
    // n e s w
    int directions[4][2] = {{-1, 0}, {0, 1}, {1, 0}, {0, -1}};
    int lr[2] = {1,3};

    heat_t start_h = {0};
    start_h.dir = 1;
    insert_heap(heap, start_h);
    start_h.dir = 2;
    insert_heap(heap, start_h);

    while (heap->length) {
        heat_t h = pop_heap(heap);        
        size_t i,j;

        if (h.row == GRID_SIZE - 1 && h.col == GRID_SIZE - 1) {
            printf("%d\n", h.heat);
            break;
        }

        if (visited[h.row][h.col][h.dir]) continue;    
        visited[h.row][h.col][h.dir] = TRUE;
        // printf("%d %d\n", h.row, h.col);
        for (i=0; i < 2; i++) {
            int new_dir = (h.dir + lr[i]) % 4;
            int new_heat = h.heat;
            // 1 - 4 in part 1
            for (j=1; j < 4; j++) {
                int row = h.row + directions[h.dir][0] * j; 
                int col = h.col + directions[h.dir][1] * j; 
                
                if (row < 0 || row >= GRID_SIZE || col < 0 || col >= GRID_SIZE) break;

                new_heat += grid[row][col]; 
            }
            
            for (j=4; j < 11; j++) {
                heat_t new_h = {0};
                new_h.dir = new_dir;
                new_h.row = h.row + directions[h.dir][0] * j;
                new_h.col = h.col + directions[h.dir][1] * j;

                if (new_h.row < 0 || new_h.row >= GRID_SIZE || new_h.col < 0 || new_h.col >= GRID_SIZE) break;
                
                new_heat += grid[new_h.row][new_h.col];
                new_h.heat = new_heat; 
                insert_heap(heap, new_h);
            }
        }
    }
    free(heap);
    free(*grid);
    free(grid);
}

void part2(char* input) {

}

SOLUTION("./inputs/q17.txt")

// for (i=0; i < 5; i++) {
//     heat_t heat = {0};
//     heat.heat = 10 - i;
//     insert_heap(heap, heat);
// }
// printf("%d\n", heap->items[1].heat);
// printf("%d\n", heap->items[2].heat);
// printf("%d\n", heap->items[3].heat);
// printf("%d\n", heap->items[4].heat);
// printf("%d\n", heap->items[5].heat);

// h = pop_heap(heap);
// printf("heat %d\n", h.heat);
// h = pop_heap(heap);
// printf("heat %d\n", h.heat);
// h = pop_heap(heap);
// printf("heat %d\n", h.heat);

// insert_heap
// void insert_heap(heap_t* heap, heat_t heat) {
//     size_t i,j;
//     if (heap->length + 1 >= heap->capacity) {
//         heap->capacity = heap->capacity? heap->capacity * 2: 4;
//         heap->items = realloc(heap->items, heap->capacity * sizeof(heat_t));
//     }
    
//     i = heap->length + 1;
//     j = i / 2;

//     // j parent i child 
//     // move parent down 
//     while (i > 1 && heap->items[j].heat > heat.heat) {
//         heap->items[i] = heap->items[j];
//         i = j;
//         j = i / 2;
//     }

//     heap->items[i].row = heat.row;
//     heap->items[i].col = heat.col;
//     heap->items[i].dx = heat.dx;
//     heap->items[i].dy = heat.dy;
//     heap->items[i].heat = heat.heat;
//     heap->length++;
// }

// // pop_heap
// heat_t pop_heap(heap_t* heap) {
//     heat_t removed = heap->items[1];
//     size_t i;

//     heap->items[1] = heap->items[heap->length];
//     heap->length--;

//     i = 1;
//     // i parent j children
//     // move child up
//     while (i != heap->length + 1) {
//         size_t k,j;
//         k = heap->length + 1;
//         j = i * 2;
//         if (j <= heap->length && heap->items[j].heat < heap->items[k].heat) {
//             k = j;
//         }    
//         if (j + 1 <= heap->length && heap->items[j + 1].heat < heap->items[k].heat) {
//             k = j + 1;
//         }    
//         heap->items[i] = heap->items[k];
//         i = k;
//     }

//     return removed;
// }

