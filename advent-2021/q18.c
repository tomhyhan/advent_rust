#include "lib.h"

typedef struct {
    int32_t depth;
    int32_t value;
} num_t;

static PointVector* parse_snailfishes(FILE* file) {
    PointVector *lines = init_ptr_vector(1000);
    static char line[1000];
    while (fscanf_s(file, "%s\n", line, sizeof(line)) != EOF) {
        push_pv(lines, _strdup(line));
    }
    return lines;
}

bool explode(PointVector* snailfishes) {
    return FALSE;
}

bool split(PointVector* snailfishes) {
    return FALSE;
}

void add_snailfishes(PointVector* snailfishes, PointVector* lines) {
    size_t i;
    for (i=0; i < lines->size; i++) {
        size_t j, k;
        int32_t depth = 0;
        char* current_line;
        for (j=0; j < snailfishes->size; j++) {
            ((num_t*)snailfishes->array[j])->depth++;
        }
        current_line = (char*)lines->array[i]; 
        while (*current_line != '\0') {
            num_t* num;
            switch (*current_line) {
                case ',':
                case ']':
                case '[':
                    if (*current_line == '[') {
                        depth++;
                    } else if (*current_line == ']') {
                        depth--;
                    }
                    current_line++;
                    continue;
                default:
                    num = (num_t*)malloc(sizeof(num_t));
                    num->depth = depth;
                    num->value = *current_line - '0';
                    push_pv(snailfishes, num);
                    current_line++;
            }
        }
        while (explode(snailfishes) || split(snailfishes));
    }
    for (i=0; i < snailfishes->size; i++) {
        num_t* n = snailfishes->array[i];
        printf("%d %d\n", n->value, n->depth);
    }
    
}

void solution(FILE* file) {
    PointVector* snailfishes = init_ptr_vector(10000);
    PointVector* lines = parse_snailfishes(file);

    add_snailfishes(snailfishes, lines);

    free_ptr_vector(lines);
    free_ptr_vector(snailfishes);
}


AOC_MAIN_ONE("./inputs/q18.txt")
