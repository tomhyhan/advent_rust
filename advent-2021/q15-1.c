#include "lib.h"

#define WALL (100)

typedef struct Pos {
    int64_t risk;
    size_t row;
    size_t col;
} Pos;

size_t find_next_pos(PointVector *risks, int8_t visited[][WALL]) {
    int64_t current_risk = INT32_MAX;
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

void solution(FILE *file) {
    int32_t cave[WALL][WALL];
    int8_t visited[WALL][WALL] = {FALSE};
    PointVector *risks = init_ptr_vector(10000); 
    struct Pos p = {0};
    size_t i, j;
    int64_t directions[4][2] = {{1,0},{-1,0},{0,1},{0,-1}};
    
    for (i=0; i < WALL; i++) {
        for (j=0; j < WALL; j++) {
            fscanf_s(file, "%1d", &cave[i][j]);
            printf("%d",cave[i][j]);
        }
        printf("\n");
    }

    push_pv(risks, &p);

    while (TRUE) {
        size_t idx;
        Pos* p;
        idx = find_next_pos(risks, visited);
        if (idx == (size_t)-1) break;
        
        p = risks->array[idx];

        if (p->row == WALL-1 && p->col == WALL-1) {
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
            if (nrow < 0 || nrow >= WALL || ncol < 0 || ncol >= WALL) continue;
            np->row = nrow;
            np->col = ncol;
            np->risk = p->risk + cave[np->row][np->col];
            push_pv(risks, np);
        }
    }
    printf("%lld\n", ((Pos*)risks->array[1])->risk);
    printf("%lld\n", ((Pos*)risks->array[2])->risk);
}

AOC_MAIN_ONE("./inputs/q15.txt")                                    
