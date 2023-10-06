#include "lib.h"

#define SIDE 10

typedef struct Coord {
    size_t row;
    size_t col;
} Coord;

PointVector *find_neighbors(size_t row, size_t col, int8_t cavern[][SIDE]) {   
    PointVector *neighbors = init_ptr_vector(10000); 
    int32_t i, j; 

    for (i=-1; i < 2; i++) {
        for (j = -1; j < 2 ; j++) {
            int32_t nrow = row + i, ncol = col + j;
            Coord *coord = (Coord *)malloc(sizeof(Coord));
            if (nrow < 0 || nrow >= SIDE || ncol < 0 || ncol >= SIDE) {
                continue;
            };
            if (cavern[nrow][ncol] == -1) {
                continue;
            };
            coord->row= nrow;
            coord->col= ncol;
            push_pv(neighbors, coord);
        }
    }
    return neighbors;    
}

void falshes(size_t row, size_t col, int8_t cavern[][SIDE]) {
    PointVector *stack = init_ptr_vector(10000), *neighbors;
    Coord *coord = (Coord *)malloc(sizeof(Coord));
    coord->row = row;
    coord->col = col;

    push_pv(stack, coord);

    /* falshes */
    while (stack->size != 0) {
        size_t i;
        Coord *current = pop_pv(stack);
 
        cavern[current->row][current->col] = -1;
        neighbors = find_neighbors(current->row,current->col, cavern);

        for (i=0;i<neighbors->size;i++) {
            Coord *neighbor = neighbors->array[i]; 
            cavern[neighbor->row][neighbor->col] += 1;
            if (cavern[neighbor->row][neighbor->col] == 10) {
                push_pv(stack,neighbor);
            }
        }
    }

    free(coord);
    free_ptr_vector(stack);
}

void solution(FILE *file) {
    int8_t cavern[SIDE][SIDE];
    size_t row, col, steps, i, j, flash = 0;

    for (row=0; row < SIDE; row++) {
        for (col=0; col < SIDE; col++) {
            fscanf_s(file, "%1d", &cavern[row][col]);
        }
    }

    for (steps = 0; steps < 10000; steps++) {
        size_t flash = 0;
        for (row=0; row < SIDE; row++) {
            for (col=0; col < SIDE; col++) {
                if (cavern[row][col] == -1) continue;
                cavern[row][col] += 1;
                if (cavern[row][col] > 9) {
                    falshes(row, col, cavern);
                }
            }
        }    
        for (i=0; i < SIDE; i++) {
            for (j=0; j < SIDE; j++) {
                if (cavern[i][j] == -1) {
                    cavern[i][j] = 0;
                    flash++;
                };
            }
        }
        if (flash == 100) {
            printf("steps: %lld\n", steps);
            break;
        }
    }

    for (i=0; i < SIDE; i++) {
        for (j=0; j < SIDE; j++) {
            printf("%d", cavern[i][j]);
        }
        printf("\n");
    }
    printf("flash - %lld\n", flash);
}


AOC_MAIN_ONE("./inputs/q11.txt")
