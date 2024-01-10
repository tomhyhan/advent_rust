#include "call_lib.h"

enum {GRID_SIZE = 10};


//  (-1,0) (1,0) (0,-1) (0,1)
typedef struct {
    int row, col;
    int dy, dx;
} dir_t;

char** create_grid(char* input) {
    size_t row, col;
    char** grid = malloc(GRID_SIZE * sizeof(char*));
    char* rows = malloc(GRID_SIZE * GRID_SIZE * sizeof(char));
    
    for (row=0; row < GRID_SIZE; row++) {
        grid[row] = rows + GRID_SIZE * row;    
    }

    for (row=0; row < GRID_SIZE; row++) {
        for (col=0; col < GRID_SIZE; col++) {
            grid[row][col] = *input++;
        }
        input++;
    }
    return grid;
}

void print_grid(char** grid) {
    size_t row, col;
    for (row=0; row < GRID_SIZE; row++) {
        for (col=0; col < GRID_SIZE; col++) {
            printf("%c", grid[row][col]);
        }
        puts("");
    }
    puts("");
}

void print_grid1(char grid[][GRID_SIZE]) {
    size_t row, col;
    for (row=0; row < GRID_SIZE; row++) {
        for (col=0; col < GRID_SIZE; col++) {
            printf("%c", grid[row][col]);
        }
        puts("");
    }
    puts("");
}

bool dir_in_energized(dir_t dir, dir_t* energized) {
    size_t i;
    
    // printf("here?\n");
    // printf("%lld\n", list_len(energized));
    for (i=0; i < list_len(energized); i++) {
        dir_t dir_in = energized[i];
        // printf("%d\n", i);
        // printf("%d %d %d %d\n", dir.row, dir.col, dir.dy, dir.dx);
        // printf("%d %d %d %d\n", dir_in.row, dir_in.col, dir_in.dy, dir_in.dx);
        // puts("");
        // if (memcmp(&dir, &dir_in, sizeof(dir_t)) == 0) {
        //     return TRUE;
        // }
        // printf("%lld\n", i);
        if (dir.col == dir_in.col && dir.row == dir_in.row && dir.dx == dir_in.dx && dir.dy == dir_in.dy) {
            return TRUE;
        }
    }
    // printf("here end?\n");
    return FALSE;
}

void light_bounces(dir_t dir, dir_t* energized, char** grid) {
    dir_t new_dir = {0};
    char tile;

    if (dir_in_energized(dir, energized)) return;
    printf("%d %d %d %d\n", dir.row, dir.col, dir.dy, dir.dx);
    push_list(energized, dir);
    
    new_dir.col = dir.col + dir.dx;
    new_dir.row = dir.row + dir.dy;

    if (new_dir.row < 0 || new_dir.row >= GRID_SIZE || new_dir.col < 0 || new_dir.col >= GRID_SIZE) return;


    tile = grid[new_dir.row][new_dir.col];
    // printf("%c %d %d %d %d\n", tile, new_dir.row, new_dir.col, dir.dy, dir.dx);
    
    switch (tile)
    {
    case '|':
        if (dir.dy == 1 || dir.dy == -1) goto DEFAULT;
        new_dir.dy = dir.dx;
        new_dir.dx = dir.dy;
        light_bounces(new_dir, energized, grid);
        new_dir.dy = -dir.dx;
        new_dir.dx = dir.dy;
        light_bounces(new_dir, energized, grid);
        return;
    case '-':
        if (dir.dx == 1 || dir.dx == -1) goto DEFAULT;
        new_dir.dy = dir.dx;
        new_dir.dx = dir.dy;
        light_bounces(new_dir, energized, grid);
        new_dir.dy = dir.dx;
        new_dir.dx = -dir.dy;
        light_bounces(new_dir, energized, grid);
        return;
    case '\\':
    // 0 1 <=> -1 0 || 0 -1 <=> 1 0
        new_dir.dy = dir.dx;
        new_dir.dx = dir.dy;
        light_bounces(new_dir, energized, grid);
        return;
    case '/':
    // 0 1 <=> -1 0 || 0 -1 <=>  0
        new_dir.dy = -dir.dx;
        new_dir.dx = -dir.dy;
        light_bounces(new_dir, energized, grid);
        return;
    DEFAULT:
        default:
            new_dir.dx = dir.dx;
            new_dir.dy = dir.dy;
            light_bounces(new_dir, energized, grid);
            return;
    }
}

void part1(char* input) {
    dir_t dir = {0};
    char** grid = create_grid(input);
    dir_t* energized = create_list(dir_t);
    // size_t i;
    // for (i=0; i < 100; i++) {
    //     dir_t t = {0};
    //     push_list(energized, t);
    // }
    // printf("%lld\n", list_len(energized));
    
    dir.col = -1;
    dir.dx = 1;
    
    light_bounces(dir, energized, grid);
    
    printf("len %d\n", list_len(energized));
    
    destroy_list(energized);
    free(*grid);
    free(grid);
}

void part2(char* input) {

}

SOLUTION("./inputs/q16.txt")
