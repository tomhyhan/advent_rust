#include "call_lib.h"

#define GRID_SIZE 140

typedef struct {
    size_t row, col;
} galaxy_t;

char** create_grid(char* input) {
    char** grid = malloc(GRID_SIZE * sizeof(char*));
    char* rows = malloc(GRID_SIZE * GRID_SIZE * sizeof(char)); 
    size_t i;
    size_t row, col;

    for (i=0; i< GRID_SIZE; i++) {
        grid[i] = rows + i * GRID_SIZE;
    }

    for (row=0; row < GRID_SIZE; row++) {
        for (col=0; col < GRID_SIZE; col++) {
            grid[row][col] = *input++;
        }
        input++;
    }

    return grid;
}

bool is_row_all_zeros(char* row) {
    size_t i;
    for (i=0; i < GRID_SIZE; i++) {
        if (row[i] != '.') {
            return FALSE;
        }
    }
    return TRUE;
}

bool is_col_all_zeros(size_t col, char** grid) {
    size_t i;
    for (i=0; i < GRID_SIZE; i++) {
        if (grid[i][col] != '.') {
            return FALSE;
        }
    }
    return TRUE;
}

size_t manhattan_distance(int64_t x, int64_t y, int64_t x1, int64_t y1) {
    return llabs(x - x1) + llabs(y - y1);
 }

size_t find_comb_distance(char** grid, size_t* empty_rows, size_t* empty_cols) {
    galaxy_t* galaxies = create_list(galaxy_t);
    size_t row, col, cnt = 0;
    size_t total = 0;
    size_t factor = 1000000 - 1;
    for (row=0; row < GRID_SIZE; row++) {
        for (col=0; col < GRID_SIZE; col++) {
            if (grid[row][col] == '#') {
                galaxy_t g = {0};
                g.row = row;
                g.col = col;
                push_list(galaxies,g);
            }
        }
    }

    for (row=0; row < list_len(galaxies); row++) {
        for (col=row+1; col < list_len(galaxies); col++) {
            size_t i, row_inc = 0, col_inc = 0;
            size_t x = galaxies[row].col, y = galaxies[row].row, x1 = galaxies[col].col, y1 = galaxies[col].row; 

            for (i=0; i < list_len(empty_rows); i++) {
                if (empty_rows[i] > y &&  y1 > empty_rows[i]) {
                    row_inc += factor;
                }
            }

            for (i=0; i < list_len(empty_cols); i++) {
                size_t min_x = MIN(x, x1);
                size_t max_x = MAX(x, x1);
                if (empty_cols[i] > min_x &&  max_x > empty_cols[i]) {
                    col_inc += factor;
                }
            }
            
            total += manhattan_distance(x, y, x1, y1) + row_inc + col_inc;
            cnt++;
        }
    }
    printf("%lld\n", cnt);
    destroy_list(galaxies); 
    return total;
}

void part1(char* input) {
    char** grid = create_grid(input);
    size_t* empty_rows = create_list(size_t);
    size_t* empty_cols = create_list(size_t);
    size_t row, col;
    size_t total_distance;

    for (row=0; row < GRID_SIZE; row++) {
        if (is_row_all_zeros(grid[row])) {
            push_list(empty_rows, row);
        }
        if (is_col_all_zeros(row, grid)) {
            push_list(empty_cols, row);
        }
    }

    total_distance = find_comb_distance(grid, empty_rows, empty_cols);
    printf("%lld\n", total_distance);

    destroy_list(empty_rows);
    destroy_list(empty_cols);
}

void part2(char* input) {

}


SOLUTION("./inputs/q11.txt")
