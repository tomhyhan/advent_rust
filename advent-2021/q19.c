#include "lib.h"

#define SIZE 25 

typedef struct {
    int32_t x;
    int32_t y;
    int32_t z;
} point_t;

typedef struct {
    point_t points[100];
    size_t i;
} scan_t;


PointVector* parse_scanners(FILE* file) {
    PointVector* scanners = init_ptr_vector(1000);
    char line[1000];
    scan_t* scanner = (scan_t*)calloc(1,sizeof(scan_t));

    while (fscanf_s(file, "%[^\n]\n", line, sizeof(line)) != EOF ) {
        point_t pt = {0};

        if (line[1] == '-') {
            if (scanner->i > 0 ) {
                printf("%s\n", line);
                push_pv(scanners, scanner);
                scanner = (scan_t*)calloc(1,sizeof(scan_t));
            }
            continue;
        }
        sscanf_s(line, "%d,%d,%d", &pt.x,&pt.y,&pt.z);
        // printf("%d %d %d\n",  pt.x, pt.y, pt.z);
        scanner->points[scanner->i++] = pt; 
    }
    push_pv(scanners, &scanner);
    
    return scanners;
}

void swap(int32_t p[], size_t i, size_t j) {
    int32_t temp = p[i];
    p[i] = p[j];
    p[j] = temp;
}

void permutation(int32_t p[], size_t left, size_t right, PointVector* r_points) {
    size_t i;
    if (left == right) {
        int32_t rotations[8][3] = {{1,1,1},{-1,1,1},{1,-1,1},{1,1,-1},{-1,-1,1},{1,-1,-1},{-1,1,-1},{-1,-1,-1}};
        for (i=0; i < 8; i++) {
            point_t *pt = (point_t*)malloc(sizeof(point_t));
            pt->x = p[0] * rotations[i][0];
            pt->y = p[1] * rotations[i][1];
            pt->z = p[2] * rotations[i][2];
            push_pv(r_points, pt);
        }
    } else {
        for (i = left; i < right; i++) {
            swap(p, left, i);
            permutation(p, left + 1, right, r_points);
            swap(p, left, i);
        }
    }
}

void scan_beacons(PointVector* scanners) {
    size_t i, j, k, w, a,b;
    for (i=0; i< scanners->size-1; i++) {
        scan_t* scanner = scanners->array[i];
        scan_t* scanner1 = scanners->array[i+1];
        
        PointVector *points1 = init_ptr_vector(100);
        PointVector *points2 = init_ptr_vector(100);
        
        for (j=0; j < scanner->i; j++) {
            PointVector* r_points = init_ptr_vector(100);
            point_t p = scanner->points[j];
            int32_t perm[3] = {0};
            perm[0] = p.x;
            perm[1] = p.y;
            perm[2] = p.z;
            permutation(perm, 0, 3, r_points);
            push_pv(points1, r_points);
        }

        for (j=0; j < scanner1->i; j++) {
            PointVector* r_points = init_ptr_vector(100);
            point_t p = scanner1->points[j];
            int32_t perm[3] = {0};
            perm[0] = p.x;
            perm[1] = p.y;
            perm[2] = p.z;
            permutation(perm, 0, 3, r_points);
            push_pv(points2, r_points);
        }

        for (j=0; j < points1 -> size; j++) {
            scan_t scans0[25] = {0};
            scan_t scans[25] = {0};
            size_t idx = 0;
            
            for (k=0; k < points1 -> size; k++) {
                scan_t scan1 = {0};
                for (w=0; w < points1 -> size; w++) {
                    PointVector *current = points1->array[w];
                    point_t* pt = current->array[k];
                    scan1.points[scan1.i++] = *pt;
                }
                scans0[idx++] = scan1;
            }

            idx = 0;
            for (k=0; k < points2 -> size; k++) {
                scan_t scan2 = {0};
                for (w=0; w < points2 -> size; w++) {
                    PointVector *current = points2->array[w];
                    point_t* pt = current->array[k];
                    scan2.points[scan2.i++] = *pt;
                }
                scans[idx++] = scan2;
            }
            
            for (k=0; k < SIZE; k++) {
                scan_t scan1 = scans0[k];
                for (b=0; b < SIZE; b++) {
                    uint32_t matching = 0;
                    scan_t scan2 = scans[b];
                    // printf("%d\n", scan1.points->x);
                    // printf("%d\n", scan2.points->x);
                    for (w=0; w < SIZE; w++) {
                        int32_t x = scan1.points[w].x, y = scan1.points[w].y, z = scan1.points[w].z;   
                        for (a=0; a < SIZE; a++) {
                            int32_t x1 = scan2.points[a].x, y1 = scan2.points[a].y, z1 = scan2.points[a].z;
                            if (x == x1 && y == y1 && z == z1) {
                                printf("asdf\n");
                                matching++;
                                break;
                            }
                        }
                    }
                    printf("matching : %d\n", matching);
                }
                break;
            }
            break;
        }
            

            // for (w=0; w < SIZE; w++) {
            //             scan_t scan2 = scans[w];
            //             for (a=0; a < SIZE; a ++ ) {
            //                 int32_t x = scan1.points[k].x, y = scan1.points[k].y, z = scan1.points[k].z;
            //                 int32_t x1 = scan2.points[w].x, y1 = scan2.points[w].y, z1 = scan2.points[w].z;
            //                 if (x == x1 && y == y1 && z == z1) {
            //                     matching++;
            //                     break;
            //                 }
            //             }
            //         }
            //         printf("matching %d\n", matching);
            //         }
        // for (j=0; j < points1 -> size; j++) {
        //     int32_t matching = 0;
        //     for (k=0; k < points1 -> size; k++) {
        //         PointVector *current = points1->array[k];
        //         point_t* pt = current->array[j];
        //         // printf("%d %d %d\n", pt->x, pt->y, pt->z);
        //         for (a = 0; a < points2->size; a++) {
        //             for (b=0; b < points2->size; b++) {
        //                 PointVector *current1 = points2->array[b];
        //                 point_t* pt1 = current->array[a];
        //                 if (pt->x == pt1->x && pt->y == pt1->y && pt->z == pt1->z) {    
        //                     matching++;
        //                     printf("%d %d %d\n", pt->x, pt->y, pt->z);
        //                     printf("%d %d %d\n", pt1->x, pt1->y, pt1->z);

        //                     break;
        //                 }
        //             }
        //         }
        //         // printf("%d %d %d\n", pt->x, pt->y, pt->z);
        //         break;
        //     }
        //     printf("%d\n", matching);
        // }


        break;
    }
}

void solution(FILE* file) {
    PointVector* scanners = parse_scanners(file);
    scan_beacons(scanners);
    
    // printf("%d\n", scanners->size);
    // printf("%d\n", ((scan_t*)scanners->array[1])->points[0].x);
    // printf("%d\n", ((scan_t*)scanners->array[0])->points[5].x);
    
    
}

AOC_MAIN_ONE("./inputs/q19.txt")
