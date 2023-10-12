#include "lib.h"

typedef struct Point {
    int32_t x;
    int32_t y;
} Point;

int32_t** create_paper(int32_t max_row, int32_t max_col) {
    int32_t** paper = (int32_t** )calloc(max_row, sizeof(int32_t*));
    size_t i;

    for (i=0; i < max_row; i++) {
        paper[i] = (int32_t* )calloc(max_col, sizeof(int32_t));
    }

    return paper;
}

void fill_paper(PointVector* points, int32_t** paper) {
    size_t i;
    for (i=0; i < points->size; i++) {
        Point *pt = points->array[i];
        paper[pt->y][pt->x] = TRUE;
    }
}

void fold_y_axis(int32_t** paper, const int32_t y_fold, const int32_t max_row, const int32_t max_col) {
    size_t row,col, distance = 2;

    for (row=y_fold+1; row < max_row; row++) {
        size_t fold_row = row - distance;
        for (col=0; col < max_col; col++) {
            paper[fold_row][col] = paper[row][col] | paper[fold_row][col] ;
        }
        if (fold_row == 0) return;
        distance+=2;
    }
}

void fold_x_axis(int32_t** paper, const int32_t x_fold, const int32_t max_row, const int32_t max_col) {
    size_t row,col;

    for (row=0; row < max_row; row++) {
        size_t distance = 2;
        for (col=x_fold+1; col < max_col; col++) {
            size_t fold_col = col - distance; 
            // printf("col: %lld\n", col);
            // printf("fold_col: %lld\n", fold_col);
            paper[row][fold_col] = paper[row][col] | paper[row][fold_col];
            if (fold_col == 0) continue;
            distance+=2;
        }
    }
}

void generate_code(PointVector* points, const int32_t x_fold, const int32_t y_fold, const int32_t max_row, const int32_t max_col) {
    int32_t **paper = create_paper(max_row,max_col); 
    size_t i,j, stars = 0;
    fill_paper(points, paper);
    // assert(paper[6][10] == TRUE);
    // fold_y_axis(paper, y_fold, max_row, max_col);
    fold_x_axis(paper, x_fold, max_row, max_col);
    for (i=0; i < max_row; i++) {
        for (j=0; j < x_fold; j++) {
            if (paper[i][j] == TRUE) {
                stars++;
            }
        }
    }
    printf("%lld\n", stars);
}

void solution(FILE *file) {
    int32_t x,y,y_fold,x_fold,max_row=INT32_MIN,max_col=INT32_MIN;
    PointVector *points = init_ptr_vector(10000);
    while (fscanf_s(file, "%d,%d", &x, &y) != 0) {
        Point* pt = (Point*)malloc(sizeof(Point));
        pt->x = x;
        pt->y = y;
        push_pv(points, pt);
        max_col = MAX(max_col, x);
        max_row = MAX(max_row, y);
    };
    // fscanf_s(file, "%*[^y]y=%d",&y_fold);
    fscanf_s(file, "%*[^x]x=%d",&x_fold);
    printf("max_col: %d\n", max_col);
    generate_code(points, x_fold, y_fold, max_row+1, max_col+1);
    
}


AOC_MAIN_ONE("./inputs/q13.txt")
