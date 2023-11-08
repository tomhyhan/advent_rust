#include "lib.h"




void solution(FILE* file) {
    int32_t x1, x2, y1, y2 ,z1, z2;
    char status[4];
    char table[100][100][100] = {0};
    int32_t i,j,k, cnt = 0;
    while (fscanf_s(file, "%3s x=%d..%d,y=%d..%d,z=%d..%d\n",status, sizeof(status), &x1, &x2, &y1, &y2 ,&z1, &z2) != EOF) {
        x1 = x1 + 50; x2 = x2 + 50; y1 = y1 + 50; y2 = y2 + 50; z1 = z1 + 50; z2 = z2 + 50;
        printf("%s %d\n", status, x1);
        for (i=x1; i <= x2; i++) {
            for (j=y1; j <= y2; j++) {
                for (k=z1; k <= z2; k++) {
                    if (strcmp(status, "on") == 0) {
                        table[i][j][k] = 1;
                    } else {
                        table[i][j][k] = 0;
                    }
                }
            }
        }
    }

    for (i=0; i < 100; i++) {
        for (j=0; j < 100; j++) {
            for (k=0; k < 100; k++) {
                if (table[i][j][k] == 1) cnt++;
            }
        }
    }
    printf("%d", cnt);
}

AOC_MAIN_ONE("./inputs/q22.txt")
