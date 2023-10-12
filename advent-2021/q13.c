#include "lib.h"

typedef struct Point {
    int32_t x;
    int32_t y;
} Point;

int32_t** create_paper(int32_t max_row, int32_t max_col) {
    int32_t** paper = (int32_t** )calloc(max_row, sizeof(int32_t*));
    int32_t i;

    for (i=0; i < max_row; i++) {
        paper[i] = (int32_t* )calloc(max_col, sizeof(int32_t));
    }

    return paper;
}

void fill_paper(PointVector* points, int32_t** paper) {
    int32_t i;
    for (i=0; i < points->size; i++) {
        Point *pt = points->array[i];
        paper[pt->y][pt->x] = TRUE;
    }
}

void fold_y_axis(int32_t** paper, const int32_t y_fold, const int32_t max_row, const int32_t max_col) {
    int32_t row,col, distance = 2;

    for (row=y_fold+1; row < max_row; row++) {
        int32_t fold_row = row - distance;
        for (col=0; col < max_col; col++) {
            paper[fold_row][col] = paper[row][col] | paper[fold_row][col] ;
        }
        if (fold_row == 0) return;
        distance+=2;
    }
}

void fold_x_axis(int32_t** paper, const int32_t x_fold, const int32_t max_row, const int32_t max_col) {
    int32_t row,col;

    for (row=0; row < max_row; row++) {
        int32_t distance = 2;
        for (col=x_fold+1; col < max_col; col++) {
            int32_t fold_col = col - distance; 
            paper[row][fold_col] = paper[row][col] | paper[row][fold_col];
            if (fold_col == 0) continue;
            distance+=2;
        }
    }
}

int32_t ** create_and_fill_paper(PointVector* points, const int32_t max_row, const int32_t max_col) {
    int32_t **paper = create_paper(max_row,max_col); 
    fill_paper(points, paper);
    return paper;
}

void fold_paper(int32_t** paper, int32_t fold, bool fold_x, const int32_t max_row, const int32_t max_col) {
    if (fold_x) {
        fold_x_axis(paper, fold, max_row, max_col);
    } else {
        fold_y_axis(paper, fold, max_row, max_col);
    }
}

void solution(FILE *file) {
    int32_t x,y,fold,max_row=INT32_MIN,max_col=INT32_MIN;
    int32_t i,j;
    int32_t ** paper;
    char x_y;
    PointVector *points = init_ptr_vector(10000);
    while (fscanf_s(file, "%d,%d", &x, &y) != 0) {
        Point* pt = (Point*)malloc(sizeof(Point));
        pt->x = x;
        pt->y = y;
        push_pv(points, pt);
        max_col = MAX(max_col, x);
        max_row = MAX(max_row, y);
    };
    max_row++;
    max_col++;

    paper = create_and_fill_paper(points,max_row, max_col);

    while (fscanf_s(file, "fold along %c=%d\n",&x_y, sizeof(x_y),&fold) != EOF) {
        if (x_y == 'x') {
            fold_paper(paper, fold, TRUE, max_row, max_col);
        } else {
            fold_paper(paper, fold, FALSE, max_row, max_col);
        }
    }
    for (i=0; i < 7; i++) {
        for (j=0; j < 50; j++) {
            if (paper[i][j] == TRUE) {
                printf("#");
            }else {
                printf(".");
            }
        }
        printf("\n");
    }
}


AOC_MAIN_ONE("./inputs/q13.txt")

