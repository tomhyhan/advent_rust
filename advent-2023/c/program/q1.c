#define _CRT_SECURE_NO_WARNINGS

#include "call_lib.h"

bool starts_with(char line[], char new_num[]) {
    size_t i;

    for(i=0; new_num[i] != '\0'; i++) {
        if (*(line + i) != *(new_num + i)) {
            return FALSE;
        }
    }
    
    return TRUE;
}

void part1(char* input) {
    char new_nums[9][6] = {"one","two","three","four","five","six","seven","eight","nine"};

    char line[50];
    int result = 0;
    int result_p2 = 0;
    int read = 0, offset = 0;
    while (sscanf(input + offset, "%s%n", line, &read) != EOF) {
        size_t i, j ;
        char* list = create_list(char);
        char* list_p2 = create_list(char);
        for (i=0; line[i] != '\0'; i++) {
            if ('1' <= line[i] && line[i] <= '9') {
                push_list(list, line[i]);
                push_list(list_p2, line[i]);
            } else {
                for (j=0; j < 9; j++) {
                    if (starts_with(line + i, new_nums[j])) {
                        char num = j + 1 + '0';
                        push_list(list_p2, num);
                        break;
                    }
                }
            }
        }
        result += (list[0] - '0') * 10 + (list[list_len(list) - 1] - '0');
        result_p2 += (list_p2[0] - '0') * 10 + (list_p2[list_len(list_p2) - 1] - '0');
        destroy_list(list);
        destroy_list(list_p2);
        offset += read;
    }
    printf("%d\n", result);
    printf("%d\n", result_p2);
}

void part2(char* input) {
}

SOLUTION("./inputs/q1.txt")

