#include "call_lib.h"

#define GRID_SIZE 140

typedef struct {
    int row, col;
} pt_t;

typedef struct {
    pt_t pt;
    int* nums;
} symbol_t;

char** create_grid(char* input) {
    int row, col, pos = 0;
    char** grid = malloc(GRID_SIZE * sizeof(char*)); 
    char* rows = calloc(GRID_SIZE * GRID_SIZE, sizeof(char));
    
    for (row=0; row < GRID_SIZE; row++) {
        grid[row] = rows + row * GRID_SIZE;
    }

    for (row=0; row < GRID_SIZE; row++) {
        for (col=0; col < GRID_SIZE; col++) {
            grid[row][col] = input[pos++];
        }
        pos++;
    }

    return grid;
}

void part1(char* input) {
    int row, col;
    char** grid = create_grid(input);
    char* digit_end;
    int digit;
    int gear = 0;
    symbol_t* symbols = create_list(symbol_t);

    for (row=0; row < GRID_SIZE; row++) {
        for (col=0; col < GRID_SIZE; col++) {
            if (grid[row][col] != '.' && !('0' <= grid[row][col] && grid[row][col] <= '9')) {
                symbol_t symbol;
                symbol.nums = create_list(int);
                symbol.pt.row = row;
                symbol.pt.col = col;
                push_list(symbols,symbol);
            }
        }
    }

    for (row=0; row < GRID_SIZE; row++) {
        col = 0;
        while (col < GRID_SIZE) {
            if (isdigit(grid[row][col])) {
                int start, end, i, j, k;
                int digit;
                char num[5];

                start = col - 1;
                i = 0;
                while (col < GRID_SIZE && isdigit(grid[row][col]) ) {
                    num[i] = grid[row][col];
                    col++;
                    i++;
                } 
                num[i] = '\0';
                end = col;
                for (i=row-1; i <= row+1; i++) {
                    for (j=start; j <= end; j++) {
                        for (k=0; k < list_len(symbols); k++) {
                            if (symbols[k].pt.row == i && symbols[k].pt.col == j) {
                                int intnum = atoi(num);
                                push_list(symbols[k].nums, intnum);
                            }
                        }
                    } 
                }
            }
            col++;
        }
    }

    for (row=0; row < list_len(symbols); row++) {
        for (col=0; col < list_len(symbols[row].nums); col++) {
            gear += symbols[row].nums[col];
        }
    }
    printf("part1 gear: %d\n", gear);

    gear = 0;
    for (row=0; row < list_len(symbols); row++) {
        if (list_len(symbols[row].nums) == 2) {
            int gear_ratio = 1;
            for (col=0; col < list_len(symbols[row].nums); col++) {
                gear_ratio *= symbols[row].nums[col];
            }
            gear += gear_ratio;
        }
    }
    
    printf("part2 gear ratio: %d\n", gear);
    
    for (row=0; row < list_len(symbols); row++) {
        destroy_list(symbols[row].nums);
    }
    destroy_list(symbols);
    free(*grid);
    free(grid);
}

void part2(char* input) {
    
}

SOLUTION("./inputs/q3.txt")
