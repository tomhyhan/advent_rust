#include <stdio.h>
#include <stdlib.h>
#include "lib.h"
#include "ptr_vector.h"

struct Coord {
    int x1;
    int y1; 
    int x2; 
    int y2; 
};

int find_max(int max, int p1, int p2 );
int draw_lines(PointVector *ptr_vector, int max_x, int max_y);
void swap(int *p1, int *p2);
void draw(struct Coord *ptr, int** surface);
int **create_surface(int max_x, int max_y);

int main(void) {
    FILE *file = read_file_data("./inputs/q5.txt");
    char line[100];
    int x1, y1, x2, y2, max_x=0, max_y=0;
    PointVector *ptr_vector = init_ptr_vector(100);

    while (fgets(line, sizeof(line), file) != NULL) {
        if (sscanf_s(line, "%d,%d -> %d,%d", &x1, &y1, &x2, &y2) == 4) {
            struct Coord *coord = (struct Coord *)malloc(sizeof(struct Coord));
            coord -> x1 = x1;
            coord -> y1 = y1;
            coord -> x2 = x2;
            coord -> y2 = y2;
            push(ptr_vector, coord);
            /* printf("%d %d %d %d\n", x1,y1,x2,y2); */
            max_x = find_max(max_x, x1, x2);
            max_y = find_max(max_y, y1, y2);
        }
    } 

    draw_lines(ptr_vector, max_x+1, max_y+1);

    free_ptr_vector(ptr_vector);
    fclose(file);
    return 1;
}

int **create_surface(int max_x, int max_y) {
    int i,j;
    int **surface = (int **)malloc(sizeof(int* ) *max_y);
    
    for (i=0; i < max_y; i++) {
        surface[i] = (int *)malloc(sizeof(int) *max_x);
    }
    
    for (i=0;i<max_y;i++) {
        for (j=0;j<max_x;j++) {
            surface[i][j] = 0;
        }
    }
    return surface;
}

int draw_lines(PointVector *ptr_vector, int max_x, int max_y) {
    int **surface = create_surface(max_x, max_y);
    int  i,j,overlaps = 0;
    struct Coord *ptr;
    printf("%d %d\n", max_x, max_y);

    for (i=0; i< ptr_vector->size; i++) {
        ptr = ptr_vector->array[i];
        draw(ptr, surface);
    }

    for (i=0;i<max_y;i++) {
        for (j=0;j<max_x;j++) {
            if (surface[i][j]> 1) {
                overlaps +=1;
            }
        }
    }

    printf("overlaps - %d", overlaps);

    for (i=0; i < max_y; i++) {
        free(surface[i]); 
    }
    free(surface);
}

void draw(struct Coord *ptr, int** surface) {
    int x1=ptr->x1;
    int x2=ptr->x2;
    int y1=ptr->y1; 
    int y2=ptr->y2;
    
    if (x1 == x2) {
        if (y1 > y2) {
            swap(&y1,&y2);
        }
        while (y1 <= y2) {
            surface[y1++][x1] += 1;
        }
    } else if (y1 == y2) {
        if (x1 > x2) {
            swap(&x1,&x2);
        }
        while (x1 <= x2) {
            surface[y1][x1++] += 1;
        }
    }
}

void swap(int *p1, int *p2) {
    int temp = *p1;
    *p1 = *p2;
    *p2 = temp;
}

int find_max(int max, int p1, int p2 ) {
    int max_pt = max;
    int values[3];
    int i;

    values[0] = p1;
    values[1] = p2;
    for (i=0; i <2; i++) {
        if (max_pt < values[i]) {
            max_pt = values[i];
        }
    }
    return max_pt;
}
