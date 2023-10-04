#include "lib.h"

int find_corrupted(char line[]) {
    size_t i ;
    PointVector *stack = init_ptr_vector(1000);
    // *(line+i) != '\0'

    for (i=0; *(line+i) != '\0'; i++) {
        char* current = line+i;

        if (strchr("([<{", *current) != NULL) {
            printf("pushed - %c\n", *current);
            push_pv(stack, current);
        } else {
            char *poped_char;
            poped_char = pop_pv(stack);
            printf("poped_char - %c \n", *poped_char);
            break;
        }
        
    }

    return 0;
}

void solution(FILE *file) {
    char line[1000];
    while (fscanf_s(file, "%s",line, sizeof(line)) != EOF) {
        printf("%s\n", line);
        find_corrupted(line);
        break;
    };
}

AOC_MAIN_ONE("./inputs/q10.txt")

