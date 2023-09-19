#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.h"
#include "vector.h"

#define TRUE 1
#define FALSE 0
#define N 1000

int main(void) {
    FILE *file = read_file_data("./inputs/q4.txt");
    char line[N];
    const char delimiter[] = ",", s_delimiter[] = " ";
    char ch;
    int is_draws = TRUE;
    int boards[100][5][5];
    char *token = NULL, *nextToken = NULL, *nt = NULL;
    Vector *draws = init_vector(N);
    size_t i, row, col, board_num;
    char input[] = "1,22,34,34,5";

    board_num = 0;
    row = 0;

    token = strtok_s(input, delimiter, &nextToken);
    while (token != NULL) {
        printf("Token: %d\n", atoi(token));
        token = strtok_s(NULL, delimiter, &nextToken);
    }

    // while (fgets(line, sizeof(line), file) != NULL) {
    //     if (is_draws) {
    //         token = strtok_s(line, delimiter, &nextToken);
    //         while (token != NULL ) {
    //             push(draws, atoi(token));
    //             token = strtok_s(NULL, delimiter, &nextToken);
    //         }
    //         is_draws = FALSE;
    //     } else {
    //         if (strlen(line) > 1) {
    //             col = 0;
    //             printf("%s", line);
    //             token = strtok_s(line, s_delimiter, &nt);
    //             while (token != NULL) {
    //                 // boards[board_num][row][col] = atoi(token);
    //                 printf("token: %d\n", atoi(token));
    //                 token = strtok_s(NULL, delimiter, &nt);
    //                 col ++;
    //             }
    //             printf("token: %s\n", token);
    //             row++;
    //         }  else {
    //             printf("");
    //             row = 0;
    //             board_num++;
    //         }
    //     }
    // }

    
    free_vector(draws);
    fclose(file);
    return 1;
}
