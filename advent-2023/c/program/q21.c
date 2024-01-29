#include "call_lib.h"
// [1] -> [2,3] -> [4,5,6,1] -> []

enum {GRID_SIZE = 131};

typedef struct {
    int row, col;
} pos_t;

typedef struct {
    bool grid[GRID_SIZE][GRID_SIZE];
    pos_t pts[10000];
    size_t size;
} plots_t;

typedef struct {
    char** grid; 
    pos_t start;
} grid_t;

grid_t create_grid(char* input) {
    grid_t grid_info = {0};
    char** grid = malloc(GRID_SIZE * sizeof(char*));
    char* rows = malloc(GRID_SIZE * GRID_SIZE * sizeof(char));
    size_t i,j;

    for (i=0; i<GRID_SIZE; i++) {
        grid[i] = rows + i * GRID_SIZE;
    }
    
    for (i=0; i < GRID_SIZE; i++) {
        for (j=0; j < GRID_SIZE; j++) {
            if (*input == 'S') {
                grid[i][j] = '.';
                grid_info.start.row = i;
                grid_info.start.col = j;
            } else {
                grid[i][j] = *input;
            }
            input++;
        }
        input++;
    }
    grid_info.grid = grid;
    return grid_info;
}

void print_grid(bool grid[][GRID_SIZE]) {
    size_t i,j;
    size_t cnt = 0; 
    for (i=0; i < GRID_SIZE; ++i) {
        for (j=0; j < GRID_SIZE; ++j) {
            if (grid[i][j]) {
                // printf("O");
                cnt++;
            } else {
                // printf(".");
            }
        }
        // puts("");
    }
    printf("%lld\n", cnt);
    // puts("");
}

void part1(char* input) {
    grid_t grid_info = create_grid(input);
    plots_t next_points = {0}; 
    plots_t current_points = {0}; 
    int directions[4][2] = {{0,1},{0,-1},{-1,0},{1,0}};
    int steps; 
    
    current_points.grid[grid_info.start.row][grid_info.start.row] = TRUE;
    current_points.pts[0].row = grid_info.start.row;
    current_points.pts[0].col = grid_info.start.col;
    current_points.size++;
    
    for (steps=0; steps <65; ++steps) {
        plots_t* current = steps % 2 == 0? &current_points: &next_points; 
        plots_t* next = steps % 2 == 0? &next_points: &current_points; 
        size_t i,j;

        for (i=0; i < current->size; i++) {
            for (j=0; j < 4; j++) {
                int nrow = current->pts[i].row + directions[j][0];
                int ncol = current->pts[i].col + directions[j][1];
                
                if (nrow < 0 || nrow >= GRID_SIZE || ncol < 0 || ncol >= GRID_SIZE || grid_info.grid[nrow][ncol] == '#') continue;

                if (next->grid[nrow][ncol]) continue;

                next->grid[nrow][ncol] = TRUE;
                next->pts[next->size].row = nrow;
                next->pts[next->size].col = ncol;
                next->size += 1;
            }
        }
        print_grid(next->grid);
    }
    free(grid_info.grid);
}

void part2(char *input) {}

SOLUTION("./inputs/q21.txt")
