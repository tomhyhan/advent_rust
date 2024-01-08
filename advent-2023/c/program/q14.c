#include "call_lib.h"

enum { GRID_SIZE = 100 };  

typedef struct {
    int uw, nw; 
    size_t i;
} weight_t;

void fill_grid(char grid[][GRID_SIZE], char* input) {
    size_t row, col;

    for (row=0; row < GRID_SIZE; row++) {
        for (col=0; col < GRID_SIZE; col++) {
            grid[row][col] = *input++;
        }
        input++;
    }
}

void print_grid(char grid[][GRID_SIZE]) {
    size_t row, col;
    for (row=0; row < GRID_SIZE; row++) {
        for (col=0; col < GRID_SIZE; col++) {
            printf("%c", grid[row][col]);
        }
        puts("");
    }
    puts("");
}

void transpose(char grid[][GRID_SIZE]) { 
    size_t row, col;
    for (row=0; row < GRID_SIZE ; row++) {
        for (col = row + 1; col < GRID_SIZE ; col++) {
            char temp = grid[row][col];
            grid[row][col] = grid[col][row];
            grid[col][row] = temp;
        }
    }
}

void reverse(char grid[][GRID_SIZE]) { 
    size_t row, col;
    for (row=0; row < GRID_SIZE; row++) {
        for (col=0; col < GRID_SIZE / 2; col++) {
            char temp = grid[row][col];
            grid[row][col] = grid[row][GRID_SIZE - col - 1];
            grid[row][GRID_SIZE - col - 1] = temp;
        }
    }
}

void rotate_90(char grid[][GRID_SIZE]) {
    transpose(grid);
    reverse(grid);
}

bool grid_changes(char grid[][GRID_SIZE]) {
    size_t row, col;
    bool did_change = FALSE;

    for (row=0; row < GRID_SIZE; row++) {
        for (col=0; col < GRID_SIZE - 1; col++) {
            if (grid[row][col] == 'O' && grid[row][col+1] == '.') {
                char temp = grid[row][col];
                grid[row][col] = grid[row][col+1];
                grid[row][col+1] = temp; 
                did_change = TRUE;
            }
        }
    }
    return did_change;
}

int calc_weight(char grid[][GRID_SIZE]) {
    size_t row, col;
    bool weight = 0;
    for (row=0; row < GRID_SIZE; row++) {
        for (col=0; col < GRID_SIZE; col++) {
            if (grid[row][col] == 'O') {
                weight += GRID_SIZE - row;
            }
        }
    }
    return weight;
}

int calc_uniq_weight(char grid[][GRID_SIZE]) {
    size_t row, col;
    bool weight = 0;
    for (row=0; row < GRID_SIZE; row++) {
        for (col=0; col < GRID_SIZE; col++) {
            if (grid[row][col] == 'O') {
                weight += row * GRID_SIZE + col;
            }
        }
    }
    return weight;
}

void part1(char* input) {
    int weight = 0;    
    char grid[GRID_SIZE][GRID_SIZE];
    fill_grid(grid, input);
    rotate_90(grid);
    while (grid_changes(grid));

    rotate_90(grid);
    rotate_90(grid);
    rotate_90(grid);
    weight = calc_weight(grid);

    // printf("%d\n", weight);
}

void rotate(size_t num, char grid[][GRID_SIZE]) {
    size_t i;
    for (i=0; i < num; i++) {
        rotate_90(grid);
    }
}
bool weight_in_weights(int weight, weight_t* weights) {
    size_t i;
    for (i=0; i < list_len(weights); i++) {
        if (weight == weights[i].uw) {
            return i;
        }
    }
    return FALSE;
}

void part2(char* input) {
    char grid[GRID_SIZE][GRID_SIZE];
    weight_t* weights = create_list(weight_t); 
    int directions[4] = {1,2,3,0}; 
    size_t idx = 0, i = 0;
    weight_t fweight = {0};
    fill_grid(grid, input);    
    
    while (TRUE) {
        weight_t weight = {0};    
        size_t found_i;
        for (i=0; i < 4; i++) {
            rotate(directions[i], grid);
            while (grid_changes(grid));
            rotate(4 - directions[i], grid);
        }
        weight.uw = calc_uniq_weight(grid);
        weight.nw = calc_weight(grid);
        weight.i = idx;
        if ((found_i = weight_in_weights(weight.uw, weights))) {
            weight.i = found_i;
            push_list(weights, weight);
            break;
        }
        push_list(weights, weight);
        idx++;
    }
    fweight = weights[list_len(weights) - 1];

    // i start 0;
    printf("%d\n", weights[(1000000000 - fweight.i - 1) % (idx - fweight.i) + fweight.i].nw);
    destroy_list(weights);
}

SOLUTION("./inputs/q14.txt")
