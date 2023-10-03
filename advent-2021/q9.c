#include "lib.h"

#define CAPACITY 10000

struct Point {
  int64_t row, col;
};

int is_point(int64_t row,int64_t col,PointVector *caves) {
    int64_t max_col = ((Vector *)caves->array[0])->size;
    int64_t current = ((Vector *)caves->array[row])->array[col];
    printf("is points is - %lld %lld\n",row,col);
    if (row >=0 && row < caves->size && col >= 0 && col < max_col && current != 9) {
        printf("current - %lld\n",current);
        return TRUE;
    }  
    return FALSE;
}

int64_t find_area(int64_t row,int64_t col,PointVector *caves, PointVector *visited) {
    PointVector *stack = init_ptr_vector(CAPACITY);
    int64_t max_col = ((Vector *)caves->array[0])->size, area = 0;
    int64_t i, directions[4][2] = {
        {0,1}, {0,-1}, {1,0},{-1,0}
    };
    struct Point *point = malloc(sizeof(struct Point));
    point->row = row;
    point->col = col;

    push_pv(stack, point);

    while (stack->size != 0) {
        struct Point *p = pop_pv(stack);
        
        if (((int64_t *)visited->array[p->row])[p->col] == TRUE) continue;
        ((int64_t *)visited->array[p->row])[p->col] = TRUE;
        area += 1;

        for (i=0; i < 4; i++ ){
            int64_t nrow = p->row + directions[i][0], ncol = p->col + directions[i][1]; 
            if (nrow >=0 && nrow < caves->size && ncol >= 0 && ncol < max_col ) {
                int64_t current = ((Vector *)caves->array[nrow])->array[ncol];
                struct Point *np = malloc(sizeof(struct Point));
                if (current == 9) continue;
                np->row = nrow;
                np->col = ncol;
                push_pv(stack, np);
            }  
        }
    }
    free_ptr_vector(stack);
    return area;
}

int compare(const void *a, const void *b) {
    const int64_t *x = (const int64_t *)a;
    const int64_t *y = (const int64_t *)b;
    return *y - *x; 
}

void solution(FILE *file) {
    PointVector *caves = init_ptr_vector(CAPACITY), *visited = init_ptr_vector(10000);
    Vector *areas = init_vector(1000);
    int64_t i, j, cols;
    char line[1000];
    int64_t low_points = 0, max_basin = 0;

    while (fscanf_s(file, "%s", line, sizeof(line)) != EOF) { 
        Vector *numbers = init_vector(CAPACITY);
        for (i = 0; line[i] != '\0'; i++) {
            push(numbers, line[i] - '0');
        }
        push_pv(caves, numbers);
    }

    for (i=0; i<caves->size; i++) {
        const Vector *numbers = caves->array[i];
        push_pv(visited, (int64_t *)calloc(numbers->size, sizeof(int64_t)));
        cols = numbers->size;
    }
    
    for (i = 0; i< caves->size; i++) {
        const Vector *numbers = caves->array[i];
        const int64_t *visit = visited->array[i];
        for (j = 0; j< numbers->size; j++) {
            int64_t current = numbers->array[j], current_area;
            if (visit[j] || current == 9) continue;
            current_area = find_area(i,j,caves, visited);
            push(areas, current_area);
        }
    }
    qsort(areas->array, areas->size, sizeof(int64_t), compare);
    printf("mul_max_areas: %lld\n", areas->array[0] * areas->array[1] * areas->array[2]);
    free_ptr_vector(visited);
    free_ptr_vector(caves);
}


AOC_MAIN_ONE("./inputs/q9.txt")
