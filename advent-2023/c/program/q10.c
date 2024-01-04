#include "call_lib.h"

#define GRID_SIZE 140

typedef struct {
    size_t row, col;
    char tile;
} point_t;

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
bool within_boundary(size_t row, size_t col) {
    if (row >=0 && row < GRID_SIZE && col >=0 && col < GRID_SIZE) return TRUE;

    return FALSE;
}

void find_start(char** grid, size_t pos[][2]) {
    size_t row, col;
    size_t i, j;
    bool found = FALSE; 

    for (row=0; row < GRID_SIZE; row++) {
        for (col=0; col < GRID_SIZE; col++) {
            if (grid[row][col] == 'S') {
                pos[0][0] = row + 1; 
                pos[0][1] = col ; 
                pos[1][0] = row; 
                pos[1][1] = col; 
            }
        }
    }
}

void find_neighbors(char tile, size_t ns[][2]){
    switch (tile)
        {
        case '|':
            size_t ver[2][2] = {{-1,0},{1,0}};
            memcpy(ns, ver, sizeof(ver));
            return;
        case '-':
            size_t hor[2][2] = {{0,1},{0,-1}};
            memcpy(ns, hor, sizeof(hor));
            return;
        case 'L':
            size_t L[2][2] = {{-1,0},{0,1}};
            memcpy(ns, L, sizeof(L));
            return;
        case 'J':
            size_t J[2][2] = {{-1,0},{0,-1}};
            memcpy(ns, J, sizeof(J));
            return;
        case '7':
            size_t A[2][2] = {{1,0},{0,-1}};
            memcpy(ns, A, sizeof(A));
            return;
        case 'F':
            size_t F[2][2] = {{1,0},{0,1}};
            memcpy(ns, F, sizeof(F));
            return;
        default:
            printf("cannot reach here!");
            exit(EXIT_FAILURE);
            break;
        }
}

void find_next(char** grid, size_t pos[][2]) {
    size_t row = pos[0][0], col = pos[0][1]; 
    size_t vrow = pos[1][0], vcol = pos[1][1]; 
    size_t ns[2][2] = {0};
    size_t i;
    
    find_neighbors(grid[row][col], ns);

    for (i=0; i<2; i++) {
        size_t nrow = row + ns[i][0];
        size_t ncol = col + ns[i][1];
        if (nrow == vrow && ncol == vcol) {
            continue;
        }
        pos[0][0] = nrow; 
        pos[0][1] = ncol;
        
        pos[1][0] = row; 
        pos[1][1] = col;
    }
}

void part1(char* input) {
    size_t row, col;
    char** grid = create_grid(input);
    size_t pos[2][2] = {0};
    size_t steps = 0;
\
    find_start(grid, pos);

    do {
        find_next(grid, pos);
        steps++;
    } while (grid[pos[0][0]][pos[0][1]] != 'S'); 

    printf("steps: %lld\n", (steps + 1));

    free(*grid);
    free(grid);
}

void part2(char* input) {
    size_t row, col;
    char** grid = create_grid(input);
    size_t pos[2][2] = {0};
    point_t* pts = create_list(point_t); 
    point_t pt = {0}; 
    int64_t area = 0; 

    find_start(grid, pos);

    pt.row = pos[1][0];
    pt.col = pos[1][1];
    pt.tile = grid[pos[1][0]][pos[1][1]];
    push_list(pts, pt);
    
    do {
        char tile = grid[pos[0][0]][pos[0][1]];
        if (!(tile == '|' || tile == '-')) {
            pt.row = pos[0][0];
            pt.col = pos[0][1];
            pt.tile = grid[pos[0][0]][pos[0][1]];
            push_list(pts, pt);
        }
        find_next(grid, pos);
    } while (grid[pos[0][0]][pos[0][1]] != 'S'); 
    
    for (row=0; row < list_len(pts) ; row++) {
        size_t i = row % list_len(pts);
        size_t i2 = (row + 1) % list_len(pts);
        area += pts[i].row * pts[i2].col - pts[i].col * pts[i2].row;
    }

    area /= 2;
    
    // i ??
    // area = i + (46 / 2) - 1
    printf("area %lld\n", area);
    printf("area: %lld\n", area - ( 13572 / 2 )  + 1);

    destroy_list(pts);
    free(*grid);
    free(grid);
}


// for (row=0; row < GRID_SIZE; row++) {
//     for (col=0; col < GRID_SIZE; col++) {
//         printf("%c", grid[row][col]);
//     }
//     puts("");
// }

SOLUTION("./inputs/q10.txt")
