#include "call_lib.h"

enum {GRID_SIZE = 110};


//  (-1,0) (1,0) (0,-1) (0,1)
typedef struct {
    int dy, dx;
    int row, col;
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

size_t print_grid1(char grid[][GRID_SIZE]) {
    size_t row, col;
    size_t energy = 0;
    for (row=0; row < GRID_SIZE; row++) {
        for (col=0; col < GRID_SIZE; col++) {
            // printf("%c", grid[row][col]);
            if (grid[row][col] == '#') {
                energy++;
            }
        }
        // puts("");
    }
    // puts("");
    return energy;
}

// bool dir_in_energized(dir_t dir, char energized[][GRID_SIZE]) {
//     size_t i;
//     size_t len = list_len(energized);
    
//     for (i=0; i < len; i++) {
//         dir_t dir_in = energized[i];
//         if (dir.col == dir_in.col && dir.row == dir_in.row && dir.dx == dir_in.dx && dir.dy == dir_in.dy) {
//             return TRUE;
//         }
//     }
//     // printf("here end?\n");
//     return FALSE;
// }

bool is_not_border(int row, int col) {
    if (row < 0 || row >= GRID_SIZE || col < 0 || col >= GRID_SIZE) return TRUE; 
    return FALSE;
}

void light_bounces(dir_t dir, char energized[][GRID_SIZE], char** grid) {
    dir_t new_dir = {0};
    char tile;

    new_dir.col = dir.col + dir.dx;
    new_dir.row = dir.row + dir.dy;

    if (is_not_border(new_dir.row, new_dir.col)) return;

    tile = grid[new_dir.row][new_dir.col];
    if (energized[new_dir.row][new_dir.col] == '#' && (tile == '|' || tile == '-')) {   
        return;
    }

    energized[new_dir.row][new_dir.col] = '#';
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
    char energized[GRID_SIZE][GRID_SIZE];

    memset(energized, '.', sizeof(energized));
    dir.col = -1;
    dir.dx = 1;
    light_bounces(dir, energized, grid);
    
    printf("len %d\n", 3);
    print_grid1(energized);
    
    
    free(*grid);
    free(grid);
}

void part2(char* input) {
    char** grid = create_grid(input);
    size_t max_energy = 0;
    size_t row, col, i;
    int directions[4][2] = {{0,1},{0,-1},{1,0},{-1,0}};

    for (row=0; row < GRID_SIZE; row++) {
        for (col=0; col < GRID_SIZE; col++) {
            for (i=0; i < 4; i++) {
                dir_t dir = {0};
                char energized[GRID_SIZE][GRID_SIZE];
                memset(energized, '.', sizeof(energized));
                dir.row = row + directions[i][0];
                dir.col = col + directions[i][1];
                dir.dy = -directions[i][0];
                dir.dx = -directions[i][1];

                if (is_not_border(dir.row, dir.col)) {
                    light_bounces(dir, energized, grid);
                    max_energy = MAX(max_energy, print_grid1(energized));
                }

            }        
        }
    } 
    printf("%lld\n", max_energy);
    free(*grid);
    free(grid);
}

SOLUTION("./inputs/q16.txt")
