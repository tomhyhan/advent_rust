#include "lib.h"

#define WALL (100)
#define NEW_WALL (WALL*5)

typedef struct Pos {
    int64_t risk;
    size_t row;
    size_t col;
} Pos;

size_t find_next_pos(PointVector *risks, int8_t** visited) {
    int64_t current_risk = INT64_MAX;
    size_t idx = -1;
    size_t i;
    
    for (i=0; i < risks->size; i++) {
        struct Pos* p = (Pos *)risks->array[i]; 
        if (visited[p->row][p->col]) continue;
        if (p->risk < current_risk) {
            current_risk =  p->risk;
            idx = i;
        }
    }
    return idx;
}

void fill_new_map(int32_t cave[][NEW_WALL]) {
    size_t repeat_c, repeat_r, row, col;    

    for (repeat_c=1; repeat_c < 5;repeat_c++) {
        size_t offset = repeat_c*WALL;
        for (row=0; row < WALL; row++) {
            for (col=0; col < WALL; col++) {
                cave[row][offset+col] = (cave[row][(offset-WALL) + col] + 1) % 10 == 0 ? 1: (cave[row][(offset-WALL) + col] + 1) % 10;
            }
        }
    }

    for (repeat_r=1; repeat_r < 5;repeat_r++) {
        for (repeat_c=0; repeat_c < 5;repeat_c++) {
            size_t offset_r = repeat_r * WALL;
            size_t offset_c = repeat_c * WALL;
            for (row=0; row < WALL; row++) {
                for (col=0; col < WALL; col++) {
                    cave[offset_r+row][offset_c+col] = (cave[offset_r-WALL+row][offset_c+col] + 1) % 10 == 0? 1: (cave[offset_r-WALL+row][offset_c+col] + 1) % 10; 
                }
            }
        }
    }
}

void solution(FILE *file) {
    int32_t cave[NEW_WALL][NEW_WALL];
    // int64_t visited[NEW_WALL][NEW_WALL] = {0};
    int8_t **visited = calloc(NEW_WALL, sizeof(int8_t *));
    PointVector *risks = init_ptr_vector(10000); 
    struct Pos p = {0};
    size_t i, j;
    int64_t directions[4][2] = {{1,0},{-1,0},{0,1},{0,-1}};
    
    for (i = 0; i < NEW_WALL; i++) {
        visited[i] = calloc(NEW_WALL, sizeof(int8_t));
    }

    for (i=0; i < WALL; i++) {
        for (j=0; j < WALL; j++) {
            fscanf_s(file, "%1d", &cave[i][j]);
        }
    }
    fill_new_map(cave);
    push_pv(risks, &p);

    while (TRUE) {
        size_t idx;
        Pos* p;
        idx = find_next_pos(risks, visited);
        if (idx == (size_t)-1) break;
        
        p = risks->array[idx];

        if (p->row == NEW_WALL-1 && p->col == NEW_WALL-1) {
            printf("p: %lld %lld\n", p->row, p->col);
            printf("risk: %lld\n", p->risk);
            break;
        }

        if (visited[p->row][p->col]) continue;
        visited[p->row][p->col] = TRUE;

        for (i=0; i < 4; i++) {
            int64_t nrow = p->row + directions[i][0];  
            int64_t ncol = p->col + directions[i][1];  
            Pos *np = (Pos*)malloc(sizeof(Pos));
            if (nrow < 0 || nrow >= NEW_WALL || ncol < 0 || ncol >= NEW_WALL) continue;
            np->row = nrow;
            np->col = ncol;
            np->risk = p->risk + cave[np->row][np->col];
            push_pv(risks, np);
        }
    }
    printf("%lld\n", ((Pos*)risks->array[1])->risk);
    printf("%lld\n", ((Pos*)risks->array[2])->risk);

    for (i = 0; i < NEW_WALL; i++) {
        free(visited[i]);
    }
    free(visited);
    free_ptr_vector(risks);
}

AOC_MAIN_ONE("./inputs/q15.txt")                                    
