#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lib.h"
#include "vector.h"

#define TRUE 1
#define FALSE 0
#define N 1000

int did_win(int mark[5][5]) {
    size_t row, col, is_bingo;
    int* line;

    for (row=0; row < 5; row++) {
        is_bingo = TRUE;
        line = mark[row];
        for (col=0; col < 5; col++) {
            if (line[col] == FALSE) {
                is_bingo = FALSE;
            }
        }
        if (is_bingo) return TRUE;
    }
    
    for (col=0; col < 5; col++) {
        is_bingo = TRUE;
        for (row=0; row < 5; row++) {
            line = mark[row];
            if (line[col] == FALSE) {
                is_bingo = FALSE;
            }
        }
        if (is_bingo) return TRUE;
    }

    return FALSE;
}

void mark_board(int current_draw, int board[5][5], int mark[5][5]) {
    size_t row, col;

    for (row=0; row < 5;row++) {
        for (col=0; col < 5;col++) {
            if (board[row][col] == current_draw) {
                mark[row][col] = TRUE; 
                return;
            }
        }
    }
}

int calc_unmarked(int board[5][5], int mark[5][5]) {
    size_t row, col;
    int total = 0;

    for (row=0; row < 5;row++) {
        for (col=0; col < 5;col++) {
            if (mark[row][col]) {
                continue;
            }
            total += board[row][col];
        }
    }

    return total;
}

int main(void) {
    FILE *file = read_file_data("./inputs/q4.txt");
    char line[N];
    const char delimiter[] = ",", s_delimiter[] = " ";
    int current_draw, winners = 0, unmarked,is_draws = TRUE;
    int boards[N][5][5], marks[N][5][5], wins[N];
    char *token = NULL, *nextToken = NULL;
    Vector *draws = init_vector(N);
    size_t i,j, row, col, board_num;

    board_num = -1;
    row = 0;
    while (fgets(line, sizeof(line), file) != NULL) {
        if (is_draws) {
            token = strtok_s(line, delimiter, &nextToken);
            while (token != NULL ) {
                push(draws, atoi(token));
                token = strtok_s(NULL, delimiter, &nextToken);
            }
            is_draws = FALSE;
        } else {
            if (strlen(line) > 1) {
                col = 0;
                token = strtok_s(line, s_delimiter, &nextToken);
                while (token != NULL) {
                    boards[board_num][row][col] = atoi(token);
                    marks[board_num][row][col] = FALSE;
                    token = strtok_s(NULL, s_delimiter, &nextToken);
                    col++;
                }
                row++;
            }  else {
                row = 0;
                board_num++;
                wins[board_num] = FALSE;
            }
        }
    }

    for (i=0; i < draws->size; i++) {
        current_draw = draws->array[i];
        for (j=0; j <= board_num; j++) {
            if (wins[j]) {
                continue;
            }
            mark_board(current_draw, boards[j], marks[j]);
            if (did_win(marks[j])) {
                wins[j] = TRUE;
                winners += 1;
                if (winners == (int)board_num + 1 ) {
                    printf("last board is %lld\n", j+1);
                    printf("draw %d\n", current_draw);
                    unmarked = calc_unmarked(boards[j], marks[j]);
                    printf("unmarked %d\n", unmarked);
                    printf("last board score is - %d\n",current_draw * unmarked);
                    goto finish;
                }
            }
        }
    }

    finish:
    free_vector(draws);
    fclose(file);
    return 1;
}
