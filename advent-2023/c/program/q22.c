#include "call_lib.h"

typedef struct {
    int x1, x2, y1, y2, z1, z2;
} brick_t;

typedef struct {
    int by[10];
    size_t size;
} support_t;

enum {SIZE = 1370};

brick_t* parse(char* input) {
    brick_t* bricks = create_list(brick_t);
    brick_t b = {0};
    int offset = 0, read;
    while(sscanf(input + offset, "%d,%d,%d~%d,%d,%d\n%n", &b.x1, &b.y1, &b.z1, &b.x2, &b.y2, &b.z2, &read) == 6) {
        push_list(bricks, b);
        offset += read;
    }
    return bricks;
}

int cmp_func(const void* a, const void* b) {
    const brick_t* ba = (const brick_t*)a;
    const brick_t* bb = (const brick_t*)b;
    return (ba->z1) - (bb->z1);
}

bool exist(int height, int* supports) {
    size_t i;
    for (i=0; i < list_len(supports); ++i) {
        if (height == supports[i]) return TRUE;
    }
    return FALSE;
}

void part1(char* input) {
    brick_t* bricks = parse(input); 
    size_t i,j ;
    int heights[20][20][2] = {0,};
    int bads[SIZE] = {0};
    support_t sby[SIZE] = {0};
    int cannot = 0, falls = 0;
    qsort(bricks, list_len(bricks), sizeof(brick_t), cmp_func);
    
    for (i=0; i < list_len(bricks); i++) {
        int max_h = -1, fall;
        brick_t b = bricks[i];
        size_t x,y,cnt=0;
        int* supports = create_list(int);
        int first;

        for (x=b.x1; x <= b.x2; x++) {
            for (y=b.y1; y <= b.y2; y++) {
                if (heights[x][y][0] + 1 > max_h) {
                    max_h = heights[x][y][0] + 1;
                    destroy_list(supports);
                    supports = create_list(int);
                    push_list(supports, heights[x][y][1]);
                } else if (heights[x][y][0] + 1 == max_h) { 
                    if (exist(heights[x][y][1], supports)) {
                        continue;
                    }
                    push_list(supports, heights[x][y][1]);
                }   
            }
        }

        for (x=0; x < list_len(supports); x++) {
            if (supports[x]-1 != -1) {
                sby[supports[x]-1].by[sby[supports[x]-1].size++] = i;
            }
        }

        if (list_len(supports) == 1) {
            bads[supports[0]] = TRUE;
        }

        fall = b.z1 - max_h;
        if (fall > 0) {
            b.z1 -= fall;
            b.z2 -= fall;
        }

        for (x=b.x1; x <= b.x2; x++) {
            for (y=b.y1; y <= b.y2; y++) {
                heights[x][y][0] = b.z2;
                heights[x][y][1] = i+1;
            }
        }
        destroy_list(supports);
    }

    for (i=0; i < SIZE; i++) {
        if (bads[i]) cannot++;
    }

    printf("part1: %d\n", list_len(bricks) - cannot + 1);

    for (i=0; i < list_len(bricks); ++i) {
        int* depends = calloc(list_len(bricks), sizeof(int));
        int* queue = create_list(int);
        int cnt = -1;
        size_t k;
        for (j=0; j < list_len(bricks); ++j) {
            for (k=0; k < sby[j].size; ++k) {
                depends[sby[j].by[k]]++;
            }
        }        
        push_list(queue, i);
        while (list_len(queue) > 0) {
            int pos;
            pop_list(queue, &pos);
            cnt++;
            for (k=0; k < sby[pos].size; ++k) {
                depends[sby[pos].by[k]]--;
                if (depends[sby[pos].by[k]] == 0) {
                    push_list(queue, sby[pos].by[k]);
                }
            }    
        }

        falls += cnt;
        free(depends);
        destroy_list(queue);
    }
    
    printf("part2: %d\n", falls);
    destroy_list(bricks);
}

void part2(char* input) {

}

SOLUTION("./inputs/q22.txt")
