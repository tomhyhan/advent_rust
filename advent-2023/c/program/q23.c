#include "call_lib.h"

// enum { GRID_SIZE = 141 };
enum { GRID_SIZE = 23 };
static const int DIRECTIONS[4][2] = {{-1,0},{0,1},{1,0},{0,-1}};
static int MAX_STEPS = 0;

char** create_grid(char* input) {
    char** grid = malloc(GRID_SIZE * sizeof(char*));
    char* rows = malloc(GRID_SIZE * GRID_SIZE * sizeof(char));
    size_t i, j;
    
    for (i=0; i < GRID_SIZE; i++) {
        grid[i] = rows + i * GRID_SIZE;
    }

    for (i=0; i < GRID_SIZE; i++) {
        for (j=0; j < GRID_SIZE; j++) {
            grid[i][j] = *input++;
        }
        input++;
    }

    return grid;
}

int find_direction(char tile) {
    size_t i;
    char pdir[] = "^>v<";
    for(i=0; i < 4; i++) {
        if (tile == pdir[i]) return i;
    }
    return -1;
}

void traverse_grid(int row, int col, char** grid, bool visited[][GRID_SIZE], int steps) {
    char tile; size_t i; int pdir;
    if (row < 0 || row >= GRID_SIZE || col < 0 || col >= GRID_SIZE) return;
    
    tile = grid[row][col];

    if (tile == '#' || visited[row][col]) return;

    if (row == GRID_SIZE -1 && col == GRID_SIZE - 2) {
        printf("steps: %d\n", steps);
        MAX_STEPS = MAX(MAX_STEPS, steps);
        return;
    }

    // part 1
    // pdir = find_direction(tile);
    // if (pdir != -1) {
    //     int nrow = row + DIRECTIONS[pdir][0];
    //     int ncol = col + DIRECTIONS[pdir][1];
    //     visited[row][col] = TRUE;
    //     traverse_grid(nrow, ncol, grid, visited, steps + 1);
    //     visited[row][col] = FALSE;
    //     return;
    // } 

    visited[row][col] = TRUE;
    for (i=0; i< 4; i++) {
        int nrow = row + DIRECTIONS[i][0];
        int ncol = col + DIRECTIONS[i][1];
        traverse_grid(nrow, ncol, grid, visited, steps + 1);
    }
    visited[row][col] = FALSE;
}

void part1(char *input) {
    char** grid = create_grid(input);
    bool visited[GRID_SIZE][GRID_SIZE] = {0};
    traverse_grid(0, 1, grid, visited, 0);
    printf("max: %d\n", MAX_STEPS);
}

void part2(char *input) {}

SOLUTION("./inputs/q23.txt")
